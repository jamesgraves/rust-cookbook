fn main() {
    cc::Build::new()
        .define("APP_NAME", "\"a cookbook demo\"")
        .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
        .define("WELCOME", None)
        .file("src/welcome.c")
        .compile("welcome");
}
