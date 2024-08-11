mod tests {
    use echo_rs::run;

    #[test]
    fn one_word() {
        assert_eq!(run(&[String::from("foo")]), ());
    }

    #[test]
    fn multiple_words() {
        assert_eq!(
            run(&[String::from("foo bar baz fkjdlshf098432whvds ofYq")]),
            ()
        );
    }

    #[test]
    fn multiple_word_vec() {
        let strs = [
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
            String::from("fkjdlshf098432whvds"),
            String::from("ofYq"),
        ];
        assert_eq!(run(&strs[..],), ());
    }
}
