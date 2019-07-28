#[macro_use]
extern crate neon;

use std::time::{Duration, Instant};
use neon::prelude::*;

fn fill_black(mut cx: FunctionContext) -> JsResult<JsUndefined> {

  let begin = Instant::now();

  let img_data: Handle<JsObject> = cx.argument::<JsObject>(0)?;
  let width = img_data.get(&mut cx, "width")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let height = img_data.get(&mut cx, "height")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;
  let mut buffer = img_data.get(&mut cx, "data")?.downcast::<JsBuffer>().unwrap_or(cx.buffer(0)?);
  let buffer_length = buffer.get(&mut cx, "length")?.downcast::<JsNumber>().unwrap_or(cx.number(0)).value() as usize;

  let fetch = Instant::now();
  println!("Fetch data elapsed: {:?}", fetch.duration_since(begin));

  {
    let guard = cx.lock();
    let data = buffer.borrow_mut(&guard);
    let slice: &[u8] = data.as_mut_slice::<u8>();

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

  println!("Fill pixels elapsed: {:?}", Instant::now().duration_since(fetch));
  Ok(cx.undefined())
}

register_module!(mut cx, {
  cx.export_function("fillBlack", fill_black)?;
  Ok(())
});
