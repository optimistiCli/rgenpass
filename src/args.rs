// use argh::{FromArgs, TopLevelCommand};

use std::i32;

use clap::{arg, command, value_parser, ArgAction, ArgMatches};
use iwrtb::uniq;

pub struct Requset {
    pub lower: bool,
    pub upper: bool,
    pub digits: bool,
    pub special: Option<String>,
    pub all: bool,
    pub num: i32,
    pub quiet: bool,
    pub length: i32,
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

macro_rules! no_empty_strings {
    ($m_str:expr) => {
        |s: &str|
            if s.len() > 0
                { Ok(s.to_owned()) }
            else
                { Err($m_str)}
    };
}

fn get_dual_bool_match(a_matches: &ArgMatches, a_id: &str, a_default: &bool) -> bool {
    *a_matches.get_one::<bool>(a_id).unwrap() 
    || if *a_matches.get_one::<bool>(a_id).unwrap() 
        { false } 
    else 
        { *a_default }
}

fn get_single_match<T: Clone + Send + Sync + 'static>(
        a_matches: &ArgMatches, 
        a_id: &str, 
        a_default: &T,
    ) -> T {
    if let Some(num) = a_matches.get_one::<T>(a_id)
        { num.clone() } 
    else 
        { a_default.clone() }
}

fn combine_special(a_matches: &ArgMatches) -> Option<String> {
    if *a_matches.get_one::<bool>("no_special").unwrap() {
        None
    } else if let Some(replacement) = a_matches.get_one::<String>("replacement") {
        Some(uniq(replacement))
    } else if let Some(extra) = a_matches.get_many::<String>("extra") {
        Some(uniq(&extra
            .map(|s| s.chars())
            .flatten()
            .collect::<String>()))
    } else if *a_matches.get_one::<bool>("special").unwrap() {
        Some(DEFAULT_SPECIALS.to_owned())
    } else {
        None
    }
}

pub fn parse_args() -> Requset {
    let matches = command!()
        .arg(arg!(lower: -l "Use lower case letters (default)")
            .action(ArgAction::SetTrue)
            .group("low"))
        .arg(arg!(no_lower: -L "Do NOT use lower case letters")
            .action(ArgAction::SetTrue)
            .group("low"))
        .arg(arg!(upper: -u "Use upper case letters (default)")
            .action(ArgAction::SetTrue)
            .group("up"))
        .arg(arg!(no_upper: -U "Do NOT use upper case letters")
            .action(ArgAction::SetTrue)
            .group("up"))
        .arg(arg!(digits: -d "Use digits (default)")
            .action(ArgAction::SetTrue)
            .group("dig"))
        .arg(arg!(no_digits: -D "Do NOT use digits")
            .action(ArgAction::SetTrue)
            .group("dig"))
        .arg(arg!(special: -s "Use special characters !$%@#")
            .action(ArgAction::SetTrue)
            .group("sp1"))
        .arg(arg!(no_special: -S "Do NOT use special characters")
            .action(ArgAction::SetTrue)
            .groups(["sp1", "sp2"]))
        .arg(arg!(extra: -x <chars> ... "Extra special characters, implies -s")
            .value_parser(no_empty_strings!("Empty extra special characters list"))
            .group("sp2"))
        .arg(arg!(replacement: -r <chars> "Replacement special characters, implies -s")
            .value_parser(no_empty_strings!("Empty replacement special characters list"))
            .group("sp2"))
        .arg(arg!(all: -a "Password must conain chars of all classes")
            .action(ArgAction::SetTrue))
        .arg(arg!(num: -n <num> "Generate number of passwords (default is {DEFAULTS.num})")
            .default_value("{DEFAULTS.num}")
            .value_parser(value_parser!(i32)
            .range(1..128)))
        .arg(arg!(quiet: -q "Do not output anything but the password(s)")
            .action(ArgAction::SetTrue))
        .arg(arg!(length: <length> "Password length (default is {DEFAULTS.length})")
            .default_value("{DEFAULTS.length}")
            .value_parser(value_parser!(i32)
            .range(1..128)))
        // .help_template("\
        //     {before-help}{about}\n\n\
        //     {usage-heading}\n  {bin} {usage}\n\n\
        //     {all-args}{after-help}")
        // .override_usage("[-l | -L]")
        .get_matches();

    let req = Requset{
        lower: get_dual_bool_match(&matches, "lower", &DEFAULTS.lower),
        upper: get_dual_bool_match(&matches, "upper", &DEFAULTS.upper),
        digits: get_dual_bool_match(&matches, "digits", &DEFAULTS.digits),
        special: combine_special(&matches),
        all: get_single_match(&matches, "all", &DEFAULTS.all),
        num: get_single_match(&matches, "num", &DEFAULTS.num),
        quiet: get_single_match(&matches, "quiet", &DEFAULTS.quiet),
        length: get_single_match(&matches, "length", &DEFAULTS.length),
    };
    if !req.lower && !req.upper && !req.digits && req.special.is_none() {
        todo!("All character classes are off")
    }
    req
}
