use std::{fs, env};
use html_parser::Dom;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Document {
    children: Vec<NodeType>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum NodeType {
    Element(Node),
    Text(String)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    children: Option<Vec<NodeType>>,
    classes: Option<Vec<String>>,
}

fn parse_html_file(path: String) -> Result<String, &'static str> {
    let html = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            println!("{}", err);
            return Err("Could not read file")
        }
    };

    // parse the string into a DOM
    let dom = match Dom::parse(&html) {
        Ok(dom) => dom,
        Err(_) => return Err("Could not parse HTML"),
    };

    match dom.to_json() {
        Ok(json) => Ok(json),
        Err(_) => Err("Could not convert DOM to JSON"),
    }
}

fn get_classnames_from_node(node: Node) -> Vec<String> {
    let mut classnames: Vec<String> = Vec::new();

    if let Some(classes) = node.classes {
        for class in classes {
            classnames.push(class);
        }
    }

    if let Some(children) = node.children {
        for child in children {
            match child {
                NodeType::Element(node) => {
                    let mut child_classnames = get_classnames_from_node(node);
                    classnames.append(&mut child_classnames);
                },
                NodeType::Text(_) => {},
            }
        }
    }

    classnames
}

fn unique(vec: Vec<String>) -> Vec<String> {
    let mut unique_vec: Vec<String> = Vec::new();

    for item in vec {
        if !unique_vec.contains(&item) {
            unique_vec.push(item);
        }
    }

    unique_vec
}

fn get_all_classnames_from_json(json: String) -> Vec<String> {
    let document: Document = serde_json::from_str(&json).unwrap();

    let mut classnames: Vec<String> = Vec::new();

    for child in document.children {
        match child {
            NodeType::Element(node) => {
                let mut child_classnames = get_classnames_from_node(node);
                classnames.append(&mut child_classnames);
            },
            NodeType::Text(_) => {},
        }
    }

    unique(classnames)
}

fn generate_css(classnames: Vec<String>) -> String {
    let mut css = String::new();

    for classname in classnames {
        css.push_str(&format!(".{} {{\n}}\n", classname));
    }

    css
}

fn main() {
    let mut args = env::args();
    args.next();

    let file_path = match args.next() {
        Some(path) => path,
        None => {
            println!("No input file provided");
            return;
        }
    };

    let json = match parse_html_file(file_path) {
        Ok(json) => json,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let classnames = get_all_classnames_from_json(json);

    let css = generate_css(classnames);

    println!("{css}");
}
