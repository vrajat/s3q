use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pyo3::exceptions::{PyLookupError, PyPermissionError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use tokio::runtime::Runtime;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("s3q Python bindings require a Tokio runtime"))
}

fn map_error(error: s3q_rs::Error) -> PyErr {
    match error {
        s3q_rs::Error::InvalidArgument(message) => PyValueError::new_err(message),
        s3q_rs::Error::OwnershipMismatch => {
            PyPermissionError::new_err("message is owned by another consumer")
        }
        s3q_rs::Error::MessageNotFound(message_id) => {
            PyLookupError::new_err(format!("message not found:{message_id}"))
        }
        s3q_rs::Error::QueueNotFound(queue_name) => {
            PyLookupError::new_err(format!("queue not found:{queue_name}"))
        }
        other => PyRuntimeError::new_err(other.to_string()),
    }
}

fn parse_json(payload_json: &str) -> PyResult<Value> {
    serde_json::from_str(payload_json)
        .map_err(|err| PyValueError::new_err(format!("payload must be JSON-serializable: {err}")))
}

fn system_time_seconds(time: SystemTime) -> f64 {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0)
}

fn state_name(state: s3q_rs::MessageState) -> &'static str {
    match state {
        s3q_rs::MessageState::Visible => "visible",
        s3q_rs::MessageState::Leased => "leased",
        s3q_rs::MessageState::Delayed => "delayed",
        s3q_rs::MessageState::Archived => "archived",
    }
}

fn message_to_dict(py: Python<'_>, message: s3q_rs::Message) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    dict.set_item("message_id", message.message_id)?;
    dict.set_item("read_count", message.read_count)?;
    dict.set_item("enqueued_at", system_time_seconds(message.enqueued_at))?;
    dict.set_item("visible_at", system_time_seconds(message.visible_at))?;
    dict.set_item(
        "payload_json",
        serde_json::to_string(&message.payload).unwrap(),
    )?;
    dict.set_item(
        "receipt_handle",
        message.receipt_handle.map(|receipt| receipt.encode()),
    )?;
    dict.set_item("state", state_name(message.state))?;
    Ok(dict.into_py(py))
}

fn queue_summary_to_dict(py: Python<'_>, queue: s3q_rs::QueueSummary) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    dict.set_item("queue_name", queue.queue_name)?;
    dict.set_item("created_at", system_time_seconds(queue.created_at))?;
    Ok(dict.into_py(py))
}

fn metrics_to_dict(py: Python<'_>, metrics: s3q_rs::QueueMetrics) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    dict.set_item("queue_name", metrics.queue_name)?;
    dict.set_item("visible_messages", metrics.visible_messages)?;
    dict.set_item("leased_messages", metrics.leased_messages)?;
    dict.set_item("delayed_messages", metrics.delayed_messages)?;
    dict.set_item("archived_messages", metrics.archived_messages)?;
    dict.set_item("total_messages", metrics.total_messages)?;
    Ok(dict.into_py(py))
}

fn message_page_to_dict(py: Python<'_>, page: s3q_rs::MessagePage) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    let messages = PyList::empty_bound(py);
    for message in page.messages {
        messages.append(message_to_dict(py, message)?)?;
    }
    dict.set_item("messages", messages)?;
    dict.set_item("next_cursor", page.next_cursor)?;
    Ok(dict.into_py(py))
}

#[pyclass(name = "ClientCore")]
#[derive(Clone)]
struct ClientCore {
    inner: s3q_rs::Client,
}

#[pymethods]
impl ClientCore {
    #[new]
    #[pyo3(signature = (dsn, namespace="default", service_name="s3q", local_cache_dir=None))]
    fn new(
        dsn: String,
        namespace: &str,
        service_name: &str,
        local_cache_dir: Option<String>,
    ) -> PyResult<Self> {
        let mut config = s3q_rs::ClientConfig::new(dsn)
            .with_namespace(namespace)
            .with_service_name(service_name);
        if let Some(path) = local_cache_dir {
            config = config.with_local_cache_dir(path);
        }

        let inner = runtime()
            .block_on(s3q_rs::Client::connect_with_config(config))
            .map_err(map_error)?;
        Ok(Self { inner })
    }

    fn create_queue(&self, name: &str) -> PyResult<()> {
        runtime()
            .block_on(self.inner.create_queue(name))
            .map(|_| ())
            .map_err(map_error)
    }

    fn delete_queue(&self, name: &str) -> PyResult<()> {
        runtime()
            .block_on(self.inner.delete_queue(name))
            .map_err(map_error)
    }

    fn purge_queue(&self, name: &str) -> PyResult<()> {
        runtime()
            .block_on(self.inner.purge_queue(name))
            .map_err(map_error)
    }

    fn producer(&self, queue_name: &str, worker_id: &str) -> PyResult<ProducerCore> {
        let queue = self.inner.queue(queue_name);
        let inner = runtime()
            .block_on(queue.producer(worker_id))
            .map_err(map_error)?;
        Ok(ProducerCore { inner })
    }

    fn consumer(&self, queue_name: &str, worker_id: &str) -> PyResult<ConsumerCore> {
        let queue = self.inner.queue(queue_name);
        let inner = runtime()
            .block_on(queue.consumer(worker_id))
            .map_err(map_error)?;
        Ok(ConsumerCore { inner })
    }

    fn list_queues(&self, py: Python<'_>) -> PyResult<PyObject> {
        let queues = runtime()
            .block_on(self.inner.inspect().list_queues())
            .map_err(map_error)?;
        let items = PyList::empty_bound(py);
        for queue in queues {
            items.append(queue_summary_to_dict(py, queue)?)?;
        }
        Ok(items.into_py(py))
    }

    fn metrics(&self, py: Python<'_>, queue_name: &str) -> PyResult<PyObject> {
        let metrics = runtime()
            .block_on(self.inner.inspect().metrics(queue_name))
            .map_err(map_error)?;
        metrics_to_dict(py, metrics)
    }

    fn metrics_all(&self, py: Python<'_>) -> PyResult<PyObject> {
        let metrics = runtime()
            .block_on(self.inner.inspect().metrics_all())
            .map_err(map_error)?;
        let items = PyList::empty_bound(py);
        for metric in metrics {
            items.append(metrics_to_dict(py, metric)?)?;
        }
        Ok(items.into_py(py))
    }

    fn get_message(&self, py: Python<'_>, queue_name: &str, message_id: i64) -> PyResult<PyObject> {
        let message = runtime()
            .block_on(self.inner.inspect().get_message(queue_name, message_id))
            .map_err(map_error)?;
        message_to_dict(py, message)
    }

    #[pyo3(signature = (queue_name, state=None, limit=None, cursor=None))]
    fn list_messages(
        &self,
        py: Python<'_>,
        queue_name: &str,
        state: Option<&str>,
        limit: Option<usize>,
        cursor: Option<String>,
    ) -> PyResult<PyObject> {
        let inspect = self.inner.inspect();
        let mut request = inspect.list_messages(queue_name.to_string());
        if let Some(state) = state {
            request = request.with_state(parse_state(state)?);
        }
        if let Some(limit) = limit {
            request = request.with_limit(limit);
        }
        if let Some(cursor) = cursor {
            request = request.with_cursor(cursor);
        }
        let page = runtime().block_on(request.execute()).map_err(map_error)?;
        message_page_to_dict(py, page)
    }

    #[pyo3(signature = (queue_name, limit=None, cursor=None))]
    fn list_archived_messages(
        &self,
        py: Python<'_>,
        queue_name: &str,
        limit: Option<usize>,
        cursor: Option<String>,
    ) -> PyResult<PyObject> {
        let inspect = self.inner.inspect();
        let mut request = inspect.list_archived_messages(queue_name.to_string());
        if let Some(limit) = limit {
            request = request.with_limit(limit);
        }
        if let Some(cursor) = cursor {
            request = request.with_cursor(cursor);
        }
        let page = runtime().block_on(request.execute()).map_err(map_error)?;
        message_page_to_dict(py, page)
    }
}

#[pyclass(name = "ProducerCore")]
#[derive(Clone)]
struct ProducerCore {
    inner: s3q_rs::Producer,
}

#[pymethods]
impl ProducerCore {
    #[getter]
    fn queue_name(&self) -> &str {
        self.inner.queue_name()
    }

    #[getter]
    fn namespace(&self) -> &str {
        self.inner.namespace()
    }

    #[getter]
    fn worker_id(&self) -> &str {
        self.inner.worker_id()
    }

    #[pyo3(signature = (payload_json, delay_seconds=None))]
    fn send(
        &self,
        py: Python<'_>,
        payload_json: &str,
        delay_seconds: Option<u64>,
    ) -> PyResult<PyObject> {
        let payload = parse_json(payload_json)?;
        let message = match delay_seconds {
            Some(delay) => runtime()
                .block_on(self.inner.send_delayed(payload, Duration::from_secs(delay)))
                .map_err(map_error)?,
            None => runtime()
                .block_on(self.inner.send(payload))
                .map_err(map_error)?,
        };
        message_to_dict(py, message)
    }

    #[pyo3(signature = (payloads_json, delay_seconds=None))]
    fn send_batch(
        &self,
        py: Python<'_>,
        payloads_json: Vec<String>,
        delay_seconds: Option<u64>,
    ) -> PyResult<PyObject> {
        let payloads = payloads_json
            .into_iter()
            .map(|payload| parse_json(&payload))
            .collect::<PyResult<Vec<_>>>()?;
        let messages = match delay_seconds {
            Some(delay) => runtime()
                .block_on(
                    self.inner
                        .send_batch_delayed(payloads, Duration::from_secs(delay)),
                )
                .map_err(map_error)?,
            None => runtime()
                .block_on(self.inner.send_batch(payloads))
                .map_err(map_error)?,
        };
        let items = PyList::empty_bound(py);
        for message in messages {
            items.append(message_to_dict(py, message)?)?;
        }
        Ok(items.into_py(py))
    }
}

#[pyclass(name = "ConsumerCore")]
#[derive(Clone)]
struct ConsumerCore {
    inner: s3q_rs::Consumer,
}

#[pymethods]
impl ConsumerCore {
    #[getter]
    fn queue_name(&self) -> &str {
        self.inner.queue_name()
    }

    #[getter]
    fn namespace(&self) -> &str {
        self.inner.namespace()
    }

    #[getter]
    fn worker_id(&self) -> &str {
        self.inner.worker_id()
    }

    #[pyo3(signature = (vt_seconds=None))]
    fn read(&self, py: Python<'_>, vt_seconds: Option<u64>) -> PyResult<Option<PyObject>> {
        let vt = Duration::from_secs(vt_seconds.unwrap_or(30));
        let message = runtime().block_on(self.inner.read(vt)).map_err(map_error)?;
        message
            .map(|message| message_to_dict(py, message))
            .transpose()
    }

    #[pyo3(signature = (vt_seconds=None, qty=1))]
    fn read_batch(
        &self,
        py: Python<'_>,
        vt_seconds: Option<u64>,
        qty: usize,
    ) -> PyResult<PyObject> {
        let vt = Duration::from_secs(vt_seconds.unwrap_or(30));
        let messages = runtime()
            .block_on(self.inner.read_batch(vt, qty))
            .map_err(map_error)?;
        let items = PyList::empty_bound(py);
        for message in messages {
            items.append(message_to_dict(py, message)?)?;
        }
        Ok(items.into_py(py))
    }

    #[pyo3(signature = (vt_seconds=None, qty=1, poll_timeout_seconds=None, poll_interval_seconds=None))]
    fn read_with_poll(
        &self,
        py: Python<'_>,
        vt_seconds: Option<u64>,
        qty: usize,
        poll_timeout_seconds: Option<f64>,
        poll_interval_seconds: Option<f64>,
    ) -> PyResult<PyObject> {
        let vt = Duration::from_secs(vt_seconds.unwrap_or(30));
        let poll_timeout = Duration::from_secs_f64(poll_timeout_seconds.unwrap_or(20.0).max(0.0));
        let poll_interval = Duration::from_secs_f64(poll_interval_seconds.unwrap_or(0.25).max(0.0));
        let messages = runtime()
            .block_on(
                self.inner
                    .read_with_poll(vt, qty, poll_timeout, poll_interval),
            )
            .map_err(map_error)?;
        let items = PyList::empty_bound(py);
        for message in messages {
            items.append(message_to_dict(py, message)?)?;
        }
        Ok(items.into_py(py))
    }

    fn delete_message(&self, receipt_handle: &str) -> PyResult<bool> {
        let receipt = s3q_rs::ReceiptHandle::parse(receipt_handle).map_err(map_error)?;
        runtime()
            .block_on(self.inner.delete_message(receipt))
            .map_err(map_error)
    }

    fn archive_message(&self, py: Python<'_>, receipt_handle: &str) -> PyResult<Option<PyObject>> {
        let receipt = s3q_rs::ReceiptHandle::parse(receipt_handle).map_err(map_error)?;
        let message = runtime()
            .block_on(self.inner.archive_message(receipt))
            .map_err(map_error)?;
        message
            .map(|message| message_to_dict(py, message))
            .transpose()
    }

    fn archive_messages(&self, receipt_handles: Vec<String>) -> PyResult<Vec<bool>> {
        let receipts = receipt_handles
            .into_iter()
            .map(|receipt| s3q_rs::ReceiptHandle::parse(receipt).map_err(map_error))
            .collect::<PyResult<Vec<_>>>()?;
        runtime()
            .block_on(self.inner.archive_messages(receipts))
            .map_err(map_error)
    }

    fn set_vt(&self, receipt_handle: &str, vt_seconds: u64) -> PyResult<bool> {
        let receipt = s3q_rs::ReceiptHandle::parse(receipt_handle).map_err(map_error)?;
        runtime()
            .block_on(self.inner.set_vt(receipt, Duration::from_secs(vt_seconds)))
            .map_err(map_error)
    }
}

fn parse_state(value: &str) -> PyResult<s3q_rs::MessageState> {
    match value {
        "visible" => Ok(s3q_rs::MessageState::Visible),
        "leased" => Ok(s3q_rs::MessageState::Leased),
        "delayed" => Ok(s3q_rs::MessageState::Delayed),
        "archived" => Ok(s3q_rs::MessageState::Archived),
        other => Err(PyValueError::new_err(format!(
            "unknown message state: {other}"
        ))),
    }
}

#[pymodule]
fn _native(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<ClientCore>()?;
    module.add_class::<ProducerCore>()?;
    module.add_class::<ConsumerCore>()?;
    Ok(())
}
