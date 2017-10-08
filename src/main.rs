extern crate cmm;

fn main() {
    let mut errors = Vec::new();

    let res = cmm::parse(&mut errors, r#"

int main (void)
{
    int x[2];
    return 1 + x[1];
}

"#).unwrap();

    println!("{:?}", res);
}

