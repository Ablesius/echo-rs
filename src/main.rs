use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let echoes = {
        // this is a bit ugly to have a double negative, but we're just cloning echo's -s
        if !cli.separate {
            echo(cli.strings)
        } else {
            unseparated_echo(cli.strings)
        }
    };
    println!("{}", echoes);
}

#[derive(Parser)]
struct Cli {
    /// Do not separate output with spaces. Defaults to false.
    #[arg(short, action)]
    separate: bool,

    /// The string to echo
    #[arg(trailing_var_arg = true)]
    strings: Vec<String>,
}

/// Echo the input.
fn echo(strings: Vec<String>) -> String {
    strings.join(" ")
}

/// Echo the input but do not separate with a space.
fn unseparated_echo(strings: Vec<String>) -> String {
    strings.join("")
}

#[test]
fn one_word() {
    assert_eq!(echo(vec!(String::from("foo"))), "foo");
}

#[test]
fn two_words() {
    assert_eq!(
        echo(vec!(String::from("foo"), String::from("bar"),)),
        "foo bar"
    );
}

#[test]
fn multiple_words() {
    assert_eq!(
        echo(vec!(
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
            String::from("fkjdlshf098432whvds"),
            String::from("ofYq")
        )),
        "foo bar baz fkjdlshf098432whvds ofYq"
    );
}

#[test]
fn multiple_words_joined_with_separator() {
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
    assert_eq!(echo(vec!(String::from("f"))), "f");
}

#[test]
fn numbers() {
    assert_eq!(echo(vec!(String::from("1"))), "1");
    assert_eq!(echo(vec!(String::from("2.5"))), "2.5");
    assert_eq!(echo(vec!(String::from("0x123"))), "0x123");
}

#[test]
fn non_latin() {
    assert_eq!(echo(vec!(String::from("đ"))), "đ");
    assert_eq!(echo(vec!(String::from("å"))), "å");
    assert_eq!(echo(vec!(String::from("scheiße"))), "scheiße");
}

#[test]
fn with_no_spaces() {
    let input = vec![String::from("foo"), String::from("bar")];
    let expected = "foobar";
    assert_eq!(unseparated_echo(input), expected)
}
