#[cfg(target_os = "linux")]
mod speech_dispatcher;

#[cfg(windows)]
mod tolk;

#[cfg(windows)]
pub(crate) mod winrt;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_os = "macos")]
mod appkit;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod av_foundation;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod thread_wrapper;

#[cfg(target_os = "linux")]
pub use self::speech_dispatcher::*;

#[cfg(windows)]
pub use self::tolk::*;

#[cfg(target_arch = "wasm32")]
pub use self::web::*;

#[cfg(target_os = "macos")]
pub use self::appkit::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::av_foundation::*;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::thread_wrapper::*;
