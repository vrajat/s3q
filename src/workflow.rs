use std::time::Duration;

use crate::Client;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowIdReusePolicy {
    RejectDuplicate,
    AllowDuplicateAfterCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowExecution {
    pub workflow_name: String,
    pub workflow_id: String,
    pub run_id: String,
    pub status: WorkflowStatus,
}

#[derive(Debug, Clone, Copy)]
pub struct WorkflowApi<'a> {
    client: &'a Client,
}

impl<'a> WorkflowApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn workflow(&self, name: impl Into<String>) -> WorkflowHandle<'a> {
        WorkflowHandle {
            client: self.client,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowHandle<'a> {
    client: &'a Client,
    pub name: String,
}

impl<'a> WorkflowHandle<'a> {
    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn start(
        &self,
        workflow_id: impl Into<String>,
        input: impl Into<Vec<u8>>,
    ) -> StartWorkflowRequest {
        StartWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
            input: input.into(),
            task_queue: None,
            id_reuse_policy: WorkflowIdReusePolicy::RejectDuplicate,
        }
    }

    pub fn describe(&self, workflow_id: impl Into<String>) -> DescribeWorkflowRequest {
        DescribeWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
        }
    }

    pub fn list(&self) -> ListWorkflowsRequest {
        ListWorkflowsRequest {
            workflow_name: Some(self.name.clone()),
            limit: 100,
        }
    }

    pub fn signal(
        &self,
        workflow_id: impl Into<String>,
        signal_name: impl Into<String>,
        payload: impl Into<Vec<u8>>,
    ) -> SignalWorkflowRequest {
        SignalWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
            signal_name: signal_name.into(),
            payload: payload.into(),
        }
    }

    pub fn query(
        &self,
        workflow_id: impl Into<String>,
        query_name: impl Into<String>,
        args: impl Into<Vec<u8>>,
    ) -> QueryWorkflowRequest {
        QueryWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
            query_name: query_name.into(),
            args: args.into(),
        }
    }

    pub fn result(&self, workflow_id: impl Into<String>) -> ResultWorkflowRequest {
        ResultWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
            follow: false,
        }
    }

    pub fn cancel(&self, workflow_id: impl Into<String>) -> CancelWorkflowRequest {
        CancelWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
        }
    }

    pub fn terminate(
        &self,
        workflow_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> TerminateWorkflowRequest {
        TerminateWorkflowRequest {
            workflow_name: self.name.clone(),
            workflow_id: workflow_id.into(),
            reason: reason.into(),
        }
    }

    pub fn start_child(
        &self,
        parent_workflow_id: impl Into<String>,
        child_workflow_name: impl Into<String>,
        child_workflow_id: impl Into<String>,
        input: impl Into<Vec<u8>>,
    ) -> ChildWorkflowRequest {
        ChildWorkflowRequest {
            parent_workflow_name: self.name.clone(),
            parent_workflow_id: parent_workflow_id.into(),
            child_workflow_name: child_workflow_name.into(),
            child_workflow_id: child_workflow_id.into(),
            input: input.into(),
        }
    }

    pub fn sleep(&self, duration: Duration) -> TimerSpec {
        TimerSpec { duration }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
    pub input: Vec<u8>,
    pub task_queue: Option<String>,
    pub id_reuse_policy: WorkflowIdReusePolicy,
}

impl StartWorkflowRequest {
    pub fn with_task_queue(mut self, task_queue: impl Into<String>) -> Self {
        self.task_queue = Some(task_queue.into());
        self
    }

    pub fn with_id_reuse_policy(mut self, policy: WorkflowIdReusePolicy) -> Self {
        self.id_reuse_policy = policy;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescribeWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListWorkflowsRequest {
    pub workflow_name: Option<String>,
    pub limit: usize,
}

impl ListWorkflowsRequest {
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
    pub signal_name: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
    pub query_name: String,
    pub args: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
    pub follow: bool,
}

impl ResultWorkflowRequest {
    pub fn follow(mut self) -> Self {
        self.follow = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminateWorkflowRequest {
    pub workflow_name: String,
    pub workflow_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildWorkflowRequest {
    pub parent_workflow_name: String,
    pub parent_workflow_id: String,
    pub child_workflow_name: String,
    pub child_workflow_id: String,
    pub input: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerSpec {
    pub duration: Duration,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{connect, WorkflowIdReusePolicy, WorkflowStatus};

    #[test]
    fn workflow_start_request_defaults_are_explicit() {
        let client = connect("s3://bucket/workflows.db");
        let workflow = client.workflows().workflow("invoice");

        let request = workflow.start("invoice-1", br#"{"customer":"acme"}"#.to_vec());

        assert_eq!(request.workflow_name, "invoice");
        assert_eq!(
            request.id_reuse_policy,
            WorkflowIdReusePolicy::RejectDuplicate
        );
    }

    #[test]
    fn timer_spec_keeps_duration() {
        let client = connect("s3://bucket/workflows.db");
        let timer = client
            .workflows()
            .workflow("invoice")
            .sleep(Duration::from_secs(5));

        assert_eq!(timer.duration, Duration::from_secs(5));
    }

    #[test]
    fn workflow_status_is_copyable() {
        let status = WorkflowStatus::Running;
        assert_eq!(status, WorkflowStatus::Running);
    }
}
