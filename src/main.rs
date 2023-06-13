use html_parser::Dom;
use serde::Deserialize;
use std::{env, fs};
use anyhow::{Result, anyhow};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Document {
    children: Vec<NodeType>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum NodeType {
    Element(Node),
    Text(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    children: Option<Vec<NodeType>>,
    classes: Option<Vec<String>>,
}

fn parse_html_file(path: String) -> Result<String> {
    let html = fs::read_to_string(path).map_err(|_| anyhow!("Could not read file"))?;

    let dom = Dom::parse(&html).map_err(|_| anyhow!("Could not parse HTML"))?;

    dom.to_json().map_err(|_| anyhow!("Could not convert DOM to JSON"))
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
                }
                NodeType::Text(_) => {}
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
            }
            NodeType::Text(_) => {}
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(anyhow!("No input file provided"));
    }

    let file_path = args[1].clone();
    let json = parse_html_file(file_path)?;
    let classnames = get_all_classnames_from_json(json);
    let css = generate_css(classnames);

    println!("{css}");

    Ok(())
}
