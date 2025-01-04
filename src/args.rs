use clap::{
    command, 
    arg, 
    value_parser, 
    ArgAction, 
    ArgMatches, 
    builder::styling::Styles, 
    Error,
    error::ErrorKind, 
};
use clap_cargo::style::NOP;
use iwrtb::uniq;

const DEFAULT_LOWER: bool = true;
const DEFAULT_UPPER: bool = true;
const DEFAULT_DIGITS: bool = true;
const DEFAULT_SPECIALS: &str = "!$%@#";
const DEFAULT_ALL: bool = false;
const DEFAULT_NUM: &str = "1";
const DEFAULT_QUIET: bool = false;
const DEFAULT_LENGTH: &str = "16";

macro_rules! no_empty_strings {
    ($m_str:expr) => {
        |s: &str|
            if s.len() > 0
                { Ok(s.to_owned()) }
            else
                { Err($m_str)}
    };
}

pub struct Requset {
    pub lower: bool,
    pub upper: bool,
    pub digits: bool,
    pub special: Option<String>,
    pub all: bool,
    pub num: usize,
    pub quiet: bool, // TODO: Implement verbose output
    pub length: usize,
    pub err: Error,
}

impl Requset {
    fn get_dual_bool_match(a_matches: &ArgMatches, a_id: &str, a_default: &bool) -> bool {
        *a_matches.get_one::<bool>(a_id).unwrap() 
        || if *a_matches.get_one::<bool>(&format!("no_{a_id}")).unwrap() 
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
            let mut combined = String::from(DEFAULT_SPECIALS);
            combined.extend(
                    extra.map(
                        |s| 
                            s.chars())
                .flatten());
            Some(uniq(&combined))
        } else if *a_matches.get_one::<bool>("special").unwrap() {
            Some(DEFAULT_SPECIALS.to_owned())
        } else {
            None
        }
    }
    
    pub fn new() -> Self {
        let mut cmd = command!()
            // TODO: Report bug
            // .name(
            //     env!("CARGO_BIN_NAME")) // Workaround for render_help bug
            .disable_version_flag(true)
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
            .arg(arg!(num: -n <num> "Generate number of passwords")
                .default_value(DEFAULT_NUM)
                .value_parser(value_parser!(u8)
                .range(1..128)))
            .arg(arg!(quiet: -q "Quiet output, doesn't affect anithing as of yet") // Do not output anything but the password(s)
                .action(ArgAction::SetTrue))
            .arg(arg!(length: <length> "Password length")
                .default_value(DEFAULT_LENGTH)
                .required(false)
                .value_parser(value_parser!(u8)
                .range(1..128)))
            .styles(Styles::default().clone()
                .error(NOP)
                .invalid(NOP)
                .literal(NOP)
                .placeholder(NOP)
                .usage(NOP)
                .valid(NOP)
                .header(NOP))
            .help_template("\
                {before-help}\
                {usage-heading}{usage}\n\n\
                {about}\n\n\
                {all-args}{after-help}")
            .override_usage(format!(
                "\n  {} [-h] [-l | -L] [-u | -U] [-d | -D] [-s | -S | -r <chars> | -x <chars>] [-a] [-n <num>] [-q] [length]",
                env!("CARGO_BIN_NAME"),
                ))
            ;
        let err = cmd.error(ErrorKind::ArgumentConflict, "{}");
        let matches = cmd.get_matches();
        let req = Requset{
            lower: Self::get_dual_bool_match(&matches, "lower", &DEFAULT_LOWER),
            upper: Self::get_dual_bool_match(&matches, "upper", &DEFAULT_UPPER),
            digits: Self::get_dual_bool_match(&matches, "digits", &DEFAULT_DIGITS),
            special: Self::combine_special(&matches),
            all: Self::get_single_match(&matches, "all", &DEFAULT_ALL),
            num: usize::from(Self::get_single_match::<u8>(
                &matches, "num",
                &DEFAULT_NUM.parse::<u8>().unwrap())),
            quiet: Self::get_single_match(&matches, "quiet", &DEFAULT_QUIET),
            length: usize::from(Self::get_single_match::<u8>(
                &matches, "length",
                &DEFAULT_LENGTH.parse::<u8>().unwrap())),
            err: err,
        };
        if !req.lower && !req.upper && !req.digits && req.special.is_none() {
            req.brag_and_exit("All character classes are off");
        }
        req
    }

    pub fn brag_and_exit(&self, a_msg: &str) {
        eprint!("{}", self.err.render().to_string().replace("{}", a_msg));
        std::process::exit(1);
    }
}
