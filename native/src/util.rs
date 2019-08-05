use ::math::{Color, Vector3, Vector4, Quaternion, Matrix4};

pub struct Ray {
  pub origin: Vector3,
  pub direction: Vector3,
}

impl Ray {
  pub fn new(origin: Vector3, direction: Vector3) -> Ray {
    Ray { origin, direction }
  }

  pub fn point_at(&self, t: f32) -> Vector3 {
    self.origin.clone() + self.direction.clone() * t
  }

  pub fn transform(&self, mat: Matrix4) -> Ray {
    Self {
      origin: self.origin.transform_dehomogenous(mat),
      direction: self.direction.transform(mat).normalize(),
    }
  }

  pub fn inverse_transform(&self, mat: Matrix4) -> Ray {
    self.transform(mat.inverse())
  }
}

#[derive(Clone, Copy)]
pub struct Intersection {
  pub position: Vector3,
  pub normal: Vector3,
  pub t: f32,
}

impl Intersection {
  pub fn min(lhs: Option<Self>, rhs: Option<Self>) -> Option<Self> {
    match (lhs, rhs) {
      (Some(i1), Some(i2)) => {
        if i1.t < i2.t {
          lhs.clone()
        } else {
          rhs.clone()
        }
      },
      _ => lhs.or(rhs)
    }
  }

  pub fn transform(&self, mat: Matrix4) -> Self {
    Self {
      position: self.position.transform_dehomogenous(mat),
      normal: self.normal.transform(mat),
      t: self.t
    }
  }
}

pub struct ImageData<'a> {
  pub width: usize,
  pub height: usize,
  pub buffer: &'a mut [u8],
}

impl<'a> ImageData<'a> {
  pub fn set_pixel(&mut self, x: usize, y: usize, c: &Color) {
    let index = (y * self.width + x) * 4;
    self.buffer[index] = c.r;
    self.buffer[index + 1] = c.g;
    self.buffer[index + 2] = c.b;
    self.buffer[index + 3] = c.a;
  }

  pub fn levels(&self) -> ImageLevelIter {
    ImageLevelIter {
      tile_size: 64,
      is_init: true,
      width: self.width,
      height: self.height,
    }
  }
}

pub struct ImageDimension {
  pub width: usize,
  pub height: usize,
}

impl ImageDimension {
  pub fn levels(&self) -> ImageLevelIter {
    ImageLevelIter {
      tile_size: 64,
      is_init: true,
      width: self.width,
      height: self.height,
    }
  }
}

impl<'a> From<ImageData<'a>> for ImageDimension {
  fn from(img_data: ImageData<'a>) -> Self {
    Self {
      width: img_data.width,
      height: img_data.height,
    }
  }
}

pub struct ImageLevel {
  pub tile_size: usize,
  pub is_init: bool,
  pub width: usize,
  pub height: usize,
}

impl ImageLevel {
  pub fn tiles(&self) -> TileIter {
    TileIter {
      tile_size: self.tile_size,
      width: self.width,
      height: self.height,
      is_init: self.is_init,
      x: 0,
      y: 0,
      in_even_row: true
    }
  }
}

pub struct ImageLevelIter {
  pub tile_size: usize,
  pub is_init: bool,
  pub width: usize,
  pub height: usize,
}

impl Iterator for ImageLevelIter {
  type Item = ImageLevel;

  fn next(&mut self) -> Option<ImageLevel> {
    if self.tile_size > 0 {
      let curr_tile_size = self.tile_size;
      let curr_is_init = self.is_init;

      // Mutate
      self.tile_size /= 2;
      self.is_init = false;

      // Generate
      Some(ImageLevel {
        tile_size: curr_tile_size,
        is_init: curr_is_init,
        width: self.width,
        height: self.height
      })
    } else {
      None
    }
  }
}

pub struct Tile {
  pub x: usize,
  pub y: usize,
  pub w: usize,
  pub h: usize,
}

pub struct TileIter {
  pub tile_size: usize,
  pub x: usize,
  pub y: usize,
  pub width: usize,
  pub height: usize,
  pub is_init: bool,
  pub in_even_row: bool,
}

impl Iterator for TileIter {
  type Item = Tile;

  fn next(&mut self) -> Option<Tile> {
    if self.y >= self.height {
      None
    } else {

      let curr_x = self.x;
      let curr_y = self.y;

      // Get the states
      let (
        next_x,
        next_y,
        curr_tile_width,
        curr_tile_height
      ) = if curr_x + self.tile_size >= self.width {
        let next_x = 0;
        let curr_tile_width = self.width - curr_x;
        if curr_y + self.tile_size >= self.height {
          let next_y = self.height;
          let curr_tile_height = self.height - curr_y;
          (next_x, next_y, curr_tile_width, curr_tile_height)
        } else {
          let next_y = curr_y + self.tile_size;
          let curr_tile_height = self.tile_size;
          (next_x, next_y, curr_tile_width, curr_tile_height)
        }
      } else {
        let next_x = curr_x + self.tile_size;
        let curr_tile_width = self.tile_size;
        let next_y = curr_y;
        let curr_tile_height = self.tile_size.min(self.height - curr_y);
        (next_x, next_y, curr_tile_width, curr_tile_height)
      };

      // Do the mutation
      self.x = next_x;
      self.y = next_y;

      Some(Tile {
        x: curr_x,
        y: curr_y,
        w: curr_tile_width,
        h: curr_tile_height,
      })
    }
  }
}

#[derive(Copy, Clone)]
pub struct Transform {
  pub position: Vector3,
  pub scale: Vector3,
  pub rotation: Quaternion,
}

impl Into<Matrix4> for Transform {
  fn into(self) -> Matrix4 {
    let pos_mat = Matrix4::translate_matrix(self.position);
    let scale_mat = Matrix4::scale_matrix(self.scale);
    let rot_mat: Matrix4 = self.rotation.into();
    pos_mat * scale_mat * rot_mat
  }
}