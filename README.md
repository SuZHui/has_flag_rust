# has-flag-rust
> Check if args has a specific flag.

**IMPORTANT**: This crate is base on [has-flag](https://github.com/sindresorhus/has-flag#readme), Thanks [author](https://github.com/sindresorhus) for his efforts

Correctly stops looking after an -- argument terminator.
 ## Usage

 command input
 ```bash
 ls -f --unicorn --foo=bar -- --rainbow
 ```

 ```rust
 use has_flag::has_flag;
 use env::args;
 
 has_flag("unicorn", args().collect());
 //=> true
 
 has_flag ("--unicorn", args().collect());
 //=> true
 
 has_flag("f", args().collect());
 //=> true
 
 has_flag("-f", args().collect());
 //=> true
 
 has_flag("foo=bar", env::args().collect());
 //=> true
 
 has_flag("foo", env::args().collect());
 //=> false
 
 has_flag("rainbow", env::args().collect());
 //=> false
 ```
