use regex::Regex;

#[macro_use]
extern crate lazy_static;

// define partial regexes
const IRI: &str = "(?:<([^:]+:[^>]*)>)";
const PLAIN: &str = "\"([^\"\\\\]*(?:\\\\.[^\"\\\\]*)*)\"";
const LANGUAGE: &str = "(?:@([a-zA-Z]+(?:-[a-zA-Z0-9]+)*))";
const WS: &str = "[ \\t]+";
const WSO: &str = "[ \\t]*";

lazy_static! {
    // https://www.w3.org/TR/turtle/#grammar-production-BLANK_NODE_LABEL
    static ref PN_CHARS_BASE: String = format!(
      "{}{}{}{}{}{}{}{}{}{}{}{}{}",
      "A-Z",
      "a-z",
      "\u{00C0}-\u{00D6}",
      "\u{00D8}-\u{00F6}",
      "\u{00F8}-\u{02FF}",
      "\u{0370}-\u{037D}",
      "\u{037F}-\u{1FFF}",
      "\u{200C}-\u{200D}",
      "\u{2070}-\u{218F}",
      "\u{2C00}-\u{2FEF}",
      "\u{3001}-\u{D7FF}",
      "\u{F900}-\u{FDCF}",
      "\u{FDF0}-\u{FFFD}"
      // TODO:
      // "\u{1000}0-\u{EFFF}F"
    );
    static ref PN_CHARS_U: String = format!(
      "{}{}",
      PN_CHARS_BASE.as_str(),
      "_"
    );
    static ref PN_CHARS: String = format!(
      "{}{}{}{}{}{}",
      PN_CHARS_U.as_str(),
      "0-9",
      "-",
      "\u{00B7}",
      "\u{0300}-\u{036F}",
      "\u{203F}-\u{2040}"
    );
    // define partial regexes
    static ref BLANK_NODE_LABEL: String = format!(
      "{}{}{}{}{}{}{}{}{}{}",
      "(_:",
        "(?:[", PN_CHARS_U.as_str(), "0-9])",
        "(?:(?:[" , PN_CHARS.as_str() , ".])*(?:[" , PN_CHARS.as_str() , "]))?",
      ")"
    );
    static ref BNODE: String = BLANK_NODE_LABEL.clone();
    static ref DATATYPE: String = format!("{}{}{}", "(?:\\^\\^", IRI, ")");
    static ref LITERAL: String = format!("(?:{}(?:{}|{})?)", PLAIN, DATATYPE.as_str(), LANGUAGE);
  
    // define quad part regexes
    static ref SUBJECT: String = format!("(?:{}|{})", BNODE.as_str(), IRI);
    static ref PROPERTY: String = format!("{}", IRI);
    static ref OBJECT: String = format!("(?:{}|{}|{})", IRI, BNODE.as_str(), LITERAL.as_str());
    static ref GRAPH: String = format!("(?:\\.|(?:(?:{}|{})))", IRI, BNODE.as_str());
  
    // full quad regex
    static ref QUAD: String = format!(
        "^{}{}{}{}{}{}$",
        WSO,
        SUBJECT.as_str(),
        PROPERTY.as_str(),
        OBJECT.as_str(),
        GRAPH.as_str(),
        WSO
    );
  
    static ref FIRST: String = format!("^{}{}", WSO, SUBJECT.as_str());
    static ref SUBJECT_REGEX: Regex = Regex::new(&FIRST).unwrap();

    static ref SECOND: String = format!("{}", SUBJECT.as_str());
    static ref PROPERTY_REGEX: Regex = Regex::new(&SECOND).unwrap();

    static ref THIRD: String = format!("{}", OBJECT.as_str());
    static ref OBJECT_REGEX: Regex = Regex::new(&THIRD).unwrap();

    static ref FOURTH: String = format!("{}", GRAPH.as_str());
    static ref GRAPH_REGEX: Regex = Regex::new(&FOURTH).unwrap();    
  
    static ref QUAD_REGEX: Regex = Regex::new(&QUAD).unwrap();
  }

  // pub fn parse_quads(target: &str) -> regex::Captures {

  pub fn parse_quads(target: &str) {
    let t: Vec<&str> = target.split(' ').collect();
    // let s = t[0];
    // let t = t[1];
    // let u = t[2 as usize];

    for (pos, p) in t.iter().enumerate() {
      match pos {
        0 => SUBJECT_REGEX.captures(p),
        1 => PROPERTY_REGEX.captures(p),
        2 => None,
        // 2 => OBJECT_REGEX.captures(p),
        3 => GRAPH_REGEX.captures(p),        
        _ => None
      };
      // println!("TTTTTTTt {}", p);
    }


    // let w = SUBJECT_REGEX.captures(s);
    // let x = PROPERTY_REGEX.captures(t);
    // match y {
    //   Some(v) => println!("xxx {}", v.len()),
    //   None => {}
    // }
    // QUAD_REGEX.captures(target).unwrap()
  }