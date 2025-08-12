use human::info;

#[repr(C)]
pub struct Entry {
    pub prev: *mut Entry,
    pub next: *mut Entry,
    pub value_pointer: crate::Pointer, // armazenar o ponteiro cru
}

impl Entry {
    /// Constrói *in-place* em `dest` usando ponteiro para *char*
    pub fn init_from_argv_ptr(dest: *mut Entry, argv_ptr: crate::Pointer) {
        unsafe {
            core::ptr::write(
                dest,
                Entry {
                    prev: core::ptr::null_mut(),
                    next: core::ptr::null_mut(),
                    value_pointer: argv_ptr,
                },
            )
        };
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            // let cstr = core::ffi::CStr::from_ptr(self.value_pointer.0 as *const i8);
            let cstr = self.value_pointer.0;
            write!(
                f,
                "Entry: {{ {:?}, {:?}, {:?} }}",
                self.prev, self.next, cstr
            )
        }
    }
}
