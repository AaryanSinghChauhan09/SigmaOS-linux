//! Kernel version model — parses `Makefile` VERSION/PATCHLEVEL/SUBLEVEL/
//! EXTRAVERSION and produces Arch pkgver/pkgrel.

use crate::KernelError;

/// Parsed kernel version components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelVersion {
    /// VERSION (e.g. 6).
    pub version: u32,
    /// PATCHLEVEL (e.g. 1).
    pub patchlevel: u32,
    /// SUBLEVEL (e.g. 0).
    pub sublevel: u32,
    /// EXTRAVERSION (e.g. "-rc1" or "").
    pub extraversion: String,
}

impl KernelVersion {
    /// `major.minor.patch[-extra]` form (kernel's own version string).
    pub fn kernel_string(&self) -> String {
        format!(
            "{}.{}.{}{}",
            self.version, self.patchlevel, self.sublevel, self.extraversion
        )
    }

    /// Arch pkgver: kernel string with `-` → `.` for extra (e.g. `6.1.0.rc1`).
    pub fn pkgver(&self) -> String {
        self.kernel_string().replace('-', ".")
    }

    /// Whether this is a release candidate/pre-release.
    pub fn is_prerelease(&self) -> bool {
        !self.extraversion.is_empty()
    }
}

/// Parse `VERSION = 6`, `PATCHLEVEL = 1`, `SUBLEVEL = 0`, `EXTRAVERSION = -rc1`
/// lines from a kernel `Makefile`.
pub fn kernel_version_from_makefile(text: &str) -> Result<KernelVersion, KernelError> {
    let mut version = None;
    let mut patchlevel = None;
    let mut sublevel = None;
    let mut extra = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        for (key, slot) in [
            ("VERSION", &mut version),
            ("PATCHLEVEL", &mut patchlevel),
            ("SUBLEVEL", &mut sublevel),
        ] {
            if let Some(rest) = line.strip_prefix(key) {
                let rest = rest.trim_start();
                if let Some(val) = rest.strip_prefix('=') {
                    if let Ok(n) = val.trim().parse::<u32>() {
                        *slot = Some(n);
                    }
                }
            }
        }
        if let Some(rest) = line.strip_prefix("EXTRAVERSION") {
            if let Some(val) = rest.trim_start().strip_prefix('=') {
                extra = val.trim().to_string();
            }
        }
    }
    match (version, patchlevel, sublevel) {
        (Some(v), Some(p), Some(s)) => Ok(KernelVersion {
            version: v,
            patchlevel: p,
            sublevel: s,
            extraversion: extra,
        }),
        _ => Err(KernelError::MakefileParse(
            "missing VERSION/PATCHLEVEL/SUBLEVEL".into(),
        )),
    }
}

/// Arch PKGBUILD `pkgver()`/`pkgrel` bump helper.
///
/// When rebuilding the same upstream version (e.g. config-only change),
/// Arch bumps `pkgrel`. `pkgrel_bump` returns the next integer.
pub fn pkgrel_bump(current: &str) -> Result<u32, KernelError> {
    current
        .trim()
        .parse::<u32>()
        .map(|n| n + 1)
        .map_err(|_| KernelError::InvalidVersion(format!("bad pkgrel `{current}`")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_makefile() {
        let text = "VERSION = 6\nPATCHLEVEL = 1\nSUBLEVEL = 55\nEXTRAVERSION =\n";
        let v = kernel_version_from_makefile(text).unwrap();
        assert_eq!(v.kernel_string(), "6.1.55");
        assert_eq!(v.pkgver(), "6.1.55");
        assert!(!v.is_prerelease());
    }

    #[test]
    fn test_rc_version() {
        let text = "VERSION = 6\nPATCHLEVEL = 8\nSUBLEVEL = 0\nEXTRAVERSION = -rc1\n";
        let v = kernel_version_from_makefile(text).unwrap();
        assert_eq!(v.pkgver(), "6.8.0.rc1");
        assert!(v.is_prerelease());
    }

    #[test]
    fn test_pkgrel_bump() {
        assert_eq!(pkgrel_bump("1").unwrap(), 2);
        assert_eq!(pkgrel_bump("15").unwrap(), 16);
        assert!(pkgrel_bump("x").is_err());
    }
}
