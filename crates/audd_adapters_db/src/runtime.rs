//! Runtime bridging utilities
//!
//! Provides a safe `block_on` helper that works correctly regardless of whether
//! the calling thread is already inside a Tokio runtime.
//!
//! - **Inside a multi-thread runtime**: uses `block_in_place` so the scheduler
//!   can move other tasks to a different worker thread while this thread blocks.
//! - **Outside any runtime**: creates a temporary runtime, runs the future, and
//!   disposes of the runtime without waiting for background tasks.

/// Execute a future from a synchronous context, safe to call from within an
/// existing Tokio multi-thread runtime.
///
/// # Panics
///
/// Panics if called from inside a `current_thread` (single-threaded) Tokio
/// runtime, because `block_in_place` requires multiple worker threads.
/// In that scenario, call this function from a dedicated `std::thread` instead.
pub(crate) fn block_on<F: std::future::Future>(future: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(future)),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .expect("Failed to create Tokio runtime");
            let result = rt.block_on(future);
            // Dispose without blocking on long-running background tasks
            // (e.g. the tokio-postgres connection driver task).
            rt.shutdown_background();
            result
        }
    }
}
