/// Repeat whatever is put in.
pub fn echo<T: ?Sized>(s: &T) -> &T {
    // TODO change so that it returns all elements as one concatenated String
    s
}

/// Run the application.
pub fn run(vec_str: &[String]) {
    let echoes = echo(vec_str);
    println!("{:?}", echoes);
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
