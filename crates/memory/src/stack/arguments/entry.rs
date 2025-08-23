use human::info;

#[repr(C)]
pub struct Entry {
    pub prev: *mut Entry,
    pub next: *mut Entry,
    pub pointer: crate::Pointer, // armazenar o ponteiro cru
}

impl Entry {
    pub fn from_pointer(pointer: crate::Pointer) -> Entry {
        Entry {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            pointer: pointer,
        }
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            let cstr = self.pointer.0;

            write!(f, "Entry: {{ ");

            let cstr_ptr = self.pointer.0 as *mut i8;
            if !cstr_ptr.is_null() {
                let cstr = core::ffi::CStr::from_ptr(cstr_ptr);
                if let Ok(str_slice) = cstr.to_str() {
                    write!(f, "\"{}\", ", str_slice);
                } else {
                    write!(f, "<invalid utf8>, ");
                }
            }
            write!(f, " }}");
            return Ok(());
        }
    }
}
