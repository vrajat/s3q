"""Python exceptions for the thin s3q SDK."""


class S3QError(RuntimeError):
    """Base class for Python s3q exceptions."""


class NotReadyError(S3QError):
    """Raised when a Python API surface exists but is not wired to Rust yet."""


class InvalidArgumentError(S3QError):
    """Raised when an argument cannot be represented by the Rust API."""


class QueueNotFoundError(S3QError):
    """Raised when a queue cannot be found."""


class OwnershipMismatchError(S3QError):
    """Raised when a consumer tries to complete a lease it does not own."""


class MessageNotFoundError(S3QError):
    """Raised when inspection cannot find a message by id."""


def translate_native_error(error: BaseException) -> S3QError:
    """Map native binding exceptions into Python s3q exceptions."""

    message = str(error)
    if isinstance(error, PermissionError):
        return OwnershipMismatchError(message)
    if isinstance(error, ValueError):
        return InvalidArgumentError(message)
    if isinstance(error, LookupError):
        if message.startswith("message not found:"):
            return MessageNotFoundError(message.removeprefix("message not found:"))
        if message.startswith("queue not found:"):
            return QueueNotFoundError(message.removeprefix("queue not found:"))
    return S3QError(message)
