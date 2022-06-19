use std::error::Error;
use semver::{Version, Prerelease};

fn main() -> Result<(), Box<dyn Error>> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert_ne!(version_1.pre, Prerelease::EMPTY);
    assert_eq!(version_2.pre, Prerelease::EMPTY);

    Ok(())
}
