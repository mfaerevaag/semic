extern crate cmm;

fn main() {
    let prog = r#"

int main(void)
{
    char s[2];
    s = "a";
    return (s == "foo");
}

"#;

    match cmm::run(prog) {
        Ok(ret) => println!("returned {:?}", ret),
        Err(()) => println!("returned error"),
    }
}
