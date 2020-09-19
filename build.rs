fn main() {
    cc::Build::new().file("src/cstuff.c").file("src/cstuff2.c").compile("cstuff");
}
