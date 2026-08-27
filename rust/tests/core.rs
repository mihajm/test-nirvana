use test_nirvana::{inspect_commit_aura, reduce_to_power_number, CommitAura, Error, Vibe};

#[test]
fn reduces_without_diminishing_master_numbers() {
    let cases = [(7.0, 7), (11.0, 11), (22.0, 22), (33.0, 33), (34.0, 7), (987.0, 6)];
    for (value, expected) in cases {
        assert_eq!(reduce_to_power_number(value), Ok(expected));
    }
}

#[test]
fn rejects_profane_total() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(
            matches!(reduce_to_power_number(value), Err(Error::InvalidTotal(_))),
            "expected {value} to be rejected"
        );
    }
}

#[test]
fn calculates_a_hexadecimal_commit_aura_deterministically() {
    let aura = inspect_commit_aura("a1b2c3").expect("a1b2c3 is a valid hash");
    assert_eq!(
        aura,
        CommitAura {
            destiny_number: 3,
            hash: "a1b2c3".to_string(),
            total: 12,
            vibe: Vibe::Neutral,
        }
    );
}

#[test]
fn curses_the_impossible_all_zero_hash() {
    let aura = inspect_commit_aura("0000000").expect("an all-zero hash is valid hex");
    assert_eq!(
        aura,
        CommitAura {
            destiny_number: 0,
            hash: "0000000".to_string(),
            total: 0,
            vibe: Vibe::Cursed,
        }
    );
}

#[test]
fn normalizes_superficial_casing_and_whitespace() {
    let aura = inspect_commit_aura("  AAAAAA  ").expect("AAAAAA is a valid hash");
    assert_eq!(aura.hash, "aaaaaa");
}

#[test]
fn rejects_matter_that_cannot_belong_to_a_commit_hash() {
    assert!(matches!(inspect_commit_aura("mercury"), Err(Error::InvalidHash(_))));
}
