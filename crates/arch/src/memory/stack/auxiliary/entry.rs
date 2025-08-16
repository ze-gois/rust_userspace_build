use super::atype;

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

    pub fn key(&self) -> atype::Type {
        atype::Type::from(unsafe { *self.pointer.0 as usize })
    }

    pub fn value(&self) -> atype::EnumTyped {
        unsafe {
            atype::EnumTyped::from_kv(
                self.pointer.0 as *mut usize,
                (self.pointer.0 as *mut usize).add(1) as atype::macro_types::Mus,
            )
        }
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            // let cstr = core::ffi::CStr::from_ptr(self.pointer.0 as *const i8);
            let cstr = self.pointer.0;

            write!(f, "Entry: {{ ");
            write!(f, "{:?}, ", self.value());
            write!(f, " }}");
            return Ok(());
        }
    }
}
