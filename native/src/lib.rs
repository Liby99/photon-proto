#[macro_use]
extern crate neon;

use std::time::{Duration, Instant};
use neon::prelude::*;

fn fill_black(mut cx: FunctionContext) -> JsResult<JsUndefined> {

  let begin = Instant::now();

  let img_data: Handle<JsObject> = cx.argument::<JsObject>(0)?;
  let width = img_data.get(&mut cx, "width")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as u32;
  let height = img_data.get(&mut cx, "height")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as u32;
  let buffer = img_data.get(&mut cx, "data")?.downcast::<JsBuffer>().unwrap_or(cx.buffer(0)?);
  let buffer_length = buffer.get(&mut cx, "length")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as u32;

  let fetch = Instant::now();
  println!("Fetch data elapsed: {:?}", fetch.duration_since(begin));

  for x in 0..width {
    for y in 0..height {
      let index = (y * width + x) * 4;
      if index + 3 < buffer_length {
        let r = cx.number(0);
        let g = cx.number(0);
        let b = cx.number(0);
        let a = cx.number(255);
        buffer.set(&mut cx, index, r)?;
        buffer.set(&mut cx, index + 1, g)?;
        buffer.set(&mut cx, index + 2, b)?;
        buffer.set(&mut cx, index + 3, a)?;
      }
    }
  }

  println!("Fill pixels elapsed: {:?}", Instant::now().duration_since(fetch));
  Ok(cx.undefined())
}

register_module!(mut cx, {
  cx.export_function("fillBlack", fill_black)?;
  Ok(())
});
