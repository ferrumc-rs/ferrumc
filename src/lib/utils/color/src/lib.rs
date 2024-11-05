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

    let mut color_level = ColorLevel::new();

    // Check FORCE_COLOR environment variable first
    if let Ok(force_color) = env::var("FORCE_COLOR") {
        color_level = match force_color.as_str() {
            "0" => ColorLevel::force_none(),
            "1" => color_level.update(ColorLevel::Basic),
            "2" => color_level.update(ColorLevel::Enhanced),
            "3" => color_level.update(ColorLevel::TrueColor),
            _ => color_level,
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
        Ok("xterm-kitty") | Ok("truecolor") | Ok("ansi") => color_level = color_level.update(ColorLevel::TrueColor),
        Ok(term) if term.ends_with("-256color") => color_level = color_level.update(ColorLevel::Enhanced),
        Ok(term) if term.starts_with("xterm") || term.starts_with("screen") => color_level = color_level.update(ColorLevel::Basic),
        _ => (),
    }

    // Final fallback based on whether the stream is a TTY
    if atty::is(stream) {
        color_level = color_level.update(ColorLevel::Basic);
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
