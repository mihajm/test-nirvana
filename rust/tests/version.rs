use test_nirvana::{
    is_power_version, next_power_version, parse_power_version, PowerVersion, PowerVersionComponent,
};

#[test]
fn accepts_power_versions_with_semver_metadata() {
    let parsed = parse_power_version("7.11.33-rc.1+moon.full").expect("valid Power Version");
    assert_eq!(
        parsed,
        PowerVersion {
            power: 7,
            master: 11,
            aura: 33,
            prerelease: Some("rc.1".to_string()),
            build: Some("moon.full".to_string()),
        }
    );
}

#[test]
fn rejects_profane_versions() {
    for version in ["0.1.1", "10.11.22", "7.11", "v7.11.33"] {
        assert!(!is_power_version(version), "expected {version} to be rejected");
    }
}

#[test]
fn advances_versions_by_component() {
    let cases = [
        ("7.11.22", PowerVersionComponent::Aura, "7.11.33"),
        ("7.11.33", PowerVersionComponent::Aura, "7.22.1"),
        ("7.33.33", PowerVersionComponent::Aura, "8.1.1"),
        ("7.11.22", PowerVersionComponent::Master, "7.22.1"),
        ("7.33.22", PowerVersionComponent::Master, "8.1.1"),
        ("7.11.22", PowerVersionComponent::Power, "8.1.1"),
    ];
    for (version, component, expected) in cases {
        assert_eq!(next_power_version(version, component).unwrap(), expected);
    }
}

#[test]
fn recognizes_completion() {
    let err = next_power_version("33.33.33", PowerVersionComponent::default())
        .expect_err("33.33.33 has no successor");
    assert!(err.to_string().contains("Archive the project"));
}
