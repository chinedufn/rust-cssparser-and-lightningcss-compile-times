fn main() {
    #[cfg(feature = "with-rust-cssparser")]
    run_cssparser();
    #[cfg(feature = "with-lightningcss")]
    run_lightningcss();
}

#[cfg(feature = "with-rust-cssparser")]
fn run_cssparser() {
    use cssparser::{Parser, ParserInput};

    let mut input = ParserInput::new("body { color: red; }");
    let mut parser = Parser::new(&mut input);

    for token in parser.next() {
        dbg!(&token);
    }
}

#[cfg(feature = "with-lightningcss")]
fn run_lightningcss() {
    use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
    let sheet = StyleSheet::parse("body { color: red; }", ParserOptions::default()).unwrap();
    println!("{}", sheet.to_css(PrinterOptions::default()).unwrap().code);
}
