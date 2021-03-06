use serde_json::to_string_pretty;
use winevents_xml_transform::{from_file, ToCEF};

fn main() {
    let eg6 = from_file("data/winevt6.xml");
    let out = eg6.unwrap();
    println!("Event Json String");
    println!("{}", to_string_pretty(&out).unwrap());
    println!("---------------");
    println!();
    println!("Event Object");
    println!("{:?}", out);
    println!("---------------");
    println!();
    println!("CEF String");
    println!("{}", out.to_cef())
}