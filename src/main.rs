#![windows_subsystem = "windows"]

use windows::Win32::System::SystemInformation::IMAGE_FILE_MACHINE;
use windows::Win32::System::SystemInformation::IMAGE_FILE_MACHINE_ARM64;
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::Win32::System::Threading::IsWow64Process2;


fn main() {
    let architecture = match is_arm64_process() {
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

fn is_arm64_process() -> bool {
    unsafe {
        let mut process_machine: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(0);
        let _ = IsWow64Process2(GetCurrentProcess(), &mut process_machine, None);
        return process_machine == IMAGE_FILE_MACHINE_ARM64;
    }
}
