use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::lorenz::{Lorenz, create_default_lorenz};

#[derive(Debug, Clone)]
pub struct Trail {
  size: u8,
  trail: Vec<Lorenz>,
}

impl Trail {
  fn add_lorenz(&mut self) -> () {
    let last_lorenz_ref = &self.trail[self.trail.len() - 1];
    let mut new_lorenz:Lorenz = Lorenz {
      ..last_lorenz_ref.clone()
    };
    new_lorenz.set_new_coords();
    self.trail.push(new_lorenz);
    if self.size < self.trail.len() as u8 {
      self.trail.remove(0);
    }
  }

  pub fn draw(&mut self, c:&mut Canvas<Window>) -> () {
    self.add_lorenz();
    let mut color_value:u8 = 0;

    for i in &self.trail {
      let color = sdl2::pixels::Color { 
        r: (color_value), 
        g: (0), 
        b: (255 - color_value),
        a: (255)
      };

      i.draw(c, color);
      color_value += (255 / self.trail.len()) as u8;
    }
  }
}

pub fn create_default_trail() -> Trail {
  let mut v = Vec::new();
  v.push(create_default_lorenz());
  return Trail { size: (64), trail: v };
}