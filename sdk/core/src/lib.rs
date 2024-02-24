use serde::Deserialize;

pub fn hello() -> &'static str {
    "Hello, world!"
}

pub fn version() -> String {
    // There are more sane ways to do this, but I want to force dependencies for the sample.
    let manifest: CargoManifest =
        toml::from_str(include_str!("../Cargo.toml")).expect("deserialize Cargo.toml");
    manifest.package.version
}

#[derive(Deserialize)]
struct CargoManifest {
    pub package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    pub version: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        assert_eq!(super::hello(), "Hello, world!");
    }

    #[test]
    fn version() {
        assert!(super::version().starts_with("0."));
    }
}
