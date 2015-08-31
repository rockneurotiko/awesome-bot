#[cfg(test)]
mod test {
    use AwesomeBot;
    use regex::Regex;

    struct Defs {
        cmd: &'static str,
        res: String,
        usern: String,
    }

    impl Default for Defs {
        fn default() -> Defs {
            Defs {
                cmd: "test",
                res: String::from("^/test(?:@usernamebot)?$"),
                usern: String::from("usernamebot"),
            }
        }
    }

    macro_rules! create_test_string {
        (
            $name: ident,
            $cmd: expr
                ) => {
            #[test]
            fn $name() {
                let defs = Defs{cmd: $cmd, ..Default::default()};
                assert_eq!(AwesomeBot::modify_command(defs.cmd, &defs.usern), defs.res);
            }
        }
    }
    create_test_string!(mt_simple_right, "test");
    create_test_string!(mt_end_right, "test$");
    create_test_string!(mt_slash_right, "/test");
    create_test_string!(mt_slashend_right, "/test$");
    create_test_string!(mt_startslash_right, "^/test");
    create_test_string!(mt_startslashend_right, "^/test$");

    #[test]
    fn test_complex_command() {
        assert_eq!(AwesomeBot::modify_command("echo (.+)", "rock"), String::from("^/echo(?:@rock)? (.+)$"));
    }

    fn create_regex(cmd: &'static str) -> Regex {
        let defs = Defs{cmd: cmd, ..Default::default()};
        let cmd = AwesomeBot::modify_command(defs.cmd, &defs.usern);
        Regex::new(&*cmd).unwrap()
    }

    #[test]
    fn rcmd_simple_right() {
        let r = create_regex("test");
        assert!(r.is_match("/test"));
    }

    #[test]
    fn rcmd_withusername_right() {
        let r = create_regex("test");
        assert!(r.is_match("/test@usernamebot"));
    }

    #[test]
    fn rcmd_capturessimple_right() {
        let r = create_regex("test");
        let cap = r.captures("/test").unwrap();
        assert_eq!(cap.len(), 1);
        assert_eq!(cap.at(0), Some("/test"));
    }

    #[test]
    fn rcmd_capturesusername_right() {
        let r = create_regex("test");
        let cap = r.captures("/test@usernamebot").unwrap();
        assert_eq!(cap.len(), 1);
        assert_eq!(cap.at(0), Some("/test@usernamebot"));
    }
}
