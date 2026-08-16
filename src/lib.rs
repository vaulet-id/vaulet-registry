//! The reserved claim vocabulary, embedded at compile time.
//!
//! **Reserved is what this crate lists, not what a name looks like.** A
//! standard namespace and a private one are both reverse-DNS —
//! `org.iso.18013.5.1` and `th.co.codefin` — so no rule about the shape of the
//! string can separate them, and an earlier attempt to use "does it contain a
//! dot" broke the moment the real standards were checked.

/// Words a credential may carry bare.
pub fn universal() -> &'static str {
    include_str!("../claims/universal.json")
}

/// Reserved namespaces: pointers to vocabularies others publish, and the few
/// definitions nobody else has written down.
pub fn namespaces() -> &'static str {
    include_str!("../claims/namespaces.json")
}

/// Identifier prefixes nobody may be allocated, so that nothing can be handed
/// out that would read as a standard vocabulary.
pub fn refused_prefixes() -> &'static str {
    include_str!("../claims/refused.json")
}

#[cfg(test)]
mod tests {
    /// **`same_as` points at something, and only one hop.**
    ///
    /// A chain resolves differently depending on how many hops a reader takes,
    /// which is a disagreement waiting to happen between two implementations
    /// that both read this file. `nationality` was such a chain: bare to the
    /// EUDI PID's word to ISO's.
    #[test]
    fn an_equivalence_lands_on_a_word_that_is_not_itself_equivalent() {
        let u: serde_json::Value = serde_json::from_str(super::universal()).unwrap();
        let n: serde_json::Value = serde_json::from_str(super::namespaces()).unwrap();

        let mut known = std::collections::HashSet::new();
        let mut points = std::collections::HashMap::new();
        for c in u["claims"].as_array().unwrap() {
            let name = c["name"].as_str().unwrap().to_string();
            if let Some(t) = c["same_as"].as_str() {
                points.insert(name.clone(), t.to_string());
            }
            known.insert(name);
        }
        for ns in n["namespaces"].as_array().unwrap() {
            let space = ns["namespace"].as_str().unwrap();
            for c in ns["claims"].as_array().into_iter().flatten() {
                let name = format!("{space}:{}", c["name"].as_str().unwrap());
                if let Some(t) = c["same_as"].as_str() {
                    points.insert(name.clone(), t.to_string());
                }
                known.insert(name);
            }
        }

        for (from, to) in &points {
            assert!(known.contains(to), "{from} points at {to}, which is not a word here");
            assert!(!points.contains_key(to), "{from} points at {to}, which points on again");
        }
    }

    /// The files parse, and say the three things an entry has to say.
    ///
    /// **A name without a kind is half a word.** One issuer writes a date and
    /// another writes Thai text into the same field, and both are following
    /// the register.
    #[test]
    fn every_defined_word_has_a_kind_and_a_meaning() {
        let u: serde_json::Value = serde_json::from_str(super::universal()).unwrap();
        for c in u["claims"].as_array().unwrap() {
            // A legacy spelling says only what it is really the name of. It is
            // not a word anybody should be offered, so it carries no meaning
            // to offer them.
            if c["legacy"] == true {
                assert!(c["name"].is_string() && c["same_as"].is_string(), "{c}");
                continue;
            }
            assert!(c["name"].is_string() && c["kind"].is_string() && c["meaning"].is_string(), "{c}");
        }

        let n: serde_json::Value = serde_json::from_str(super::namespaces()).unwrap();
        for ns in n["namespaces"].as_array().unwrap() {
            match ns["kind"].as_str() {
                // A pointer records who defines the words and copies none of
                // them: a copy goes stale without anybody noticing.
                Some("pointer") => assert!(ns["defined_by"].is_string(), "{ns}"),
                Some("definition") => {
                    for c in ns["claims"].as_array().unwrap() {
                        assert!(
                            c["name"].is_string() && c["kind"].is_string() && c["meaning"].is_string(),
                            "{c}"
                        );
                    }
                }
                other => panic!("a namespace is a pointer or a definition, not {other:?}"),
            }
        }
    }

    /// Every refused prefix ends where a name would continue, so `org.isotope`
    /// is not refused because `org.iso.` is.
    #[test]
    fn a_refused_prefix_stops_at_a_boundary() {
        let r: serde_json::Value = serde_json::from_str(super::refused_prefixes()).unwrap();
        for p in r["prefixes"].as_array().unwrap() {
            assert!(p.as_str().unwrap().ends_with('.'), "{p}");
        }
    }
}
