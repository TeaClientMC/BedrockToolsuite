use std::{env, ffi::CString, mem::size_of, path::Path, ptr::null_mut};

pub static WINDOWS_PROC: &str = "Minecraft.Windows.exe";

pub struct Injection {
    dll: Path,
}

impl Injection {
    pub fn new() {}

    fn getProcId {

    }

    pub fn inject() {
        let openProcess = unsafe {
            OpenProcess()
        }
    }
}
