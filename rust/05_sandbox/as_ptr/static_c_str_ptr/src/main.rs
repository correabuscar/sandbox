#![feature(const_cstr_from_ptr)]

static FORMAT_STRING: &[u8; 3] = b"%s\0";
static mut FORMAT_STRING_PTR: *const u8 = FORMAT_STRING.as_ptr();//no errors
//static FORMAT_STRING_PTR2: *const u8 = FORMAT_STRING.as_ptr();// XXX: error, can't be shared between threads

//see: https://users.rust-lang.org/t/why-static-mut-can-be-shared-between-threads-safely-but-without-mut-it-cant/109688

//"Mutable statics have the same restrictions as normal statics, except that the type does not have
//to implement the Sync trait." - https://doc.rust-lang.org/reference/items/static-items.html
// and unsafe is needed when reading/writing it.

//src: https://github.com/rust-lang/rust/issues/53639
pub struct CustomSynchronizingAbstraction {
    ptr: *const u8,
}
// Promise that proper synchronization exists *around accesses*.
unsafe impl Sync for CustomSynchronizingAbstraction{}

static FORMAT_STRING2: &std::ffi::CStr= unsafe { std::ffi::CStr::from_ptr("%s\0".as_ptr() as *const i8) };
static FORMAT_STRING_PTR2: CustomSynchronizingAbstraction = CustomSynchronizingAbstraction { ptr:FORMAT_STRING.as_ptr() };

//static FORMAT_STRING_PTR3: std::sync::OnceLock<*const u8> = std::sync::OnceLock::new();
mod hidden {
    pub struct SyncAbs1 {
        ptr: *const u8,
    }
    // Promise that proper synchronization exists *around accesses*.
    unsafe impl Sync for SyncAbs1 {}
    pub static FORMAT_STRING_PTR4: SyncAbs1 = SyncAbs1 {
        ptr: crate::FORMAT_STRING.as_ptr() //XXX: hmm, a diff. pointer address than for the others
                                           //outside of the module!
    };

    impl SyncAbs1 {
        #[inline]
        pub fn get(&self) -> *const u8 {
            self.ptr
        }
    }
}

fn main() {
    println!("Hello, world! '{:?}''{:?}'", FORMAT_STRING, FORMAT_STRING_PTR2.ptr);
    println!("Hello, world! '{:?}''{:?}'", FORMAT_STRING2, FORMAT_STRING_PTR2.ptr as *const u8);
    println!("Hello, world! '{:?}''{:?}'", FORMAT_STRING2, hidden::FORMAT_STRING_PTR4.get() as *const u8);
    // Convert the pointer back into a reference to [u8; 3]
    let ptr=hidden::FORMAT_STRING_PTR4.get();
    let reference_to_array: &[u8; 3] = unsafe {
        // Safety: We know that ptr points to a valid memory location holding a [u8; 3]
        // and the reference will have the same lifetime as the static array.
        // Thus, it's safe to create a reference from the pointer.
        //&*ptr.cast::<[u8; 3]>()//this works too
        &*(ptr as *const [u8; 3])
    };

    println!("{:?}", reference_to_array);//got FORMAT_STRING from the pointer
    // Create a mutable copy of the array
    // FIXME: to disable mutating it, i guess we would have to never expose the pointer directly
    // so, maybe wrapper for the function call that wants to be used, but even so, then that
    // function can access it, but i guess we'd assume it won't touch it, or double check contents
    // before/after.
    let mut mutable_array = *reference_to_array;//wrong way, this is making a copy (thanks chatgpt3.5 /s)
    mutable_array[1]=b'd';
    // Now you can use reference_to_array as needed
    println!("{:?}", mutable_array);//got FORMAT_STRING from the pointer
    println!("{:?}", reference_to_array);//got FORMAT_STRING from the pointer
}
