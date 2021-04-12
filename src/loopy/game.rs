const DEFAULT_DRAW_BACKGROUND: bool = true;
const DEFAULT_BACKGROUND_COLOR: (u8, u8, u8) = (0, 0, 0);
const DEFAULT_FPS_LIMIT: u16 = 60;
const DEFAILT_ASSETS_ROOT: &'static str = "./assets/";
const DEFAULT_LAST_TICK: u64 = 0;
const DEFAULT_WINDOW_SIZE: (i32, i32) = (800, 600);

pub struct Game {
  pub draw_background: bool,
  pub background_color: (u8, u8, u8),
  pub fps_limit: u16,
  pub assets_root: &'static str,
  game_objects: Vec<super::GameObject>,
  last_tick: u64,
  window_size: (i32, i32),
}

impl Game {
  pub fn new() -> Self {
    Game {
      draw_background: DEFAULT_DRAW_BACKGROUND,
      background_color: DEFAULT_BACKGROUND_COLOR,
      fps_limit: DEFAULT_FPS_LIMIT,
      assets_root: DEFAILT_ASSETS_ROOT,
      game_objects: Vec::new(),
      last_tick: DEFAULT_LAST_TICK,
      window_size: DEFAULT_WINDOW_SIZE,
    }
  }

  pub fn set_draw_background(&mut self, value: bool) {
    self.draw_background = value;
  }

  pub fn set_assets_root(&mut self, value: &'static str) {
    self.assets_root = value;
  }
}
