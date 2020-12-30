mod tests;

pub fn has_flag(flag: &str, args: Vec<String>) -> bool
{
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
    let position = args.into_iter().position(|x| x == &format!("{}{}", prefix, flag));
    let terminator_position = args.into_iter().position(|x| "--" == x);
    match position {
        Some(p) => match terminator_position {
            Some(tp) => p < tp,
            None => true,
        },
        _ => false,
    }
}
