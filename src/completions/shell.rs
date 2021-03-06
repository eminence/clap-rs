#[allow(deprecated, unused_imports)]
use std::ascii::AsciiExt;
use std::str::FromStr;
use std::fmt;

/// Describes which shell to produce a completions file for
#[cfg_attr(feature = "lints", allow(enum_variant_names))]
#[derive(Debug, Copy, Clone)]
pub enum Shell {
    /// Generates a .bash completion file for the Bourne Again SHell (BASH)
    Bash,
    /// Generates a .fish completion file for the Friendly Interactive SHell (fish)
    Fish,
    /// Generates a completion file for the Z SHell (ZSH)
    Zsh,
    /// Generates a completion file for PowerShell
    PowerShell,
}

impl Shell {
    /// A list of possible variants in `&'static str` form
    pub fn variants() -> [&'static str; 4] { ["zsh", "bash", "fish", "powershell"] }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ZSH" | _ if s.eq_ignore_ascii_case("zsh") => Ok(Shell::Zsh),
            "FISH" | _ if s.eq_ignore_ascii_case("fish") => Ok(Shell::Fish),
            "BASH" | _ if s.eq_ignore_ascii_case("bash") => Ok(Shell::Bash),
            "POWERSHELL" | _ if s.eq_ignore_ascii_case("powershell") => Ok(Shell::PowerShell),
            _ => Err(String::from("[valid values: bash, fish, zsh, powershell]")),
        }
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Shell::Bash => write!(f, "BASH"),
            Shell::Fish => write!(f, "FISH"),
            Shell::Zsh => write!(f, "ZSH"),
            Shell::PowerShell => write!(f, "POWERSHELL"),
        }
    }
}
