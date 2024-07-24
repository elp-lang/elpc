#[derive(Debug, PartialEq)]
pub enum TargetTripletPlatforms {
    Darwin,
    Windows,
    Linux,
}

#[derive(Debug, PartialEq)]
pub enum TargetTripletArchitecture {
    ALL,
    X86_64,
    ARM64,
}

#[derive(Debug, PartialEq)]
pub struct ParserConfig<'a> {
    pub enabled_targets: &'a Vec<(TargetTripletPlatforms, Vec<TargetTripletArchitecture>)>,
    pub manifest: &'a Option<String>,
}

impl<'a> ParserConfig<'a> {
    pub fn new(
        enabled_targets: &'a Vec<(TargetTripletPlatforms, Vec<TargetTripletArchitecture>)>,
        manifest: &'a Option<String>,
    ) -> Self {
        Self {
            enabled_targets,
            manifest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ParserConfig, TargetTripletArchitecture, TargetTripletPlatforms};

    #[test]
    fn test_config() {
        let targets = vec![(
            TargetTripletPlatforms::Darwin,
            vec![TargetTripletArchitecture::ALL],
        )];
        let manifest_path = Some(".".into());
        let config = ParserConfig::new(&targets, &manifest_path);

        assert_eq!(config.enabled_targets[0], targets[0]);
        assert_eq!(
            config.manifest.clone().unwrap(),
            manifest_path.clone().unwrap()
        );
    }
}
