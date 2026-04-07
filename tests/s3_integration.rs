use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pgqrs::store::s3::S3Store;
use pgqrs::store::Store as _;
use serde_json::json;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn prepare_localstack_tls_env() {
    let endpoint = std::env::var("AWS_ENDPOINT_URL")
        .expect("AWS_ENDPOINT_URL must be set for LocalStack-backed s3q integration tests");
    if endpoint.starts_with("http://") {
        std::env::remove_var("SSL_CERT_FILE");
        std::env::remove_var("SSL_CERT_DIR");
        std::env::remove_var("AWS_CA_BUNDLE");
    }
}

fn unique_name(prefix: &str) -> String {
    let seq = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("{prefix}-{}-{ts}-{seq}", std::process::id())
}

fn unique_dsn(prefix: &str) -> String {
    let bucket = std::env::var("PGQRS_S3_BUCKET").unwrap_or_else(|_| "s3q-test-bucket".to_string());
    format!("s3://{bucket}/{}.sqlite", unique_name(prefix))
}

async fn oracle_store(dsn: &str) -> S3Store {
    let config =
        pgqrs::Config::from_dsn_with_schema(dsn, "s3q_test").expect("oracle config should build");
    S3Store::new(&config)
        .await
        .expect("oracle S3 store should connect")
}

#[tokio::test]
#[ignore = "requires LocalStack; run with `make test-localstack`"]
async fn queue_lifecycle_uses_s3_store() {
    prepare_localstack_tls_env();
    let client = s3q::Client::connect_with_config(
        s3q::ClientConfig::new(unique_dsn("s3q-lifecycle")).with_namespace("s3q_test"),
    )
    .await
    .expect("client should connect");

    let queue_name = unique_name("lifecycle");
    let queue = client
        .create_queue(&queue_name)
        .await
        .expect("queue should be created");
    assert_eq!(queue.name(), queue_name);

    client
        .purge_queue(&queue_name)
        .await
        .expect("empty queue purge should succeed");
    client
        .delete_queue(&queue_name)
        .await
        .expect("empty queue delete should succeed");
}

#[tokio::test]
#[ignore = "requires LocalStack; run with `make test-localstack`"]
async fn producers_consumers_and_completion_work_end_to_end() {
    prepare_localstack_tls_env();
    let dsn = unique_dsn("s3q-flow");
    let client =
        s3q::Client::connect_with_config(s3q::ClientConfig::new(&dsn).with_namespace("s3q_test"))
            .await
            .expect("client should connect");

    let queue_name = unique_name("flow");
    let queue = client
        .create_queue(&queue_name)
        .await
        .expect("queue should be created");
    let producer = queue
        .producer("api-worker")
        .await
        .expect("producer should be created");
    let consumer_a = queue
        .consumer("worker-a")
        .await
        .expect("consumer A should be created");
    let consumer_b = queue
        .consumer("worker-b")
        .await
        .expect("consumer B should be created");

    assert_eq!(producer.queue_name(), queue_name);
    assert_eq!(producer.worker_id(), "api-worker");
    assert_eq!(consumer_a.queue_name(), queue_name);
    assert_eq!(consumer_a.worker_id(), "worker-a");

    let sent = producer
        .send(json!({ "kind": "single" }))
        .await
        .expect("single send should succeed");
    assert!(sent.receipt_handle.is_none());

    let batch = producer
        .send_batch(vec![
            json!({ "kind": "batch", "n": 1 }),
            json!({ "kind": "batch", "n": 2 }),
        ])
        .await
        .expect("batch send should succeed");
    assert_eq!(batch.len(), 2);

    let mut messages = consumer_a
        .read_batch(Duration::from_secs(30), 3)
        .await
        .expect("read_batch should succeed");
    assert_eq!(messages.len(), 3);
    assert!(messages
        .iter()
        .all(|message| message.receipt_handle.is_some()));
    assert!(messages
        .iter()
        .all(|message| message.state == s3q::MessageState::Leased));

    let wrong_owner = messages[0]
        .receipt_handle
        .clone()
        .expect("leased message should carry receipt handle");
    assert_eq!(
        consumer_b.delete_message(wrong_owner.clone()).await,
        Err(s3q::Error::OwnershipMismatch)
    );
    assert_eq!(
        consumer_b.archive_message(wrong_owner.clone()).await,
        Err(s3q::Error::OwnershipMismatch)
    );
    assert_eq!(
        consumer_b.set_vt(wrong_owner, Duration::from_secs(5)).await,
        Err(s3q::Error::OwnershipMismatch)
    );

    let delete_handle = messages
        .pop()
        .and_then(|message| message.receipt_handle)
        .expect("message should carry delete receipt handle");
    assert!(
        consumer_a
            .delete_message(delete_handle)
            .await
            .expect("delete should succeed"),
        "delete should report a completed message"
    );

    let archive_handle = messages
        .pop()
        .and_then(|message| message.receipt_handle)
        .expect("message should carry archive receipt handle");
    let archived = consumer_a
        .archive_message(archive_handle)
        .await
        .expect("archive should succeed")
        .expect("archive should return the archived message");
    assert_eq!(archived.state, s3q::MessageState::Archived);

    let set_vt_handle = messages
        .pop()
        .and_then(|message| message.receipt_handle)
        .expect("message should carry set_vt receipt handle");
    assert!(consumer_a
        .set_vt(set_vt_handle.clone(), Duration::from_secs(60))
        .await
        .expect("set_vt should succeed"));
    assert!(consumer_a
        .archive_messages(vec![set_vt_handle])
        .await
        .expect("batch archive should succeed")
        .into_iter()
        .all(|ok| ok));

    let oracle = oracle_store(&dsn).await;
    let queue_record = oracle
        .queues()
        .get_by_name(&queue_name)
        .await
        .expect("queue should exist in backing store");
    let workers = oracle
        .workers()
        .filter_by_fk(queue_record.id)
        .await
        .expect("worker records should be queryable");
    let worker_names = workers
        .iter()
        .map(|worker| worker.name.as_str())
        .collect::<Vec<_>>();
    assert!(worker_names.contains(&"api-worker"));
    assert!(worker_names.contains(&"worker-a"));
    assert!(worker_names.contains(&"worker-b"));

    let archived_messages = oracle
        .messages()
        .list_archived_by_queue(queue_record.id)
        .await
        .expect("archived messages should be retained");
    assert_eq!(archived_messages.len(), 2);
}

#[tokio::test]
#[ignore = "requires LocalStack; run with `make test-localstack`"]
async fn delayed_send_is_not_visible_until_delay_expires() {
    prepare_localstack_tls_env();
    let client = s3q::Client::connect_with_config(
        s3q::ClientConfig::new(unique_dsn("s3q-delayed")).with_namespace("s3q_test"),
    )
    .await
    .expect("client should connect");

    let queue = client
        .create_queue(unique_name("delayed"))
        .await
        .expect("queue should be created");
    let producer = queue
        .producer("api-worker")
        .await
        .expect("producer should be created");
    let consumer = queue
        .consumer("worker-a")
        .await
        .expect("consumer should be created");

    producer
        .send_delayed(json!({ "kind": "delayed" }), Duration::from_secs(60))
        .await
        .expect("delayed send should succeed");

    let messages = consumer
        .read_batch(Duration::from_secs(30), 1)
        .await
        .expect("read should succeed");
    assert!(messages.is_empty(), "delayed message should not be visible");
}
