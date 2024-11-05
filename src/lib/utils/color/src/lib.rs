use std::env;
use atty::Stream;

#[derive(Debug, PartialEq)]
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

    pub fn basic(&self) -> Self {
        match self {
            ColorLevel::Enhanced => ColorLevel::Enhanced,
            ColorLevel::TrueColor => ColorLevel::TrueColor,
            _ => ColorLevel::Basic,
        }
    }

    pub fn enhanced(&self) -> Self {
        match self {
            ColorLevel::TrueColor => ColorLevel::TrueColor,
            _ => ColorLevel::Enhanced,
        }
    }

    pub fn true_color(&self) -> Self {
        Self::TrueColor
    }

    pub fn force_none() -> Self {
        Self::None
    }
}
pub struct ColorSupport {
    pub stdout: ColorLevel,
    pub stderr: ColorLevel,
}

impl ColorSupport {
    pub fn new() -> Self {
        Self {
            stdout: determine_color_level(Stream::Stdout),
            stderr: determine_color_level(Stream::Stderr),
        }
    }

    pub fn supports_color(&self) -> bool {
        self.stdout.to_bool() || self.stderr.to_bool()
    }
}

fn determine_color_level(stream: Stream) -> ColorLevel {
    // Check FORCE_COLOR environment variable first
    let mut color_level = ColorLevel::new();

    if let Ok(force_color) = env::var("FORCE_COLOR") {
        match force_color.as_str() {
            "0" => color_level = ColorLevel::force_none(),
            "1" => color_level = ColorLevel::basic(&color_level),
            "2" => color_level = ColorLevel::enhanced(&color_level),
            "3" => color_level = ColorLevel::true_color(&color_level),
            _ => (),
        };
        if color_level == ColorLevel::None {
            return color_level;
        }
    }

    // Handle specific CI environments
    if env::var("CI").is_ok() {
        if env::var("GITHUB_ACTIONS").is_ok() || env::var("GITEA_ACTIONS").is_ok() {
            return ColorLevel::TrueColor;
        }

        let ci_providers = ["TRAVIS", "CIRCLECI", "APPVEYOR", "GITLAB_CI", "BUILDKITE", "DRONE", "codeship"];
        if ci_providers.iter().any(|ci| env::var(ci).is_ok()) {
            return ColorLevel::Basic;
        }
    }

    // Check terminal types and other environment variables
    match env::var("TERM").as_deref() {
        Ok("dumb") => (),
        Ok("xterm-kitty") | Ok("truecolor") | Ok("ansi") => color_level = ColorLevel::true_color(&color_level),
        Ok(term) if term.ends_with("-256color") => color_level = ColorLevel::enhanced(&color_level),
        Ok(term) if term.starts_with("xterm") || term.starts_with("screen") => color_level = ColorLevel::basic(&color_level),
        _ => (),
    }

    // Final fallback based on whether the stream is a TTY
    if atty::is(stream) {
        color_level = ColorLevel::basic(&color_level);
    }

    color_level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_color_level() {
        let color_support = ColorSupport::new();
        assert!(matches!(color_support.stdout, ColorLevel::Basic) || matches!(color_support.stdout, ColorLevel::None));
        assert!(matches!(color_support.stderr, ColorLevel::Basic) || matches!(color_support.stderr, ColorLevel::None));
    }

    #[test]
    fn test_force_color_env() {
        env::set_var("FORCE_COLOR", "3");
        assert_eq!(determine_color_level(Stream::Stdout), ColorLevel::TrueColor);
        env::remove_var("FORCE_COLOR");
    }
}
