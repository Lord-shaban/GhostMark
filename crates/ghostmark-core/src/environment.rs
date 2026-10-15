//! Serializable, privacy-limited environment information and pure compatibility rules.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Windows,
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Architecture {
    X86_64,
    X86,
    Arm64,
    Other,
    Unknown,
}

impl Architecture {
    pub fn current_process() -> Self {
        match std::env::consts::ARCH {
            "x86_64" => Self::X86_64,
            "x86" => Self::X86,
            "aarch64" => Self::Arm64,
            _ => Self::Other,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WindowsVersion {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: Option<u32>,
    /// VER_NT_WORKSTATION is 1; server/domain-controller values are not desktop targets.
    pub product_type: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Environment {
    pub platform: Platform,
    pub version: Option<WindowsVersion>,
    pub native_architecture: Architecture,
    pub process_architecture: Architecture,
    pub edition: Option<String>,
    pub locale: Option<String>,
    /// System DPI only, not a per-monitor or application DPI measurement.
    pub system_dpi: Option<u32>,
    pub monitor_count: Option<u32>,
    pub remote_session: Option<bool>,
    pub warnings: Vec<String>,
}

impl Environment {
    pub fn other_platform() -> Self {
        Self {
            platform: Platform::Other,
            version: None,
            native_architecture: Architecture::Unknown,
            process_architecture: Architecture::current_process(),
            edition: None,
            locale: None,
            system_dpi: None,
            monitor_count: None,
            remote_session: None,
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityStatus {
    Supported,
    Unsupported,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Assessment {
    pub status: CompatibilityStatus,
    pub reason: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Inspection {
    pub schema_version: u32,
    pub mutation_capable: bool,
    pub environment: Environment,
    pub compatibility: Assessment,
}

impl Inspection {
    pub(crate) fn new(environment: Environment) -> Self {
        Self {
            schema_version: 1,
            mutation_capable: false,
            compatibility: assess(&environment),
            environment,
        }
    }
}

struct VerifiedBuild<'a> {
    version: WindowsVersion,
    edition: &'a str,
}

/// No build is authorized until an exact-build hide/restore experiment is accepted.
pub fn assess(environment: &Environment) -> Assessment {
    assess_against(environment, &[])
}

fn assess_against(environment: &Environment, verified: &[VerifiedBuild<'_>]) -> Assessment {
    use CompatibilityStatus::{Supported, Unknown, Unsupported};
    let result = |status, reason| Assessment { status, reason };
    if environment.platform != Platform::Windows {
        return result(Unsupported, "windows_required");
    }
    let Some(version) = environment.version else {
        return result(Unknown, "windows_version_unavailable");
    };
    if version.major == 0 || version.build == 0 || !(1..=3).contains(&version.product_type) {
        return result(Unknown, "invalid_windows_version");
    }
    if version.major != 10 || version.minor != 0 {
        return result(Unknown, "unrecognized_windows_version");
    }
    if version.product_type != 1 || version.build < 22000 {
        return result(Unsupported, "windows_11_desktop_required");
    }
    if environment.native_architecture == Architecture::Unknown
        || environment.process_architecture == Architecture::Unknown
    {
        return result(Unknown, "architecture_unavailable");
    }
    if environment.native_architecture != Architecture::X86_64
        || environment.process_architecture != Architecture::X86_64
    {
        return result(Unsupported, "native_x64_process_required");
    }
    if version.revision.is_none() {
        return result(Unknown, "build_revision_unavailable");
    }
    let Some(edition) = environment.edition.as_deref() else {
        return result(Unknown, "edition_unavailable");
    };
    if edition.is_empty()
        || edition.len() > 64
        || !edition.bytes().all(|c| c.is_ascii_alphanumeric())
    {
        return result(Unknown, "invalid_edition");
    }
    if verified
        .iter()
        .any(|item| item.version == version && item.edition == edition)
    {
        result(Supported, "exact_build_verified")
    } else {
        result(Unsupported, "build_not_verified")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Environment {
        serde_json::from_str(include_str!("../tests/fixtures/windows11-unverified.json")).unwrap()
    }

    #[test]
    fn valid_unverified_fixture_never_advertises_support() {
        assert_eq!(assess(&fixture()).reason, "build_not_verified");
        let report = Inspection::new(fixture());
        assert!(!report.mutation_capable);
        assert_eq!(
            report.compatibility.status,
            CompatibilityStatus::Unsupported
        );
    }

    #[test]
    fn support_requires_exact_revision_and_edition() {
        let mut environment = fixture();
        let verified = [VerifiedBuild {
            version: environment.version.unwrap(),
            edition: "Professional",
        }];
        assert_eq!(
            assess_against(&environment, &verified).status,
            CompatibilityStatus::Supported
        );
        environment.version.as_mut().unwrap().revision = Some(9999);
        assert_eq!(
            assess_against(&environment, &verified).status,
            CompatibilityStatus::Unsupported
        );
        environment.version = Some(verified[0].version);
        environment.edition = Some("Core".into());
        assert_eq!(
            assess_against(&environment, &verified).status,
            CompatibilityStatus::Unsupported
        );
    }

    #[test]
    fn missing_or_malformed_identity_is_unknown() {
        for (version, reason) in [
            (None, "windows_version_unavailable"),
            (
                Some(WindowsVersion {
                    major: 0,
                    minor: 0,
                    build: 0,
                    revision: None,
                    product_type: 0,
                }),
                "invalid_windows_version",
            ),
            (
                Some(WindowsVersion {
                    major: 11,
                    minor: 0,
                    build: 30000,
                    revision: Some(1),
                    product_type: 1,
                }),
                "unrecognized_windows_version",
            ),
        ] {
            let mut environment = fixture();
            environment.version = version;
            assert_eq!(
                assess(&environment),
                Assessment {
                    status: CompatibilityStatus::Unknown,
                    reason
                }
            );
        }
        let mut environment = fixture();
        environment.version.as_mut().unwrap().revision = None;
        assert_eq!(assess(&environment).reason, "build_revision_unavailable");
        environment = fixture();
        environment.edition = None;
        assert_eq!(assess(&environment).reason, "edition_unavailable");
        environment.edition = Some("Professional\nSupported".into());
        assert_eq!(assess(&environment).reason, "invalid_edition");
    }

    #[test]
    fn other_platforms_servers_and_old_clients_are_rejected() {
        assert_eq!(
            assess(&Environment::other_platform()).reason,
            "windows_required"
        );
        for (build, product_type) in [(19045, 1), (26100, 2), (26100, 3)] {
            let mut environment = fixture();
            let version = environment.version.as_mut().unwrap();
            version.build = build;
            version.product_type = product_type;
            assert_eq!(assess(&environment).reason, "windows_11_desktop_required");
        }
    }

    #[test]
    fn emulated_and_unknown_architectures_do_not_pass() {
        let mut environment = fixture();
        environment.native_architecture = Architecture::Arm64;
        assert_eq!(assess(&environment).reason, "native_x64_process_required");
        environment.native_architecture = Architecture::X86_64;
        environment.process_architecture = Architecture::X86;
        assert_eq!(
            assess(&environment).status,
            CompatibilityStatus::Unsupported
        );
        environment.process_architecture = Architecture::Unknown;
        assert_eq!(assess(&environment).status, CompatibilityStatus::Unknown);
    }
}
