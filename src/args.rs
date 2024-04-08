use argh::FromArgs;
use iwrtb::uniq;

#[allow(dead_code)]
#[derive(FromArgs, Debug)]
/// Generates random password(s). Default is 16 chars of capital, lowercase letters and digits.
#[argh(help_triggers("-h", "--help"))] 
pub struct Args {
    #[argh(switch, short = 'l')]
    /// use lower case letters (default)
    pub lower: bool,

    #[argh(switch, short = 'L')]
    /// do NOT use lower case letters
    pub no_lower: bool,

    #[argh(switch, short = 'u')]
    /// use upper case letters (default)
    pub upper: bool,

    #[argh(switch, short = 'U')]
    /// do NOT use upper case letters
    pub no_upper: bool,

    #[argh(switch, short = 'd')]
    /// use digits (default)
    pub digits: bool,

    #[argh(switch, short = 'D')]
    /// do NOT use digits
    pub no_digits: bool,

    #[argh(switch, short = 's')]
    /// use special characters !$%@#
    pub special: bool,

    #[argh(switch, short = 'S')]
    /// do NOT use special characters (default)
    pub no_special: bool,

    #[argh(option, short = 'x')]
    /// extra special characters, implies -s
    pub extra: Vec<String>,

    #[argh(option, short = 'r')]
    /// replacement special characters, implies -s
    pub replacement: Option<String>,

    #[argh(switch, short = 'a')]
    /// password must conain chars of all classes
    pub all: bool,

    #[argh(option, short = 'n')]
    /// generate number of passwords (default is 1)
    pub num: Option<usize>,

    #[argh(switch, short = 'q')]
    /// do not output anything but the password(s)
    pub quiet: bool,

    #[argh(positional)]
    pub length: Option<usize>,

    #[argh(help_text)]
    pub usage: Option<String>,
}

pub struct Requset {
    pub lower: bool,
    pub upper: bool,
    pub digits: bool,
    pub special: Option<String>,
    pub all: bool,
    pub num: usize,
    pub quiet: bool,
    pub length: usize,
}

const DEFAULT_SPECIALS: &str = "!$%@#";

const DEFAULTS: Requset = Requset {
    lower: true,
    upper: true,
    digits: true,
    special: None,
    all: false,
    num: 1,
    quiet: false,
    length: 16,
};

impl Args {
    pub fn check(&self) -> Result<Requset, &'static str> {
        if self.lower && self.no_lower {
            return Err("Lower case letters both enabled and disabled")
        }
        if self.upper && self.no_upper {
            return Err("Upper case letters both enabled and disabled")
        }
        if self.digits && self.no_digits {
            return Err("Digits both enabled and disabled")
        }
        if self.special && self.no_special {
            return Err("Special characters both enabled and disabled")
        }
        if let Some(num) = self.num {
            if num < 1 {
                return Err("Too few passwords requested")
            }
        }
        if let Some(length) = self.length {
            if length < 1 {
                return Err("Too short a passwords requested")
            }
        }

        if let Some(replacement) = &self.replacement {
            if replacement.len() < 1 {
                return Err("Empty replacement special characters list")
            }
        }
        for extra in &self.extra {
            if extra.len() < 1 {
                return Err("Empty extra special characters list")
            }
        }
        let req = Requset{
            lower: self.lower || if self.no_lower { false } else { DEFAULTS.lower },
            upper: self.upper || if self.no_upper { false } else { DEFAULTS.upper },
            digits: self.digits || if self.no_digits { false } else { DEFAULTS.digits },
            special: self.combine_special(),
            all: self.all || DEFAULTS.all,
            num: if let Some(num) = self.num { num } else { DEFAULTS.num },
            quiet: self.quiet || DEFAULTS.quiet,
            length: if let Some(length) = self.length { length } else { DEFAULTS.length },
        };
        if !req.lower && !req.upper && !req.digits && req.special.is_none() {
            return Err("All character classes are off")
        }
        Ok(req)
    }

    fn combine_special(&self) -> Option<String> {
        // -S
        if self.no_special {
            return None
        }
        // neither -s, -r nor -x
        if !self.special && self.replacement.is_none() && self.extra.is_empty()  {
            return None
        }
        let special = 
            if let Some(replacement) = &self.replacement {
                replacement.clone()
            } else {
                DEFAULT_SPECIALS.to_owned()
            };
        let special = self.extra.iter().fold(
            special, 
            |acc, s| {
                acc + s
            },
        );
        let special = uniq(special);
        Some(special)
    }
}
