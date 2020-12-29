use std::env::Args;

pub fn has_flag(flag: &str, mut args: Args) -> bool {
    let prefix = if flag.starts_with("-") {
        r#""#
    } else {
        if flag.len() == 1 {
            "-"
        } else {
            "--"
        }
    };

    let position = args.position(|x| format!("{}{}", prefix, flag) == x);

    let terminator_position = args.position(|x| "--" == x);

    match position {
        Some(p) => match terminator_position {
            Some(tp) => p < tp,
            None => true,
        },
        _ => false,
    }
}
