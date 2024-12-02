mod random;
mod wasm_random;

pub use self::random::Random;
pub use self::wasm_random::WasmRandom;

#[cfg(not(target_arch = "wasm32"))]
mod os_random;
#[cfg(not(target_arch = "wasm32"))]
pub use self::os_random::OsRandom;