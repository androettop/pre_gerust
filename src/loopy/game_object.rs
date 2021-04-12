use super::Game;

const DEFAULT_VISIBLE: bool = false;
const DEFAULT_PRIORITY: u8 = 16;
const DEFAULT_LAYER: u8 = 16;

pub struct GameObject {
  visible: bool,
  priority: u8,
  layer: u8,
  game_instance: Game,
}

impl GameObject {
  pub fn init(game: Game) -> Self {
    Self {
      visible: DEFAULT_VISIBLE,
      priority: DEFAULT_PRIORITY,
      layer: DEFAULT_LAYER,
      game_instance: game,
    }
  }

  pub fn is_visible(&self) {
    self.visible;
  }
  pub fn set_visible(&mut self, value: bool) {
    self.visible = value;
  }

  pub fn get_priority(&self) {
    self.priority;
  }
  pub fn set_priority(&mut self, value: u8) {
    self.priority = value;
  }

  pub fn get_layer(&self) {
    self.layer;
  }
  pub fn set_layer(&mut self, value: u8) {
    self.layer = value;
  }
}
