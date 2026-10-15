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
                GetNativeSystemInfo, OSVERSIONINFOEXW, PROCESSOR_ARCHITECTURE_AMD64,
                PROCESSOR_ARCHITECTURE_ARM64, PROCESSOR_ARCHITECTURE_INTEL, SYSTEM_INFO,
            },
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

    let mut info = SYSTEM_INFO::default();
    // SAFETY: GetNativeSystemInfo writes one SYSTEM_INFO to a valid, uniquely borrowed buffer.
    unsafe { GetNativeSystemInfo(&mut info) };
    // SAFETY: the documented processor-architecture member was initialized by GetNativeSystemInfo.
    environment.native_architecture =
        match unsafe { info.Anonymous.Anonymous.wProcessorArchitecture } {
            PROCESSOR_ARCHITECTURE_AMD64 => Architecture::X86_64,
            PROCESSOR_ARCHITECTURE_INTEL => Architecture::X86,
            PROCESSOR_ARCHITECTURE_ARM64 => Architecture::Arm64,
            _ => Architecture::Unknown,
        };
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
