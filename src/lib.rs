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
    /// The files parse, and say the three things an entry has to say.
    ///
    /// **A name without a kind is half a word.** One issuer writes a date and
    /// another writes Thai text into the same field, and both are following
    /// the register.
    #[test]
    fn every_defined_word_has_a_kind_and_a_meaning() {
        let u: serde_json::Value = serde_json::from_str(super::universal()).unwrap();
        for c in u["claims"].as_array().unwrap() {
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
