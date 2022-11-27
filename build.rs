fn main() {
    cc::Build::new().file("src/main.s").compile("asm_main");
}
