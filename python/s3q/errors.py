"""Python exceptions for the thin s3q SDK."""


class S3QError(RuntimeError):
    """Base class for Python s3q exceptions."""


class NotReadyError(S3QError):
    """Raised when a Python API surface exists but is not wired to Rust yet."""


class InvalidArgumentError(S3QError):
    """Raised when an argument cannot be represented by the Rust API."""


class OwnershipMismatchError(S3QError):
    """Raised when a consumer tries to complete a lease it does not own."""


class MessageNotFoundError(S3QError):
    """Raised when inspection cannot find a message by id."""
