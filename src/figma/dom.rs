use std::{collections::HashMap};

pub struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}
impl Node {
  // fn format_styles
  pub fn to_html(&self) -> String {
    match &self.node_type {
      NodeType::Text(text) => format!("{}", text),
      NodeType::Element(data) => {
        let mut attrs = String::from("");
        if let Some (attr) = &data.attributes {
          for (key, value) in attr.iter() {
            attrs += &format!("{}=\"{}\'", key, value);
          }
        }

        // let json_styles = serde_json::to_string(&data.attributes).unwrap();
        let mut styles = String::from("");
        if let Some (data_styles) = &data.styles {
          for (key, value) in data_styles.iter() {
            styles += &format!("{}: {};", key, value);
          }
        }
        

        let mut children_html = String::new();
        for child in self.children.iter() {
          children_html += &child.to_html();
        }
      
        format!("<{tag} {}style=\"{}\">{}</{tag}>", attrs, styles, children_html, tag = data.tag_name)
      }
    }
  }
}

pub enum NodeType {
  Text(String),
  Element(ElementData),
}

pub struct ElementData { 
  pub tag_name: String,
  pub styles: Option<StyleMap>,
  pub attributes: Option<AttrMap>,
}

type AttrMap = HashMap<String, String>;
type StyleMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
  Node { 
    children: Vec::new(), 
    node_type: NodeType::Text(data) 
  }
}

pub fn elem(name: String, attrs: Option<AttrMap>, styles: Option<StyleMap>, children: Vec<Node>) -> Node {
  Node { 
    children,
    node_type: NodeType::Element(ElementData {
      styles,
      tag_name: name,
      attributes: attrs,
    }),
  }
}
