use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug)]
pub struct Lorenz {
  pub x: f32,
  pub y: f32,
  pub z: f32,

  pub a: f32,
  pub b: f32,
  pub c: f32,

  pub dt: f32,
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

  pub fn set_new_coords(&mut self) -> () {
    let dx: f32 = self.get_dx();
    let dy: f32 = self.get_dy();
    let dz: f32 = self.get_dz();

    self.x += dx;
    self.y += dy;
    self.z += dz;
  }

  pub fn draw(&self, c:&mut Canvas<Window>, color:sdl2::pixels::Color) {
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

pub fn create_default_lorenz() -> Lorenz {
  Lorenz { x: (0.01), y: (0.0), z: (0.0), a: (10.0), b: (28.0), c: (8.0/3.0), dt: (0.01) }
}