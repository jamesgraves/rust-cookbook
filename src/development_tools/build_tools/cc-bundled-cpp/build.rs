fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/multiply.cpp")
        .compile("multiply");
}
