extern crate process_memory;
extern crate winapi;
extern crate kernel32;
extern crate read_process_memory;
use std::io;
use std::mem::transmute;
use std::mem;
use std::ptr::null_mut;
use winapi::shared::ntdef::HANDLE;
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::winnt::{PROCESS_ALL_ACCESS};
use winapi::um::tlhelp32::{TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, MODULEENTRY32, CreateToolhelp32Snapshot, Module32First, Module32Next};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::memoryapi::{WriteProcessMemory};
use winapi::shared::minwindef::{LPVOID, LPCVOID};


fn main() {
    println!("warsim cheat");
    let pid = process_memory::platform::get_pid("Warsim.exe");
    if pid == 0 {
        println!("please launch warsim");
        return
    }

    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, pid) };
    if handle == null_mut() {
        return
    }

    while true {
        println!("\n# menu");
        println!("write x for:\n 0. exit\n 1. set gold\n 2. set public opinion");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("failed to read choice");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if choice == 0 {
            break
        }
        println!("write how many set: ");
        let mut howmany = String::new();
        io::stdin().read_line(&mut howmany)
            .expect("failed to read how many");
        let howmany: u32 = match howmany.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if choice == 1 {
            unsafe { set_gold(handle, pid, howmany) }
        } else if choice == 2 {
            unsafe { set_opinion(handle, pid, howmany) }
        }
        println!("ready");
    }
    return
}

unsafe fn set_gold(handle: HANDLE, pid: u32, howmany: u32) {
    let address = get_module_base_address(pid, "Warsim.exe");
    let mut bytes: [u8; 4] = transmute(howmany.to_le());

    write_u32(handle, (address as usize) + 0x4730CC, &mut bytes)
}

unsafe fn set_opinion(handle: HANDLE, pid: u32, howmany: u32) {
    let address = get_module_base_address(pid, "Warsim.exe");
    let mut bytes: [u8; 4] = transmute(howmany.to_le());

    write_u32(handle, (address as usize) + 0x473000, &mut bytes)
}

unsafe fn write_u32(handle: HANDLE, addr: usize, buf: &mut [u8]) {
    let addr_lpvoid: LPVOID = addr as *mut usize as *mut std::ffi::c_void;
    let buf_ptr: LPCVOID = buf.as_mut_ptr() as *mut _ as *mut std::ffi::c_void;

    WriteProcessMemory(handle, addr_lpvoid, buf_ptr, mem::size_of_val(buf), std::ptr::null_mut());
}

unsafe fn get_module_base_address(pid: u32, mod_name: &str) -> *mut u8 {
    let h_snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid);
   
    let mut mod_base_addr: *mut u8 = null_mut();
    let size: u32 = std::mem::size_of::<MODULEENTRY32>() as u32;

    if h_snap != INVALID_HANDLE_VALUE {
        let mut mod_entry = MODULEENTRY32 {
            dwSize: size,
            th32ModuleID: 0,
            th32ProcessID: 0,
            GlblcntUsage: 0,
            ProccntUsage: 0,
            modBaseAddr: null_mut(),
            modBaseSize: 0,
            hModule: null_mut(),
            szModule: [0; 256],
            szExePath: [0; 260],
        };

        let mod_ptr: *mut MODULEENTRY32 = &mut mod_entry;

        if Module32First(h_snap, mod_ptr) != 0 {
            while {
                let mut ctn = true;

                let name_vec: Vec<u8> = mod_entry.szModule.into_iter().map(|x| *x as u8).collect();

                let name: &str = std::str::from_utf8(&name_vec).unwrap().trim_matches(char::from(0));

                if name == mod_name {
                    mod_base_addr = mod_entry.modBaseAddr;
                    ctn = false
                } else {
                    (*mod_ptr).szModule = [0; 256];
                }
                
                Module32Next(h_snap, mod_ptr) != 0 && ctn
            } {}
        }
    }

    mod_base_addr
}