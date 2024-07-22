pub enum TargetTripletPlatforms {
    Darwin,
    Windows,
    Linux,
}

pub enum TargetTripletArchitecture {
    X86_64,
    ARM64,
}

pub struct ParserConfig {
    enabled_targets: Vec<(TargetTripletPlatforms, TargetTripletArchitecture)>,
    manifest: Option<String>,
}

impl ParserConfig {
    pub fn new(
        enabled_targets: Vec<(TargetTripletPlatforms, TargetTripletArchitecture)>,
        manifest: Option<String>,
    ) -> Self {
        Self {
            enabled_targets,
            manifest,
        }
    }
}
