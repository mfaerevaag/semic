extern crate cmm;

fn main() {
    let mut errors = Vec::new();

    let res = cmm::parse(&mut errors, "int main (void) { return 0; }").unwrap();

    println!("{:?}", res);
}
