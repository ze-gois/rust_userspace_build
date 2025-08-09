use crate::Arch;
use crate::arch::*;
use crate::traits::callable::Callable;

impl Callable for Arch {
    fn _syscall0(n: usize) -> Result {
        crate::info!("Syscall Input: 0x{:x}", n);
        let syscall_result = syscall0(n);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }

    fn _syscall1(n: usize, a1: usize) -> Result {
        crate::info!("Syscall Input: 0x{:x}, {}", n, a1);
        let syscall_result = syscall1(n, a1);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }

    fn _syscall2(n: usize, a1: usize, a2: usize) -> Result {
        crate::info!("Syscall Input: 0x{:x}, {}, {}", n, a1, a2);
        let syscall_result = syscall2(n, a1, a2);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }

    fn _syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result {
        crate::info!("Syscall Input: 0x{:x}, {}, {}, {}", n, a1, a2, a3);
        let syscall_result = syscall3(n, a1, a2, a3);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }

    fn _syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result {
        crate::info!("Syscall Input: 0x{:x}, {}, {}, {}, {}", n, a1, a2, a3, a4);
        let syscall_result = syscall4(n, a1, a2, a3, a4);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }

    fn _syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Result {
        crate::info!(
            "Syscall Input: 0x{:x}, {}, {}, {}, {}, {}",
            n,
            a1,
            a2,
            a3,
            a4,
            a5
        );
        let syscall_result = syscall5(n, a1, a2, a3, a4, a5);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }

    fn _syscall6(
        n: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
    ) -> Result {
        crate::info!(
            "Syscall Input: 0x{:x}, {}, {}, {}, {}, {}, {}",
            n,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6
        );
        let syscall_result = syscall6(n, a1, a2, a3, a4, a5, a6);
        crate::info!("Syscall Result: {:?}", syscall_result);
        syscall_result
    }
}
