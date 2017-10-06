extern crate cmm;

fn main() {
    let mut errors = Vec::new();

    let res = cmm::parse(&mut errors, r#"

int double(int a);

int main (void)
{
    int x;
    x = double(1 + 2);
    return x;
}

int double(int a)
{
    return a * 2;
}

"#).unwrap();

    println!("{:?}", res);
}

