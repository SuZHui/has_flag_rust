#[cfg(test)]
mod tests {
    fn _c_iter(args: Vec<&str>) -> Vec<String> {
        args.into_iter().map(|a| a.to_string()).collect()
    }
    #[test]
    fn default_prefix() {
        assert!(has_flag::has_flag(
            "unicorn",
            _c_iter(vec!["--foo", "--unicorn", "--bar"])
        ));
    }

    #[test]
    fn optional_prefix() {
        assert!(has_flag::has_flag(
            "--unicorn",
            _c_iter(vec!["--foo", "--unicorn", "--bar"])
        ))
    }

    #[test]
    fn contain_equal_sign() {
      assert!(has_flag::has_flag(
        "unicorn=rainbow",
        _c_iter(vec!["--foo", "--unicorn=rainbow", "--bar"])
      ))
    }

    #[test]
    fn contain_empty() {
      assert!(has_flag::has_flag(
        "unicorn",
        _c_iter(vec!["--unicorn", "--", "--foo"])
      ))
    }

    #[test]
    fn don_not_match_flags_after_terminator() {
      assert!(!has_flag::has_flag(
        "unicorn",
        _c_iter(vec!["--foo", "--", "--unicorn"])
      ))
    }

    #[test]
    fn not_contain() {
      assert!(!has_flag::has_flag(
        "unicorn",
        _c_iter(vec!["--foo"])
      ))
    }

    #[test]
    fn default_short_prefix() {
      assert!(has_flag::has_flag(
        "-u",
        _c_iter(vec!["-f", "-u", "-b"])
      ))
    }

    #[test]
    fn short_contain_empty() {
      assert!(has_flag::has_flag(
        "-u",
        _c_iter(vec!["-u", "--", "-f"])
      ))
    }

    #[test]
    fn short_without_prefix() {
      assert!(has_flag::has_flag(
        "u",
        _c_iter(vec!["-f", "-u", "-b"])
      ))
    }

    #[test]
    fn short_and_empty () {
      assert!(has_flag::has_flag(
        "u",
        _c_iter(vec!["-u", "--", "-f"])
      ))
    }

    #[test]
    fn short_in_last() {
      assert!(!has_flag::has_flag(
        "f",
        _c_iter(vec!["-u", "--", "-f"])
      ))
    }
}
