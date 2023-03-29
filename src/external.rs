use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug)]
pub struct Lorenz {
  x: f32,
  y: f32,
  z: f32,

  a: f32,
  b: f32,
  c: f32,

  dt: f32,
}

impl Lorenz {
  fn get_dx(&self) -> f32 {
    return (self.a * (self.y - self.x)) * self.dt;
  }
  fn get_dy(&self) -> f32 {
    return (self.x * (self.b - self.z) - self.y) * self.dt;
  }
  fn get_dz(&self) -> f32 {
    return (self.x * self.y - self.c * self.z) * self.dt;
  }

  fn set_new_coords(&mut self) -> () {
    let dx: f32 = self.get_dx();
    let dy: f32 = self.get_dy();
    let dz: f32 = self.get_dz();

    self.x += dx;
    self.y += dy;
    self.z += dz;
  }

  fn draw(&self, c:&mut Canvas<Window>, color:sdl2::pixels::Color) {
    let scale: u32 = 6;

    let window_size: (u32, u32) = c.window().drawable_size();
    let x_offset: i32 = (window_size.0 / (2 * scale)) as i32;
    let y_offset: i32 = (window_size.1 / (2 * scale)) as i32;

    let r:Rect = Rect::new( 
      x_offset + (self.x as i32),
      y_offset + (self.y as i32), 
      2, 
      2
    );
    c.set_scale(scale as f32, scale as f32).expect("Expect Ok");
    c.set_draw_color(color);
    c.fill_rect(r).expect("Expect Ok");
  }
}

fn create_default_lorenz() -> Lorenz {
  Lorenz { x: (0.01), y: (0.0), z: (0.0), a: (10.0), b: (28.0), c: (8.0/3.0), dt: (0.01) }
}

pub fn create_default_trail() -> Trail {
  let mut v = Vec::new();
  v.push(create_default_lorenz());
  return Trail { size: (64), trail: v };
}

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