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

use std::sync::mpsc::{self, RecvTimeoutError, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use neon::prelude::*;

use math::{Color, Vector3, Quaternion};
use util::{ImageData, ImageDimension, Transform};
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

#[derive(Debug)]
pub enum Event {
  SetPixels {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: Color,
  },
  Update,
  Finish
}

fn event_thread(
  scene: Scene,
  camera: Camera,
  img_dim: ImageDimension,
  shutdown_rx: mpsc::Receiver<()>
) -> mpsc::Receiver<Event> {
  let (tx, events_rx) = mpsc::channel();
  thread::spawn(move || {
    for level in img_dim.levels() {

      // Render the tiles
      for tile in level.tiles() {
        let ray = camera.ray(tile.x, tile.y, img_dim.width, img_dim.height);
        // let color = match scene.lock().unwrap().intersect(&ray) {
        let color = match scene.intersect(&ray) {
          Some(itsct) => Color::from(itsct.normal),
          None => Color::black()
        };
        tx.send(Event::SetPixels {
          x: tile.x,
          y: tile.y,
          w: tile.w,
          h: tile.h,
          color
        }).expect("Send failed");
        // println!("Setting pixels {}, {}, {}, {}", tile.x, tile.y, tile.w, tile.h);
      }

      // Check for shutdown signal
      match shutdown_rx.try_recv() {
        Ok(_) | Err(TryRecvError::Disconnected) => { break; }
        Err(TryRecvError::Empty) => {}
      }

      // Finished one level
      tx.send(Event::Update).expect("Send failed");
      // println!("Updating");
    }
    tx.send(Event::Finish).expect("Send failed");
    // println!("Finished");
  });
  events_rx
}

pub struct EventEmitterTask(Arc<Mutex<mpsc::Receiver<Event>>>);

impl Task for EventEmitterTask {
  type Output = Option<Event>;
  type Error = String;
  type JsEvent = JsValue;

  /// The work performed on the `libuv` thread. First acquire a lock on
  /// the receiving thread and then return the received data.
  /// In practice, this should never need to wait for a lock since it
  /// should only be executed one at a time by the `EventEmitter` class.
  fn perform(&self) -> Result<Self::Output, Self::Error> {
    let rx = self
      .0
      .lock()
      .map_err(|_| "Could not obtain lock on receiver".to_string())?;

    // Attempt to read from the channel. Block for at most 100 ms.
    match rx.recv_timeout(Duration::from_millis(100)) {
      Ok(event) => Ok(Some(event)),
      Err(RecvTimeoutError::Timeout) => Ok(None),
      Err(RecvTimeoutError::Disconnected) => Err("Failed to receive event".to_string()),
    }
  }

  /// After the `perform` method has returned, the `complete` method is
  /// scheduled on the main thread. It is responsible for converting the
  /// Rust data structure into a JS object.
  fn complete(
    self,
    mut cx: TaskContext,
    event: Result<Self::Output, Self::Error>,
  ) -> JsResult<Self::JsEvent> {
    // Receive the event or return early with the error
    let event = event.or_else(|err| cx.throw_error(&err.to_string()))?;

    // Timeout occured, return early with `undefined
    let event = match event {
      Some(event) => event,
      None => return Ok(JsUndefined::new().upcast()),
    };

    // Creates an object of the shape `{ "event": string, ...data }`
    let o = match event {
      Event::Update => {
        let o = cx.empty_object();
        let event_type = cx.string("update");
        o.set(&mut cx, "type", event_type).unwrap();
        o
      },
      Event::Finish => {
        let o = cx.empty_object();
        let event_type = cx.string("finish");
        o.set(&mut cx, "type", event_type).unwrap();
        o
      },
      Event::SetPixels { x, y, w, h, color } => {
        let o = cx.empty_object();
        let event_type = cx.string("set_pixel");
        o.set(&mut cx, "type", event_type).unwrap();
        let x = cx.number(x as f64);
        let y = cx.number(y as f64);
        let w = cx.number(w as f64);
        let h = cx.number(h as f64);
        let r = cx.number(color.r as f64);
        let g = cx.number(color.g as f64);
        let b = cx.number(color.b as f64);
        let a = cx.number(color.a as f64);
        o.set(&mut cx, "x", x).unwrap();
        o.set(&mut cx, "y", y).unwrap();
        o.set(&mut cx, "w", w).unwrap();
        o.set(&mut cx, "h", h).unwrap();
        o.set(&mut cx, "r", r).unwrap();
        o.set(&mut cx, "g", g).unwrap();
        o.set(&mut cx, "b", b).unwrap();
        o.set(&mut cx, "a", a).unwrap();
        o
      }
    };

    Ok(o.upcast())
  }
}

pub struct EventEmitter {
  events: Arc<Mutex<mpsc::Receiver<Event>>>,
  shutdown: mpsc::Sender<()>,
}

declare_types! {
  pub class JsEventEmitter for EventEmitter {
    // Called by the `JsEventEmitter` constructor
    init(mut cx) {

      // Image Data
      let img_data: Handle<JsObject> = cx.argument::<JsObject>(0)?;
      let width = img_data.get(&mut cx, "width")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
      let height = img_data.get(&mut cx, "height")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
      // let mut buffer = img_data.get(&mut cx, "data")?.downcast::<JsBuffer>().unwrap_or(cx.buffer(0)?);

      // Set this
      // let this = cx.this();
      // this.set(&mut cx, "img_data", img_data)?;

      // Camera
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

      // Scene
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

      let camera = Camera::third_person(&tpc);

      let (shutdown, shutdown_rx) = mpsc::channel();

      let img_dim = ImageDimension { width, height };

      // Start work in a separate thread
      let rx = event_thread(scene, camera, img_dim, shutdown_rx);

      // Construct a new `EventEmitter` to be wrapped by the class.
      Ok(EventEmitter {
        events: Arc::new(Mutex::new(rx)),
        shutdown,
      })
    }

    // This method should be called by JS to receive data. It accepts a
    // `function (err, data)` style asynchronous callback. It may be called
    // in a loop, but care should be taken to only call it once at a time.
    method poll(mut cx) {
      // The callback to be executed when data is available
      let cb = cx.argument::<JsFunction>(0)?;
      let this = cx.this();

      // Create an asynchronously `EventEmitterTask` to receive data
      let events = cx.borrow(&this, |emitter| Arc::clone(&emitter.events));
      let emitter = EventEmitterTask(events);

      // Schedule the task on the `libuv` thread pool
      emitter.schedule(cb);

      // The `poll` method does not return any data.
      Ok(JsUndefined::new().upcast())
    }

    // The shutdown method may be called to stop the Rust thread. It
    // will error if the thread has already been destroyed.
    method shutdown(mut cx) {
      let this = cx.this();

      // Unwrap the shutdown channel and send a shutdown command
      cx.borrow(&this, |emitter| emitter.shutdown.send(()))
          .or_else(|err| cx.throw_error(&err.to_string()))?;

      Ok(JsUndefined::new().upcast())
    }
  }
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
  cx.export_class::<JsEventEmitter>("RenderStream")?;
  Ok(())
});
