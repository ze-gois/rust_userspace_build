use crate::arch;

macros::r#struct!(pub WriteOk {
    syscall : arch::Ok
});

pub fn handle_result(arch_result: crate::arch::Result) -> crate::syscall::Result {
    match arch_result {
        Ok(e) => Ok(crate::syscall::Ok::UnoticedWrite(e.discriminant())),
        Err(_e) => Err(crate::syscall::Error::UnoticedWriteErr(32)),
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
