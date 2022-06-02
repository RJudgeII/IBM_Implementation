#[typetag::serde(tag = "type", content = "value")]
pub trait ActsAsControl {
  fn port_number(&self) -> String;
  fn post_url(&self) -> String;
  fn get_url(&self) -> String;
}
