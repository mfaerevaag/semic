extern crate cmm;

fn main() {
    let mut errors = Vec::new();

    let res = cmm::parse(&mut errors, "int main ( void ) { int x; x = 7; }").unwrap();

    println!("{:?}", res);
}
