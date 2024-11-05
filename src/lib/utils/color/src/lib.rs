use std::env;

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

pub fn supports_color() -> bool {
    determine_color_level().to_bool()
}


fn determine_color_level() -> ColorLevel {

    // Check FORCE_COLOR environment variable first
    if let Some(color) = force_color_check() {
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

    // Final fallback based on whether the stream is a TTY
    if let Some(color) = tty_color_check() {
        return color;
    }

    ColorLevel::None
}

fn force_color_check() -> Option<ColorLevel> {
    if env::var("FORCE_COLOR").is_ok() {
        match env::var("FORCE_COLOR").unwrap().as_str() {
            "0" => Some(ColorLevel::None),
            "1" => Some(ColorLevel::Basic),
            "2" => Some(ColorLevel::Enhanced),
            "3" => Some(ColorLevel::TrueColor),
            _ => None,
        }
    } else {
        None
    }
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

}
