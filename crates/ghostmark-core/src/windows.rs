//! Read-only Win32 adapter. Do not introduce mutation APIs into this module.

use crate::environment::{Architecture, Environment, Platform, WindowsVersion};
use std::mem::{size_of, size_of_val};
use windows::{
    Wdk::System::SystemServices::RtlGetVersion,
    Win32::{
        Globalization::GetUserDefaultLocaleName,
        System::{
            Registry::{HKEY_LOCAL_MACHINE, RRF_RT_REG_DWORD, RRF_RT_REG_SZ, RegGetValueW},
            SystemInformation::{
                IMAGE_FILE_MACHINE, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64,
                IMAGE_FILE_MACHINE_I386, IMAGE_FILE_MACHINE_UNKNOWN, OSVERSIONINFOEXW,
            },
            Threading::{GetCurrentProcess, IsWow64Process2},
        },
        UI::{
            HiDpi::GetDpiForSystem,
            WindowsAndMessaging::{GetSystemMetrics, SM_CMONITORS, SM_REMOTESESSION},
        },
    },
    core::w,
};

pub(crate) fn read_environment() -> Environment {
    let mut environment = Environment::other_platform();
    environment.platform = Platform::Windows;
    let mut version = OSVERSIONINFOEXW {
        dwOSVersionInfoSize: size_of::<OSVERSIONINFOEXW>() as u32,
        ..Default::default()
    };
    // SAFETY: RtlGetVersion accepts the extended layout when its exact size is set;
    // this uniquely borrowed, initialized allocation outlives the synchronous call.
    let status = unsafe { RtlGetVersion((&mut version as *mut OSVERSIONINFOEXW).cast()) };
    if status.0 >= 0 {
        environment.version = Some(WindowsVersion {
            major: version.dwMajorVersion,
            minor: version.dwMinorVersion,
            build: version.dwBuildNumber,
            revision: read_revision(&mut environment.warnings),
            product_type: version.wProductType,
        });
    } else {
        environment
            .warnings
            .push(format!("version: NTSTATUS {:#010x}", status.0 as u32));
    }

    let mut process_machine = IMAGE_FILE_MACHINE_UNKNOWN;
    let mut native_machine = IMAGE_FILE_MACHINE_UNKNOWN;
    // SAFETY: the current-process pseudo-handle needs no closing. Both initialized outputs
    // are uniquely borrowed and remain valid for this synchronous, read-only query.
    match unsafe {
        IsWow64Process2(
            GetCurrentProcess(),
            &mut process_machine,
            Some(&mut native_machine),
        )
    } {
        Ok(()) => environment.native_architecture = native_architecture(native_machine),
        Err(error) => environment.warnings.push(format!(
            "architecture: HRESULT {:#010x}",
            error.code().0 as u32
        )),
    }
    environment.edition = read_edition(&mut environment.warnings);

    let mut locale = [0u16; 85];
    // SAFETY: the slice exposes its actual capacity; the API writes at most that many UTF-16 units.
    let length = unsafe { GetUserDefaultLocaleName(&mut locale) };
    if length > 1 && length as usize <= locale.len() {
        environment.locale = String::from_utf16(&locale[..length as usize - 1]).ok();
    }
    if environment.locale.is_none() {
        environment
            .warnings
            .push("locale: unavailable or invalid UTF-16".into());
    }
    // SAFETY: these parameter-free/read-only metric queries have no borrowed or retained pointers.
    let dpi = unsafe { GetDpiForSystem() };
    environment.system_dpi = (dpi > 0).then_some(dpi);
    // SAFETY: SM_CMONITORS is a documented read-only metric, with no pointer parameters.
    let monitors = unsafe { GetSystemMetrics(SM_CMONITORS) };
    environment.monitor_count = (monitors > 0).then_some(monitors as u32);
    // SAFETY: SM_REMOTESESSION is a documented read-only metric, with no pointer parameters.
    environment.remote_session = Some(unsafe { GetSystemMetrics(SM_REMOTESESSION) } != 0);
    environment
}

fn native_architecture(machine: IMAGE_FILE_MACHINE) -> Architecture {
    match machine {
        IMAGE_FILE_MACHINE_AMD64 => Architecture::X86_64,
        IMAGE_FILE_MACHINE_I386 => Architecture::X86,
        IMAGE_FILE_MACHINE_ARM64 => Architecture::Arm64,
        _ => Architecture::Unknown,
    }
}

fn read_revision(warnings: &mut Vec<String>) -> Option<u32> {
    let mut revision = 0u32;
    let mut bytes = size_of::<u32>() as u32;
    // SAFETY: the predefined key and NUL-terminated literals are valid. The output is
    // aligned, sized for one DWORD, and remains exclusively borrowed for the call.
    let result = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"),
            w!("UBR"),
            RRF_RT_REG_DWORD,
            None,
            Some((&mut revision as *mut u32).cast()),
            Some(&mut bytes),
        )
    };
    if result.0 == 0 && bytes == size_of::<u32>() as u32 {
        Some(revision)
    } else {
        warnings.push(format!("revision: WIN32_ERROR {}", result.0));
        None
    }
}

fn read_edition(warnings: &mut Vec<String>) -> Option<String> {
    let mut value = [0u16; 128];
    let mut bytes = size_of_val(&value) as u32;
    // SAFETY: fixed NUL-terminated key/value names and a writable UTF-16 buffer with
    // its byte capacity are supplied. RegGetValueW never retains these pointers.
    let result = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"),
            w!("EditionID"),
            RRF_RT_REG_SZ,
            None,
            Some(value.as_mut_ptr().cast()),
            Some(&mut bytes),
        )
    };
    if result.0 == 0
        && bytes >= 2
        && bytes as usize <= size_of_val(&value)
        && bytes.is_multiple_of(2)
    {
        let units = bytes as usize / 2;
        if value[units - 1] == 0
            && let Ok(edition) = String::from_utf16(&value[..units - 1])
        {
            return Some(edition);
        }
    }
    warnings.push(format!(
        "edition: WIN32_ERROR {} or invalid UTF-16",
        result.0
    ));
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::assess;

    #[test]
    fn native_machine_type_controls_compatibility_of_an_x64_process() {
        let mut environment: Environment =
            serde_json::from_str(include_str!("../tests/fixtures/windows11-unverified.json"))
                .unwrap();
        environment.process_architecture = Architecture::X86_64;
        for (machine, architecture, reason) in [
            (
                IMAGE_FILE_MACHINE_AMD64,
                Architecture::X86_64,
                "build_not_verified",
            ),
            (
                IMAGE_FILE_MACHINE_ARM64,
                Architecture::Arm64,
                "native_x64_process_required",
            ),
            (
                IMAGE_FILE_MACHINE_I386,
                Architecture::X86,
                "native_x64_process_required",
            ),
            (
                IMAGE_FILE_MACHINE_UNKNOWN,
                Architecture::Unknown,
                "architecture_unavailable",
            ),
            (
                IMAGE_FILE_MACHINE(0xffff),
                Architecture::Unknown,
                "architecture_unavailable",
            ),
        ] {
            environment.native_architecture = native_architecture(machine);
            assert_eq!(environment.native_architecture, architecture);
            assert_eq!(assess(&environment).reason, reason);
        }
    }
}
