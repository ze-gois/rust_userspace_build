macro_rules! syscalls_publishing {
    (
        $(
            [
                $syscall_number:expr;
                $syscall_label:expr;
                $syscall_ident:ident;
                $($syscall_path:tt)::+;
                $($syscall_signature:tt)::+
            ]
        ),
        *) => {
        $(pub mod $($syscall_path)::+;)*
        $(pub use $($syscall_path)::+::$($syscall_path)::+;)*

        pub mod numbers {
            $(pub const $syscall_ident : usize = $syscall_number;)*
        }

        pub mod labels {
            $(pub const $syscall_ident : &str = $syscall_label;)*
        }

        pub mod signatures {
            $(pub type $syscall_ident = $($syscall_signature)::+;)*
        }

        #[repr(usize)]
        enum Syscall {
            $(
                $syscall_ident = $syscall_number,
            )*
            TODO = usize::MAX,
        }

        impl Syscall {
            fn to_no(&self) -> usize {
                match self {
                    $(Syscall::$syscall_ident => $syscall_number,)*
                    Syscall::TODO => <usize>::MAX,
                }
            }

            fn from_no(n: usize) -> Syscall {
                match n {
                    $($syscall_number => Syscall::$syscall_ident,)*
                    _ => Syscall::TODO,
                }
            }
        }


        impl Into<usize> for Syscall {
            fn into(self) -> usize {
                self.to_no()
            }
        }
    };
}
