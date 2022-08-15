fn main() {
    cc::Build::new()
        .file("add.c")
        .compile("add");
}
