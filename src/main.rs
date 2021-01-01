use regex::Regex;


fn main() {
    let t = r#"a b c
1 2 3
"#;

    let re = Regex::new(r"\S+").unwrap();
    let mut y: Vec<&str> = Vec::new();
    for line in t.lines() {
        for c in re.captures_iter(line) {
            // println!("X {:?}", c);
            y.push(c.get(0).unwrap().as_str());
        }

    }
    println!("{:?}", y);
}

