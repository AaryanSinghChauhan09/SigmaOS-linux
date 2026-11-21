//! Integration tests for sigmaos-linux.

use linux::{apply_fragments, kernel_version_from_makefile, parse_fragment, pkgrel_bump};

#[test]
fn test_kernel_version_full() {
    let mk = "VERSION = 6\nPATCHLEVEL = 12\nSUBLEVEL = 1\nEXTRAVERSION =\n";
    let v = kernel_version_from_makefile(mk).unwrap();
    assert_eq!(v.kernel_string(), "6.12.1");
    assert_eq!(v.pkgver(), "6.12.1");
}

#[test]
fn test_config_fragment_pipeline() {
    let base = "CONFIG_64BIT=y\n# CONFIG_EXPERT is not set\nCONFIG_MODULES=y\n";
    let frag = parse_fragment("CONFIG_EXPERT=y\nCONFIG_MODULE_SIG=y\n");
    let out = apply_fragments(base, &[frag]);
    assert!(out.contains("CONFIG_MODULE_SIG=y"));
    assert!(out.contains("CONFIG_EXPERT=y"));
    assert!(!out.contains("# CONFIG_EXPERT is not set"));
}

#[test]
fn test_pkgrel_workflow() {
    let cur = "3";
    assert_eq!(pkgrel_bump(cur).unwrap(), 4);
}
