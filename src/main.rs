use winapi::um::{
    winnt::*,
    sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO, LPSYSTEM_INFO}
};

fn main() {
    unsafe {
        let mut system_info: SYSTEM_INFO = std::mem::zeroed();
        GetNativeSystemInfo(&mut system_info as LPSYSTEM_INFO);

        let architecture = match system_info.u.s().wProcessorArchitecture {
            winapi::um::winnt::PROCESSOR_ARCHITECTURE_AMD64 => "x86_64",
            winapi::um::winnt::PROCESSOR_ARCHITECTURE_INTEL => "x86",
            winapi::um::winnt::PROCESSOR_ARCHITECTURE_ARM => "ARM",
            winapi::um::winnt::PROCESSOR_ARCHITECTURE_ARM64 => "AArch64",
            _ => "Unknown",
        };

        println!("Architecture: {}", architecture);
    }
}
