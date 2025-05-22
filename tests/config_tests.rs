#[cfg(test)]
mod tests {
    use orbit_analyzer::{Config, Severity};
    use std::path::Path;

    #[test]
    fn test_load_config() {
        // Get the path to example config file
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join(".orlint.toml");

        // Load the config
        let config = Config::from_file(&config_path).unwrap();

        // Verify some key settings
        assert!(config.analyzer.parallel);
        assert_eq!(config.renderer_analysis.default_renderer, "skia");
        assert!(config.is_rule_enabled("component-naming"));
        assert!(config.is_rule_enabled("prop-type-required"));

        // Check rule severities
        assert_eq!(
            config.get_rule_severity("non-empty-template", Severity::Warning),
            Severity::Error
        );
        assert_eq!(
            config.get_rule_severity("public-function", Severity::Warning),
            Severity::Info
        );
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();

        // Check some default values
        assert!(!config.analyzer.parallel);
        assert_eq!(config.renderer_analysis.default_renderer, "auto");
        assert!(config.renderer_analysis.enabled);

        // All rules should be enabled by default
        assert!(config.is_rule_enabled("component-naming"));
        assert!(config.is_rule_enabled("non-existent-rule"));

        // Default severities should be preserved
        assert_eq!(
            config.get_rule_severity("any-rule", Severity::Warning),
            Severity::Warning
        );
    }

    #[test]
    fn test_disabled_rules() {
        let mut config = Config::default();
        config.analyzer.disabled_rules = vec!["component-naming".to_string()];

        // The disabled rule should not be enabled
        assert!(!config.is_rule_enabled("component-naming"));
        // Other rules should still be enabled
        assert!(config.is_rule_enabled("prop-type-required"));
    }

    #[test]
    fn test_config_with_enabled_rules() {
        let mut config = Config::default();
        config.analyzer.enabled_rules = vec!["rule1".to_string(), "rule2".to_string()];

        // Only the explicitly enabled rules should be enabled
        assert!(config.is_rule_enabled("rule1"));
        assert!(config.is_rule_enabled("rule2"));
        assert!(!config.is_rule_enabled("rule3"));
    }
}
