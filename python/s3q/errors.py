class NotReadyError(RuntimeError):
    """Raised when a scaffolded API surface has not been wired to Rust yet."""
