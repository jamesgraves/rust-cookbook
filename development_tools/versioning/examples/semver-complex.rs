use std::error::Error;
use semver::{BuildMetadata, Prerelease, Version};

fn main() -> Result<(), Box<dyn Error>> {
    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str)?;

    assert_ne!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: Prerelease::new("125").unwrap(),
            build: BuildMetadata::new("7208347").unwrap(),
        }
    );

    assert_eq!(
        parsed_version.build,
        BuildMetadata::new("g72ee7853").unwrap(),
    );

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}
