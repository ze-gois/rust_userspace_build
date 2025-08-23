use crate::arch;
use crate::arch::Result;

crate::macros::r#struct!(pub WriteOk {
    syscall : Result
});

pub fn handle_result(arch_result: crate::arch::Result) -> crate::syscall::Result {
    match arch_result {
        Ok(e) => Ok(crate::syscall::Ok::UnoticedWrite(e.discriminant())),
        Err(e) => Err(crate::syscall::Error::UnnoticedWrite(e.discriminant())),
    }
    // arch::Result::Ok(arch::Ok::UnoticedSys3X86_64(syscall_return))
}

pub fn write(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_count: usize,
) -> crate::syscall::Result {
    let returned_value = arch::syscall3(
        1usize,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(returned_value)
}
