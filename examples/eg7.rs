use serde_json::to_string_pretty;
use winevents_xml_transform::{from_file, ToCEF};

fn main() {
    let eg1 = from_file("data/winevt7.xml");
    let out = eg1.unwrap();
    // println!("Event Json String");
    // println!("{}", to_string_pretty(&out).unwrap());
    // println!("---------------");
    // println!();
    // println!("Event Object");
    println!("{:?}", out.to_cef());
}