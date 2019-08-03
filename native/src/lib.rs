#[macro_use]
extern crate neon;

#[macro_use]
pub mod math;
pub mod util;
pub mod scene;
pub mod intersectable;
pub mod renderer;
pub mod camera;
pub mod object;
pub mod bounded;

use neon::prelude::*;

use math::{Vector3, Quaternion};
use util::{ImageData, Transform};
use scene::Scene;
use intersectable::{Sphere, Cube, Plane};
use renderer::RayTracer;
use camera::{Camera, ThirdPersonCamera};
use object::Object as RenderObject;

fn render(mut cx: FunctionContext) -> JsResult<JsUndefined> {

  let img_data: Handle<JsObject> = cx.argument::<JsObject>(0)?;
  let width = img_data.get(&mut cx, "width")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let height = img_data.get(&mut cx, "height")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let mut buffer = img_data.get(&mut cx, "data")?.downcast::<JsBuffer>().unwrap_or(cx.buffer(0)?);

  let camera: Handle<JsObject> = cx.argument::<JsObject>(1)?;
  let target = camera.get(&mut cx, "target")?.downcast::<JsObject>().unwrap_or(JsObject::new(&mut cx));
  let target_x = target.get(&mut cx, "x")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as f32;
  let target_y = target.get(&mut cx, "y")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as f32;
  let target_z = target.get(&mut cx, "z")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as f32;
  let azimuth = camera.get(&mut cx, "azimuth")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as f32;
  let incline = camera.get(&mut cx, "incline")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as f32;
  let distance = camera.get(&mut cx, "distance")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as f32;
  let tpc = ThirdPersonCamera {
    target: vec3!(target_x, target_y, target_z),
    azimuth,
    incline,
    distance,
  };

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
            position: vec3!(0.0, 0.15, 0.0),
            scale: vec3!(1.0, 1.0, 1.0),
            rotation: Quaternion::axis_angle(vec3!(0.0, 1.0, 0.0), 3.14),
          },
          intersectable: Box::new(Sphere::new(0.3))
        },
        RenderObject {
          transform: Transform {
            position: vec3!(0.0),
            scale: vec3!(1.0, 1.0, 1.0),
            rotation: Quaternion::identity(),
          },
          intersectable: Box::new(Plane::new())
        }
      ]
    };

    // Create the camera
    let camera = Camera::third_person(&tpc);

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
