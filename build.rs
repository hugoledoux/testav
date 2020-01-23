extern crate cc;

// hugo

fn main() {
    cc::Build::new()
        .file("src/predicates.c")
        .compile("libpredicates.a");
}
