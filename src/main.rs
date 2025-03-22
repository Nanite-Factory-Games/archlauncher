use winapi::um::{
    winnt::*,
    sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO, LPSYSTEM_INFO}
};

fn main() {
    unsafe {
        let mut system_info: SYSTEM_INFO = std::mem::zeroed();
        GetNativeSystemInfo(&mut system_info as LPSYSTEM_INFO);

        let architecture = match system_info.u.s().wProcessorArchitecture {
            PROCESSOR_ARCHITECTURE_AMD64 | PROCESSOR_ARCHITECTURE_INTEL => Some("x86_64"),
            PROCESSOR_ARCHITECTURE_ARM | PROCESSOR_ARCHITECTURE_ARM64=> Some("aarch64"),
            _ => None
        };

        // Get all the arguments after the executable name
        let mut args = std::env::args();
        args.next();
        let cli_arg = args.next().expect("No program name provided");
        // Execute the program in the folder of the architecture name
        let exe_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(architecture.expect("Architecture not supported"));
        let mut command = std::process::Command::new(exe_dir.join(cli_arg));
        command.args(args);        
        command.current_dir(exe_dir);
        command.spawn().expect("Failed to execute process");
    }
}
