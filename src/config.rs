//! Kernel `.config` fragment handling.
//!
//! Arch kernel PKGBUILDs apply config fragments (e.g. `config.x86_64`,
//! debug/RT fragments) on top of the defconfig. This module parses
//! fragment files (`CONFIG_X=y`, `# CONFIG_Y is not set`) and applies them
//! to a base config text.

/// A parsed config fragment: set/unset directives.
#[derive(Debug, Clone, Default)]
pub struct ConfigFragment {
    /// `CONFIG_X=y`/`CONFIG_X=m`/`CONFIG_X=\"str\"` directives.
    pub sets: Vec<(String, String)>,
    /// `# CONFIG_X is not set` directives.
    pub unsets: Vec<String>,
}

/// Parse a fragment file into [`ConfigFragment`].
pub fn parse_fragment(text: &str) -> ConfigFragment {
    let mut frag = ConfigFragment::default();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("#") {
            // Check for "# CONFIG_X is not set".
            if let Some(rest) = line.strip_prefix("# CONFIG_") {
                if let Some(name) = rest.strip_suffix(" is not set") {
                    frag.unsets.push(format!("CONFIG_{name}"));
                }
            }
            continue;
        }
        if let Some(eq) = line.find('=') {
            let (k, v) = line.split_at(eq);
            frag.sets.push((k.trim().to_string(), v[1..].trim().to_string()));
        }
    }
    frag
}

/// Apply fragments to a base `.config` text.
///
/// Later fragments override earlier ones; unsets remove the key.
pub fn apply_fragments(base: &str, fragments: &[ConfigFragment]) -> String {
    // Collect effective directives in order.
    let mut sets: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut unsets: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for frag in fragments {
        for (k, v) in &frag.sets {
            sets.insert(k.clone(), v.clone());
            unsets.remove(k);
        }
        for k in &frag.unsets {
            sets.remove(k);
            unsets.insert(k.clone());
        }
    }
    // Start from base config lines, then overlay.
    let mut out = String::new();
    let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for line in base.lines() {
        let key = line.split('=').next().unwrap_or("").trim().to_string();
        let unset_key = line
            .strip_prefix("# ")
            .and_then(|l| l.strip_suffix(" is not set"))
            .map(|s| s.trim().to_string());
        let effective_key = if key.starts_with("CONFIG_") { Some(key.clone()) } else { unset_key };
        if let Some(k) = effective_key {
            if sets.contains_key(&k) {
                out.push_str(&format!("{}={}\n", k, sets[&k]));
                seen.insert(k);
                continue;
            }
            if unsets.contains(&k) {
                out.push_str(&format!("# {} is not set\n", k));
                seen.insert(k);
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    // Append new keys not in base.
    for (k, v) in &sets {
        if !seen.contains(k) {
            out.push_str(&format!("{k}={v}\n"));
        }
    }
    for k in &unsets {
        if !seen.contains(k) {
            out.push_str(&format!("# {k} is not set\n"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fragment() {
        let frag = parse_fragment("CONFIG_FOO=y\nCONFIG_BAR=m\n# CONFIG_BAZ is not set\n");
        assert_eq!(frag.sets.len(), 2);
        assert_eq!(frag.unsets, vec!["CONFIG_BAZ"]);
    }

    #[test]
    fn test_apply_fragment_overrides() {
        let base = "CONFIG_A=y\nCONFIG_B=m\n";
        let frag = parse_fragment("CONFIG_B=n\nCONFIG_C=y\n");
        let out = apply_fragments(base, &[frag]);
        assert!(out.contains("CONFIG_B=n"));
        assert!(out.contains("CONFIG_C=y"));
        assert!(!out.contains("CONFIG_B=m"));
    }
}
