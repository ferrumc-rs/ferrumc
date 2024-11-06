use std::env;
use std::sync::LazyLock;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorLevel {
    None,
    Basic,
    Enhanced,
    TrueColor,
}

impl ColorLevel {
    pub fn to_bool(&self) -> bool {
        matches!(self, ColorLevel::Basic | ColorLevel::Enhanced | ColorLevel::TrueColor)
    }

    pub fn new() -> Self {
        Self::None
    }

    pub fn update(self, new_level: Self) -> Self {
        match (&self, new_level) {
            (ColorLevel::None, level) => level,
            (_, level) if level > self => level,
            _ => self,
        }
    }

    pub fn force_none() -> Self {
        Self::None
    }
}

impl Default for ColorLevel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialOrd for ColorLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (Self::None, _) => std::cmp::Ordering::Less,
            (_, Self::None) => std::cmp::Ordering::Greater,
            (Self::Basic, Self::Basic) => std::cmp::Ordering::Equal,
            (Self::Basic, Self::Enhanced) => std::cmp::Ordering::Less,
            (Self::Basic, Self::TrueColor) => std::cmp::Ordering::Less,
            (Self::Enhanced, Self::Basic) => std::cmp::Ordering::Greater,
            (Self::Enhanced, Self::Enhanced) => std::cmp::Ordering::Equal,
            (Self::Enhanced, Self::TrueColor) => std::cmp::Ordering::Less,
            (Self::TrueColor, Self::Basic) => std::cmp::Ordering::Greater,
            (Self::TrueColor, Self::Enhanced) => std::cmp::Ordering::Greater,
            (Self::TrueColor, Self::TrueColor) => std::cmp::Ordering::Equal,
        })
    }
}

static COLOR_LEVEL: LazyLock<ColorLevel> = LazyLock::new(|| determine_color_level());

pub fn supports_color() -> bool {
    COLOR_LEVEL.to_bool()
}

fn determine_color_level() -> ColorLevel {

    // Check FORCE_COLOR environment variable first
    if let Some(color) = force_color_check() {
        return color;
    }

    // Windows checks
    if let Some(color) = windows_color_check() {
        return color;
    }

    // CI check
    if let Some(color) = ci_color_check() {
        return color;
    }

    // Term check
    if let Some(color) = term_color_check() {
        return color;
    }

    // Term Program check
    if let Some(color) = term_program_check() {
        return color;
    }

    // Final fallback based on whether the stream is a TTY
    if let Some(color) = tty_color_check() {
        return color;
    }

    ColorLevel::None
}

fn force_color_check() -> Option<ColorLevel> {
    env::var("FORCE_COLOR").ok().and_then(|val| match val.as_str() {
        "0" => Some(ColorLevel::None),
        "1" => Some(ColorLevel::Basic),
        "2" => Some(ColorLevel::Enhanced),
        "3" => Some(ColorLevel::TrueColor),
        _ => None,
    })
}

fn windows_color_check() -> Option<ColorLevel> {
    if std::env::consts::OS == "windows" {
        return Some(ColorLevel::Basic);
    }
    None
}

fn ci_color_check() -> Option<ColorLevel> {
    if env::var("CI").is_ok() {
        if env::var("GITHUB_ACTIONS").is_ok() || env::var("GITEA_ACTIONS").is_ok() {
            return Some(ColorLevel::TrueColor);
        }
        let ci_providers = ["TRAVIS", "CIRCLECI", "APPVEYOR", "GITLAB_CI", "BUILDKITE", "DRONE", "codeship"];
        if ci_providers.iter().any(|ci| env::var(ci).is_ok()) {
            return Some(ColorLevel::Basic);
        }
    }
    None
}

fn term_color_check() -> Option<ColorLevel> {
    match env::var("TERM").as_deref() {
        Ok("dumb") => None,
        Ok("xterm-kitty") | Ok("truecolor") | Ok("ansi") => Some(ColorLevel::TrueColor),
        Ok(term) if term.ends_with("-256color") => Some(ColorLevel::Enhanced),
        Ok(term) if term.starts_with("xterm") || term.starts_with("screen") => Some(ColorLevel::Basic),
        Ok("cygwin") => Some(ColorLevel::Basic),
        _ => None,
    }
}

fn term_program_check() -> Option<ColorLevel> {
    match env::var("TERM_PROGRAM").as_deref() {
        Ok("iTerm.app") => Some(ColorLevel::TrueColor),
        Ok("Apple_Terminal") => Some(ColorLevel::Enhanced),
        _ => None,
    }
}

fn tty_color_check() -> Option<ColorLevel> {
    if atty::is(atty::Stream::Stdout) {
        return Some(ColorLevel::TrueColor);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // test to_bool
    #[test]
    fn test_to_bool() {
        assert!(!ColorLevel::None.to_bool(), "None should not be true");
        assert!(ColorLevel::Basic.to_bool(), "Basic should be true");
        assert!(ColorLevel::Enhanced.to_bool(), "Enhanced should be true");
        assert!(ColorLevel::TrueColor.to_bool(), "TrueColor should be true");
    }

    // test new
    #[test]
    fn test_new() {
        assert_eq!(ColorLevel::None, ColorLevel::new(), "None should be the default color level");
    }

    // test update
    #[test]
    fn test_update() {
        assert_eq!(ColorLevel::Basic, ColorLevel::None.update(ColorLevel::Basic), "Updating from None to Basic should change the color level");
        assert_eq!(ColorLevel::Enhanced, ColorLevel::Basic.update(ColorLevel::Enhanced), "Updating from Basic to Enhanced should change the color level");
        assert_eq!(ColorLevel::TrueColor, ColorLevel::Enhanced.update(ColorLevel::TrueColor), "Updating from Enhanced to TrueColor should change the color level");
        // should never downgrade
        // updating to none should not change the color level
        assert_eq!(ColorLevel::Basic, ColorLevel::Basic.update(ColorLevel::None), "Updating from Basic to None should not change the color level");
        assert_eq!(ColorLevel::Enhanced, ColorLevel::Enhanced.update(ColorLevel::None), "Updating from Enhanced to None should not change the color level");
        assert_eq!(ColorLevel::TrueColor, ColorLevel::TrueColor.update(ColorLevel::None), "Updating from TrueColor to None should not change the color level");
        // updating to a lower level should not change the color level
        assert_eq!(ColorLevel::Enhanced, ColorLevel::Enhanced.update(ColorLevel::Basic), "Updating from Enhanced to Basic should not change the color level");
        assert_eq!(ColorLevel::TrueColor, ColorLevel::TrueColor.update(ColorLevel::Enhanced), "Updating from TrueColor to Enhanced should not change the color level");

    }

    #[test]
    fn test_force_color_levels() {
        use std::env;

        // Define test cases as tuples: (env_var_value, expected_color_level)
        let test_cases = [
            ("0", ColorLevel::None),
            ("1", ColorLevel::Basic),
            ("2", ColorLevel::Enhanced),
            ("3", ColorLevel::TrueColor),
            // Add a case for an invalid test that should fail
            ("0", ColorLevel::Basic), // this case is intentionally wrong
        ];

        for (value, expected) in test_cases.iter() {
            env::set_var("FORCE_COLOR", value);
            let color_level = determine_color_level();

            // If the expected color level is ColorLevel::Basic, this case should fail
            if *value == "0" && *expected == ColorLevel::Basic {
                assert_ne!(color_level, *expected, "Expected failure for FORCE_COLOR = {}", value);
            } else {
                assert_eq!(color_level, *expected, "Unexpected color level for FORCE_COLOR = {}", value);
            }

            env::remove_var("FORCE_COLOR");
        }
    }

    // Test CI color level
    #[test]
    fn test_ci_color_levels() {
        // Define a list of CI providers and their expected ColorLevel
        let ci_providers = [
            ("GITHUB_ACTIONS", ColorLevel::TrueColor),
            ("GITEA_ACTIONS", ColorLevel::TrueColor),
            ("TRAVIS", ColorLevel::Basic),
            ("CIRCLECI", ColorLevel::Basic),
            ("APPVEYOR", ColorLevel::Basic),
            ("GITLAB_CI", ColorLevel::Basic),
            ("BUILDKITE", ColorLevel::Basic),
            ("DRONE", ColorLevel::Basic),
            ("codeship", ColorLevel::Basic),
        ];

        // Outer loop: Set the main "CI" environment variable to enable CI mode
        env::set_var("CI", "true");

        for (provider, expected_level) in ci_providers.iter() {
            // Set the specific CI provider environment variable
            env::set_var(provider, "true");

            // Check if the color level matches the expected level
            assert_eq!(
                &ci_color_check().unwrap_or(ColorLevel::None),
                expected_level,
                "Unexpected color level for CI provider {}",
                provider
            );

            // Clean up by removing the provider-specific environment variable
            env::remove_var(provider);
        }

        // Remove the "CI" environment variable to clean up
        env::remove_var("CI");
    }

}
