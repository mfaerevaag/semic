extern crate cmm;

fn main() {
    let prog = r#"

int main(void)
{
    char c, s[2];
    c = 'a';
    s = "a";
    return (s == "foo");
}

"#;

    match cmm::run(prog) {
        Ok(()) => println!("returned success"),
        Err(()) => println!("returned error"),
    }
}
