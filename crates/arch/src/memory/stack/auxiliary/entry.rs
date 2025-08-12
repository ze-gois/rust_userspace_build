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
        // Create a slice from the raw pointer
        // let s = core::slice::from_raw_parts(env_ptr, l);
        // let c_str = unsafe { core::ffi::CStr::from_ptr(self.pointer.0 as *mut i8) };

        // let r_str = c_str.to_str().unwrap();

        // if let Some(pos) = r_str.find('=') {
        //     &r_str[..pos]
        // } else {
        //     r_str
        // }
        atype::Type::from(unsafe { *self.pointer.0 as usize })
    }

    pub fn value(&self) -> atype::EnumTyped {
        match self.key() {
            _ => atype::EnumTyped::TODO(0),
        }
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            // let cstr = core::ffi::CStr::from_ptr(self.pointer.0 as *const i8);
            let cstr = self.pointer.0;

            write!(f, "Entry: {{ ");
            write!(f, "{:?}, ", self.prev);
            write!(f, "{:?}, ", self.next);
            write!(f, "{:?}, ", self.key());
            write!(f, "{:?}, ", self.value());
            write!(f, "{:?}, ", self.pointer.0);

            // Print the argument string from pointer as CStr
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

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct Entry {
//     pub atype: usize,
//     pub value: usize,
// }

// impl Entry {
//     pub fn is_null(&self) -> bool {
//         self.atype == 0
//     }
// }
