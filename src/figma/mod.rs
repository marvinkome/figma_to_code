pub mod types;
pub mod dom;

use std::collections::HashMap;
use types::{File, Node};

fn parser(node: &Node) -> (Option<String>, HashMap<String, String>) {
  let elem = match node.node_type {
    types::NodeType::Frame => Some("div".to_string()),
    types::NodeType::Rectangle => Some("div".to_string()),
    types::NodeType::Text => Some("p".to_string()),
    _ => None
  };

  // parse styling
  let mut styles = HashMap::<String, String>::new();
  if let Some(fills) = &node.fills {
    let paint = fills.get(0);
    let color = paint.unwrap().color.unwrap();

    styles.insert(String::from("background-color"), color.to_string());
  }

  if let Some(bounding_box) = node.absolute_bounding_box {
    styles.insert(String::from("width"), format!("{}px", bounding_box.width).to_string());
    styles.insert(String::from("height"), format!("{}px", bounding_box.height).to_string());
    styles.insert(String::from("position"), "relative".to_string());
  }

  (elem, styles)
}

fn parse_frame(frame: Node) {
  // generate background element from root frame
  let (elem, styles) = parser(&frame);
  
  let root_node = if elem.is_some()  {
    Some(dom::elem(elem.unwrap(), None, Some(styles), Vec::new()))
  } else {
    None
  };

  println!("{}", root_node.unwrap().to_html());
}

pub fn run(file: File) -> Option<String> {
  // parse document
  let document = file.document;

  // get first page in document
  let canvas = document.children?;
  let page_children = &canvas.get(0)?.children;

  // get first frame in document
  let frames = match page_children {
      Some(children) => {
        children.to_owned().into_iter().filter(|c| c.node_type == types::NodeType::Frame)
      }
      None => return None, 
  }.collect::<Vec<_>>();
  let frame = frames.get(0)?.to_owned();

  parse_frame(frame);

  return Some("".to_string());
}