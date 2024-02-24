use serde::Deserialize;

pub fn hello(who: Option<&str>) -> String {
    const WHO: &str = "world";
    let who = match who {
        Some(s) if s.is_empty() => WHO,
        Some(s) => s,
        None => WHO,
    };
    format!("Hello, {who}!")
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
        assert_eq!(super::hello(Some("Dave")), "Hello, Dave!");
    }

    #[test]
    fn hello_default() {
        assert_eq!(super::hello(None), "Hello, world!");
        assert_eq!(super::hello(Some("")), "Hello, world!");
    }

    #[test]
    fn version() {
        assert!(super::version().starts_with("0."));
    }
}
