use std::io;

/// Global error handler
static mut ERROR_HANDLER: &dyn ErrorHandler = &DefaultErrorHandler;

/// Set a custom error handler
///
/// # Safety
///
/// This function is unsafe because it modifies a global static variable.
/// It should only be called once, at the start of the program, before any
/// other rpian-terminal functions are used.
pub unsafe fn set_error_handler(handler: &'static dyn ErrorHandler) {
    ERROR_HANDLER = handler;
}

// Internal function to handle I/O errors
pub fn handle_io_error(error: io::Error) {
    unsafe {
        ERROR_HANDLER.handle_io_error(error);
    }
}

// Internal function to handle boundary errors
pub fn handle_boundary_error(message: &str) {
    unsafe {
        ERROR_HANDLER.handle_boundary_error(message);
    }
}

/// Trait for custom error handling in rpian-terminal
pub trait ErrorHandler {
    /// Handle an I/O error
    fn handle_io_error(&self, error: io::Error);

    /// Handle a boundary error (e.g., cursor or drawing outside viewport)
    fn handle_boundary_error(&self, message: &str);
}

/// Default error handler that logs errors to stderr
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_io_error(&self, error: io::Error) {
        eprintln!("rpian-terminal I/O Error: {}", error);
    }

    fn handle_boundary_error(&self, message: &str) {
        eprintln!("rpian-terminal Boundary Error: {}", message);
    }
}