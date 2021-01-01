use regex::Regex;

#[derive(Debug)]
struct A<'a> {
    value: &'a str,
}

#[derive(Debug)]
struct B<'a> {
    value: &'a str,
}

#[derive(Debug)]
struct C<'a> {
    value: &'a str,
}

#[derive(Debug)]
struct D<'a> {
    value: &'a str,
}

#[derive(Debug)]
struct Letters<'a> {
    a: A<'a>,
    b: B<'a>,
    c: C<'a>,
    d: D<'a>,
}

fn parse_them(dataset: &str) -> Vec<Letters> {
    let re = Regex::new(r"(\w).(\w).(\w).(\w)").unwrap();
    let mut letters: Vec<Letters> = Vec::new();
    for line in dataset.lines() {
        for cap in re.captures_iter(line) {
            let a = A {
                value: cap.get(1).unwrap().as_str()
            };
            let b = B {
                value: cap.get(2).unwrap().as_str()
            };
            let c = C {
                value: cap.get(3).unwrap().as_str()
            };
            let d = D {
                value: cap.get(4).unwrap().as_str()
            };            
            letters.push(Letters {a, b, c, d});
        }
    }
    letters
}

fn main() {
    let t = r#"a b c d
1 2 3 4
"#;
    let d = parse_them(&t);
    println!("{:?}", d);
}

