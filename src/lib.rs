/// Repeat whatever is put in.
pub fn echo<T: ?Sized>(s: &T) -> &T {
    // TODO change so that it returns all elements as one concatenated String
    s
}

/// Run the application.
pub fn run(s: &str) {
    let echoes = echo(s);
    println!("{}", echoes);
}

#[test]
fn one_word() {
    assert_eq!(echo("foo"), "foo");
}

#[test]
fn two_words() {
    assert_eq!(echo("foo bar"), "foo bar");
}

#[test]
fn multiple_words() {
    assert_eq!(
        echo("foo bar baz fkjdlshf098432whvds ofYq"),
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
    assert_eq!(echo("f"), "f");
}

#[test]
fn numbers() {
    assert_eq!(echo("1"), "1");
    assert_eq!(echo("2.5"), "2.5");
    assert_eq!(echo("0x123"), "0x123");
}

#[test]
fn non_latin() {
    assert_eq!(echo("đ"), "đ");
    assert_eq!(echo("å"), "å");
    assert_eq!(echo("scheiße"), "scheiße");
}
