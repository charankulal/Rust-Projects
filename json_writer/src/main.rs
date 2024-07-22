use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
    name:String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>
}

fn main() {
    let article:Article=Article{
        article:String::from("How to work with json in RUST"),
        author:String::from("akhil"),
        paragraph:vec![
            Paragraph{name:String::from("Introduction")},
            Paragraph{name:String::from("Conclusion")},
            ]
    };
    
    let json=serde_json::to_string(&article).unwrap();
    println!("The JSON is {}",json);
}
