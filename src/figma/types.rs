use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
  pub name: String,
  pub last_modified: String,
  pub thumbnail_url: String,
  pub version: String,
  pub role: String,
  pub editor_type: String,
  pub link_access: String,
  pub document: Node
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
  #[serde(rename = "type")]
  pub node_type: NodeType,

  pub id: String,
  pub name: String,
  pub children: Option<Vec<Node>>,
  pub background_color: Option<Color>,
  pub fills: Option<Vec<Paint>>,
  pub strokes: Option<Vec<Paint>>,
  pub stroke_weight: Option<f32>,
  pub stroke_align: Option<StrokeAlign>,
  pub corner_radius: Option<f32>,
  pub rectangle_corner_radii: Option<Vec<f32>>,
  pub absolute_bounding_box: Option<Rectangle>
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
  Document,
  Canvas,
  Frame,
  Text,
  Group,
  Vector,
  BooleanOperation,
  Star,
  Line,
  Ellipse,
  RegularPolygon,
  Rectangle,
  Slice,
  Component,
  ComponentSet,
  Instance,
  Sticky,
  ShapeWithText,
  Connector
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrokeAlign {
  Inside,
  Outside,
  Center
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}
impl Color {
  pub fn to_string(&self) -> String {
    String::from(format!("rgba({}, {}, {}, {})", self.r * 255.0, self.g * 255.0, self.b * 255.0, self.a))
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Paint {
  #[serde(rename = "type")]
  pub paint_type: String,
  pub opacity: Option<f32>,
  pub color: Option<Color>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Rectangle {
  pub x: f32, 
  pub y: f32, 
  pub width: f32,
  pub height: f32
}