extern crate regex;
extern crate reqwest;
extern crate image_base64;

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
        let filename = path::Path::new(md_path.clone().as_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let out_name = format!(
            "{}__out.md",
            filename
                .split(".md")
                .collect::<Vec<&str>>()[0]
        );
        let out_path = path::Path::new(&md_path)
            .join("../")
            .canonicalize()
            .unwrap()
            .join(&out_name)
            .to_str()
            .unwrap()
            .to_string();
//        println!("{}-{}", out_name, out_path);
        let content = read_md(md_path);
        let out = convert(content);
        write_md(out, out_path);
    }

    println!("✨ Done!");
}

fn read_md(path: String) -> String {
    let mut f = fs::File::open(&path)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(format!("⛔ Something went wrong reading file: {}", &path).as_str());
    contents
}

fn convert(mut content: String) -> String {
    let re = Regex::new(r"(?x)!\[(?P<desc>.*)]\s*\((?P<url>https?://.+)\)").unwrap();
    while let Some(m) = &re.find(&content.clone()) {
        let url = &re.replace(m.as_str(), "$url").clone().to_string();
        let desc = &re.replace(m.as_str(), "desc").clone().to_string();
        let filename = url
            .split("/")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap();
        let md_img = format!("![{}]({})",
             desc,
             url_to_base64(&url, &filename)
        );

        let start = &content.clone()[..m.start()];
        let end = &content.clone()[m.end()..];
        content = format!("{}{}{}", start, md_img, end);
    }
    content.to_string()
}

fn url_to_base64(url: &str, filename: &str) -> String {
    let mut resp = reqwest::get(url).expect("⛔ Request failed");
    let mut out = fs::File::create(filename).expect("⛔ Failed to create file");
    io::copy(&mut resp, &mut out).expect("⛔ Failed to copy content");
    let out: String = image_base64::to_base64(filename);
    fs::remove_file(filename);
    out
}

fn write_md(content: String, filename: String) {
    let mut file = match fs::File::create(&filename) {
        Err(why) => panic!(
            "⛔ Couldn't create {}: {:?}",
            &filename,
            why
        ),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!(
                "⛔ Couldn't write to {}: {:?}",
                &filename,
                why
            )
        },
        Ok(_) => println!("✨ Successfully wrote to {}", &filename),
    }
}
