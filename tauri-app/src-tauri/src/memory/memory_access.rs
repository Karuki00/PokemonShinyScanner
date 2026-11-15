use std::ffi::c_void;

use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};

use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;

use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};

/// Represents an open handle to another process.
pub struct ProcessMemory {
    handle: HANDLE,
}

impl ProcessMemory {
    /// Open a process with memory-reading permissions.
    pub fn open(pid: u32) -> Result<Self, String> {
        let handle = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid) };

        if handle.is_null() {
            return Err(format!(
                "Failed to open process {}. Windows error: {}",
                pid,
                unsafe { GetLastError() }
            ));
        }

        Ok(Self { handle })
    }

    /// Read bytes from the target process memory.
    pub fn read(&self, address: usize, size: usize) -> Result<Vec<u8>, String> {
        if size == 0 {
            return Ok(Vec::new());
        }

        if address == 0 {
            return Err("Cannot read from a null address.".to_string());
        }

        let mut buffer = vec![0u8; size];
        let mut bytes_read: usize = 0;

        let success = unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                size,
                &mut bytes_read,
            )
        };

        if success == 0 {
            return Err(format!(
                "ReadProcessMemory failed at {:#X}. Windows error: {}",
                address,
                unsafe { GetLastError() }
            ));
        }

        if bytes_read != size {
            return Err(format!(
                "Incomplete memory read at {:#X}: expected {} bytes, received {}.",
                address, size, bytes_read
            ));
        }

        Ok(buffer)
    }
}

impl Drop for ProcessMemory {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}
