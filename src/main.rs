use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let echoes = echo(&args[1..]);
    println!("{}", echoes);
}

/// Echo the input.
fn echo(strings: &[String]) -> String {
    String::from(strings.join(" ").as_str())
}

#[test]
fn one_word() {
    assert_eq!(echo(&[String::from("foo")]), "foo");
}

#[test]
fn two_words() {
    assert_eq!(echo(&[String::from("foo bar")]), "foo bar");
}

#[test]
fn multiple_words() {
    assert_eq!(
        echo(&[String::from("foo bar baz fkjdlshf098432whvds ofYq")]),
        "foo bar baz fkjdlshf098432whvds ofYq"
    );
}

#[test]
fn multiple_words_joined() {
    let strs = [
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("fkjdlshf098432whvds"),
        String::from("ofYq"),
    ];
    assert_eq!(strs.join(" "), "foo bar baz fkjdlshf098432whvds ofYq");
}

#[test]
fn one_char() {
    assert_eq!(echo(&[String::from("f")]), "f");
}

#[test]
fn numbers() {
    assert_eq!(echo(&[String::from("1")]), "1");
    assert_eq!(echo(&[String::from("2.5")]), "2.5");
    assert_eq!(echo(&[String::from("0x123")]), "0x123");
}

#[test]
fn non_latin() {
    assert_eq!(echo(&[String::from("đ")]), "đ");
    assert_eq!(echo(&[String::from("å")]), "å");
    assert_eq!(echo(&[String::from("scheiße")]), "scheiße");
}

// #[test]
// fn with_no_spaces() {
//     let input = vec!["foo", "bar"];
//     let expected = "foobar";
//
//     assert_eq!(echo(&input, None), expected);
// }
