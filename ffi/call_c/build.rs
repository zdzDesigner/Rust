extern crate cc;

fn main() {
    cc::Build::new().file("lib/sum.c").compile("libsum.a");
}
