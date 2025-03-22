#![windows_subsystem = "windows"]

use windows::core::BOOL;
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::Win32::System::Threading::IsWow64Process;


fn main() {
    let architecture = match is_wow64_process() {
        true => "aarch64",
        false => "x86_64",
    };

    // Get all the arguments after the executable name
    let mut args = std::env::args();
    args.next();
    let cli_arg = args.next().expect("No program name provided");
    // Execute the program in the folder of the architecture name
    let exe_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(architecture);
    
    let mut command = std::process::Command::new(exe_dir.join(cli_arg));
    command.args(args);        
    command.current_dir(exe_dir);
    command.spawn().expect("Failed to execute process");
}

fn is_wow64_process() -> bool {
    unsafe {
        let mut is_wow64: BOOL = BOOL(0);
        let result = IsWow64Process(GetCurrentProcess(), &mut is_wow64);
        result.is_ok() && is_wow64.as_bool()
    }
}
