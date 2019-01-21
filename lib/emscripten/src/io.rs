use libc::printf as _printf;

use wasmer_runtime::{vm::Ctx, types::{MemoryIndex}, structures::{TypedIndex}};
/// putchar
pub use libc::putchar;

/// printf
pub extern "C" fn printf(memory_offset: i32, extra: i32, vmctx: &mut Ctx) -> i32 {
    debug!("emscripten::printf {}, {}", memory_offset, extra);
    unsafe {
        let addr = vmctx.memory(MemoryIndex::new(0))[memory_offset as usize] as _;
        _printf(addr, extra)
    }
}
