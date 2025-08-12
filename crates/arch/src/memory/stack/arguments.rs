pub mod entry;
pub use entry::*;

// use crate::memory;

// use core::ptr;
// use human::info;

use crate::memory;
use human::info;

#[repr(C)]
#[derive(Debug)]
pub struct List {
    pub counter: usize,
    pub former: *mut Entry,
    pub latter: *mut Entry,
}

impl Default for List {
    fn default() -> List {
        List {
            counter: 0,
            former: core::ptr::null_mut(),
            latter: core::ptr::null_mut(),
        }
    }
}

impl List {
    #[rustfmt::skip]
    pub fn from_pointer(argc_pointer: crate::Pointer) -> (List, crate::Pointer) {
        let counter = unsafe { *argc_pointer.0 } as usize;
        let argv_ptrs = unsafe { argc_pointer.0.add(1) as *const crate::PointerType };
        let env_pointer = unsafe { argc_pointer.0.add(2 + counter) };

        if counter == 0 {
            return (List::default(), crate::Pointer(env_pointer));
        }

        // aloca bloco de Entries
        let entries_ptr = crate::memory::alloc::<Entry>(counter);

        unsafe {
            // preenche cada Entry in-place
            for i in 0..counter {
                let arg_ptr = *argv_ptrs.add(i);
                Entry::init_from_argv_ptr(entries_ptr.add(i), crate::Pointer(arg_ptr));
            }
            // liga prev/next
            for i in 0..counter {
                let cur = entries_ptr.add(i);
                (*cur).prev = if i == 0 { core::ptr::null_mut() } else { entries_ptr.add(i - 1) };
                (*cur).next = if i + 1 == counter { core::ptr::null_mut() } else { entries_ptr.add(i + 1) };
            }
        }

        let list = List {
            counter,
            former: entries_ptr,
            latter: unsafe { entries_ptr.add(counter - 1) },
        };

        (list, crate::Pointer(env_pointer))
    }

    pub fn print(&self) {
        info!("Arguments {{\n");
        for i in 0..self.counter as usize {
            if let Some(e) = self.get(i) {
                info!("\t{:?}\n", e);
            }
        }
        info!("}} Arguments \n");
    }

    pub fn print_arguments(&self) {
        info!("Argument count: {}\n", self.counter);
        for i in 0..self.counter as usize {
            if let Some(entry) = self.get(i) {
                // Assumindo Entry tem campo `value: *const u8` ou similar; ajustar conforme Entry real.
                unsafe {
                    // se Entry tiver método para converter a string, use-o aqui
                    info!("Arg {}: '{:?}'\n", i, entry.value_pointer);
                }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&Entry> {
        if index >= self.counter as usize || self.former.is_null() {
            return None;
        }
        unsafe { Some(&*self.former.add(index)) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Entry> {
        if index >= self.counter as usize || self.former.is_null() {
            return None;
        }
        unsafe { Some(&mut *self.former.add(index)) }
    }

    pub fn len(&self) -> usize {
        self.counter as usize
    }

    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }
}

pub struct Iter<'l> {
    list: &'l List,
    index: usize,
}

impl<'l> Iterator for Iter<'l> {
    type Item = &'l Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.counter as usize {
            return None;
        }
        let item = self.list.get(self.index);
        self.index += 1;
        item
    }
}

impl Drop for List {
    fn drop(&mut self) {
        if !self.former.is_null() && self.counter > 0 {
            unsafe {
                // primeiro dropar cada Entry in-place (isso chama Drop de Entry, se houver)
                for i in 0..self.counter as usize {
                    let entry_ptr = self.former.add(i);
                    core::ptr::drop_in_place(entry_ptr);
                }

                // desaloca o bloco que foi alocado por alloc
                let total_size = core::mem::size_of::<Entry>() * self.counter as usize;
                let aligned_size =
                    (total_size + memory::page::SIZE - 1) & !(memory::page::SIZE - 1);

                let _ = crate::memory::munmap(self.former as *mut u8, aligned_size);
                // opcional: limpar para evitar double-drop
                self.former = core::ptr::null_mut();
                self.latter = core::ptr::null_mut();
                self.counter = 0;
            }
        }
    }
}
