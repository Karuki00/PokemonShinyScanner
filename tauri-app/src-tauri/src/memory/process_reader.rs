use std::mem::size_of;

use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE};

use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, MODULEENTRY32W, Module32FirstW, Module32NextW, PROCESSENTRY32W,
    Process32FirstW, Process32NextW, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub executable: String,
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub base_address: usize,
    pub size: usize,
}

// Find a running process by executable name.
pub fn find_process(executable_name: &str) -> Result<Option<ProcessInfo>, String> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if snapshot == INVALID_HANDLE_VALUE {
        return Err(format!(
            "Failed to create process snapshot. Windows error: {}",
            unsafe { GetLastError() }
        ));
    }

    let result = enumerate_processes(snapshot, executable_name);

    unsafe {
        CloseHandle(snapshot);
    }

    result
}

fn enumerate_processes(
    snapshot: HANDLE,
    executable_name: &str,
) -> Result<Option<ProcessInfo>, String> {
    let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };

    entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

    if unsafe { Process32FirstW(snapshot, &mut entry) } == 0 {
        return Err(format!(
            "Failed to enumerate processes. Windows error: {}",
            unsafe { GetLastError() }
        ));
    }

    loop {
        let length = entry
            .szExeFile
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(entry.szExeFile.len());

        let executable = String::from_utf16_lossy(&entry.szExeFile[..length]);

        if executable.eq_ignore_ascii_case(executable_name) {
            return Ok(Some(ProcessInfo {
                pid: entry.th32ProcessID,
                executable,
            }));
        }

        if unsafe { Process32NextW(snapshot, &mut entry) } == 0 {
            break;
        }
    }

    Ok(None)
}

// Find a loaded module inside a target process.
pub fn find_module(process_id: u32, module_name: &str) -> Result<Option<ModuleInfo>, String> {
    let snapshot =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id) };

    if snapshot == INVALID_HANDLE_VALUE {
        return Err(format!(
            "Failed to create module snapshot. Windows error: {}",
            unsafe { GetLastError() }
        ));
    }

    let result = enumerate_modules(snapshot, module_name);

    unsafe {
        CloseHandle(snapshot);
    }

    result
}

fn enumerate_modules(snapshot: HANDLE, module_name: &str) -> Result<Option<ModuleInfo>, String> {
    let mut entry: MODULEENTRY32W = unsafe { std::mem::zeroed() };

    entry.dwSize = size_of::<MODULEENTRY32W>() as u32;

    if unsafe { Module32FirstW(snapshot, &mut entry) } == 0 {
        return Err(format!(
            "Failed to enumerate modules. Windows error: {}",
            unsafe { GetLastError() }
        ));
    }

    loop {
        let length = entry
            .szModule
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(entry.szModule.len());

        let name = String::from_utf16_lossy(&entry.szModule[..length]);

        if name.eq_ignore_ascii_case(module_name) {
            return Ok(Some(ModuleInfo {
                name,
                base_address: entry.modBaseAddr as usize,
                size: entry.modBaseSize as usize,
            }));
        }

        if unsafe { Module32NextW(snapshot, &mut entry) } == 0 {
            break;
        }
    }

    Ok(None)
}
