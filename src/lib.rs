/// Check if argv has a specific flag
/// Correctly stops looking after an -- argument terminator.
/// ## Usage
/// command input
/// ```bash
/// ls -f --unicorn --foo=bar -- --rainbow
/// ```
///#[test]
///#[ignore]
/// ```edition2018
/// use has_flag::has_flag;
/// use std::env;
/// 
/// has_flag("unicorn", env::args().into_iter().collect());
/// //=> true
/// 
/// has_flag ("--unicorn", env::args().into_iter().collect());
/// //=> true
/// 
/// has_flag("f", env::args().into_iter().collect());
/// //=> true
/// 
/// has_flag("-f", env::args().into_iter().collect());
/// //=> true
/// 
/// has_flag("foo=bar", env::args().into_iter().collect());
/// //=> true
/// 
/// has_flag("foo", env::args().into_iter().collect());
/// //=> false
/// 
/// has_flag("rainbow", env::args().into_iter().collect());
/// //=> false
/// ```
pub fn has_flag(flag: &str, args: Vec<String>) -> bool {
    let prefix = if flag.starts_with("-") {
        ""
    } else {
        if flag.len() == 1 {
            "-"
        } else {
            "--"
        }
    };
    let args = &args;
    let position = args
        .into_iter()
        .position(|x| x == &format!("{}{}", prefix, flag));
    let terminator_position = args.into_iter().position(|x| "--" == x);
    match position {
        Some(p) => match terminator_position {
            Some(tp) => p < tp,
            None => true,
        },
        _ => false,
    }
}



