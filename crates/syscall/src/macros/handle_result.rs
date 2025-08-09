macro_rules! handle_result {
    ($($body:tt)*) => {
        fn handle_result(result: arch::Result) -> crate::Result {
            match result {
                Ok(signed_result) => Ok(signed_result),
                Err(err) => {
                    let errno : isize = err.into();

                    human::info!("\nRaw syscall error: {} = {}\n",
                                SYSCALL_NAME, errno);

                    $($body)*

                    Err(crate::Error::$result_variant(matched_error))
                }
            }
        }
    };
}
