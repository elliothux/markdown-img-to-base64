extern crate regex;
extern crate reqwest;

use std::{env, path, fs, io};
use std::io::prelude::*;
use regex::Regex;


fn main() {
    let args = &env::args().into_iter().collect::<Vec<String>>()[1..];
    if args.len() == 0 {
        println!("⛔️ No argument found!");
        return;
    }

    for arg in args {
        let dir_path = env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let md_path = path::Path::new(&dir_path)
            .join(arg)
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let content = read_md(md_path);
        println!("{}", convert(content));
    }

    println!("✨ Done!");
}

fn read_md(path: String) -> String {
    let mut f = fs::File::open(path)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn convert(_content: String) -> String {
    let re = Regex::new(r"(?x)!\[(?P<desc>.*)]\s*\((?P<url>https?://.+)\)").unwrap();
    let mut content = "##hello\n![111](https://xx.yy)\n#d".to_string();
    while let Some(m) = &re.find(&content.clone()) {
        let url = &re.replace(m.as_str(), "$url");
        let desc = &re.replace(m.as_str(), "desc");
        let filename = url.clone().split("/").collect::<Vec<str>>().pop();
        let md_img = format!("![{}](data:image/png;base64,{})",
             desc.clone(),
             url_to_base64(url.clone().to_string())
        );

        let start = &content.clone()[..m.start()];
        let end = &content.clone()[m.end()..];
        content = format!("{}{}{}", start, md_img, end);
    }
    content.to_string()
}

fn url_to_base64(url: String, filename: String) -> String {
    let mut resp = reqwest::get(url).expect("request failed");
    let mut out = fs::File::create(filename).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    image_base64::to_base64(filename)
}
