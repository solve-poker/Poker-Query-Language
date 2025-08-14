#[cfg(target_arch = "wasm32")]
mod run_wasm;

#[cfg(target_arch = "wasm32")]
pub use run_wasm::parallel_exec;

#[cfg(not(target_arch = "wasm32"))]
mod run_native;

#[cfg(not(target_arch = "wasm32"))]
pub use run_native::parallel_exec;
