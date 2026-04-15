fn main() {
    cc::Build::new()
        .flag("-masm=intel")
        .opt_level(3)
        .file("src/gf5_248.S")
        .compile("gf5_248");
}
