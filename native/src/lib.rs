#[macro_use]
extern crate neon;

pub mod math;
pub mod util;
pub mod scene;
pub mod intersectable;
pub mod renderer;
pub mod camera;
pub mod object;

use neon::prelude::*;

use math::{Vector3, Quaternion};
use util::{ImageData, Transform};
use scene::Scene;
use intersectable::{Sphere, Cube, Plane};
use renderer::RayTracer;
use camera::Camera;
use object::Object as RenderObject;

fn render(mut cx: FunctionContext) -> JsResult<JsUndefined> {

  let img_data: Handle<JsObject> = cx.argument::<JsObject>(0)?;
  let width = img_data.get(&mut cx, "width")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let height = img_data.get(&mut cx, "height")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let mut buffer = img_data.get(&mut cx, "data")?.downcast::<JsBuffer>().unwrap_or(cx.buffer(0)?);

  { // Tricks to get rid of borrow checker

    // Setup image data
    let guard = cx.lock();
    let data = buffer.borrow_mut(&guard);
    let mut slice = data.as_mut_slice::<u8>();
    let mut img_data = ImageData { width, height, buffer: &mut slice };

    // Create the scene
    let scene = Scene {
      objects: vec![
        RenderObject {
          transform: Transform {
            position: Vector3::new(0.0, 0.15, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::axis_angle(Vector3::new(0.0, 1.0, 0.0), 3.14),
          },
          intersectable: Box::new(Sphere::new(0.3))
        },
        RenderObject {
          transform: Transform {
            position: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Quaternion::identity(),
          },
          intersectable: Box::new(Plane::new())
        }
      ]
    };

    // Create the camera
    let camera = Camera::two_point(Vector3::new(3.0, 0.2, 1.0), Vector3::zero());

    // Render to image data
    RayTracer::render(&scene, &camera, &mut img_data);
  }

  Ok(cx.undefined())
}

fn fill_black(mut cx: FunctionContext) -> JsResult<JsUndefined> {

  let img_data: Handle<JsObject> = cx.argument::<JsObject>(0)?;
  let width = img_data.get(&mut cx, "width")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let height = img_data.get(&mut cx, "height")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let mut buffer = img_data.get(&mut cx, "data")?.downcast::<JsBuffer>().unwrap_or(cx.buffer(0)?);
  let buffer_length = buffer.get(&mut cx, "length")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;

  { // Tricks to get rid of borrow checker
    let guard = cx.lock();
    let data = buffer.borrow_mut(&guard);
    let slice = data.as_mut_slice::<u8>();

    for x in 0..width {
      for y in 0..height {
        let index: usize = (y * width + x) * 4;
        if index + 3 < buffer_length {
          slice[index] = 0;
          slice[index + 1] = 0;
          slice[index + 2] = 0;
          slice[index + 3] = 255;
        }
      }
    }
  }

  Ok(cx.undefined())
}

register_module!(mut cx, {
  cx.export_function("render", render)?;
  cx.export_function("fillBlack", fill_black)?;
  Ok(())
});
