use open;

fn main() {
    let url = String::from("http://rust-lang.org");
    open::that(url).unwrap();
}
