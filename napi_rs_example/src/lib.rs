#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
struct Event {
  name: JsString,
  callback: JsFunction,
}

#[napi]
pub fn listen(name: JsString, callback: JsFunction) {
  let mut list: Vec<Event> = Vec::new();
  list.push(Event { name, callback });
  println!("listen,{:?}", list.len());
}
#[napi(constructor)]
struct Animal {
  pub name: String,
  pub kind: u32,
}

#[napi]
impl Animal {
  #[napi]
  pub fn change_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}

use std::thread;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  JsString,
};

#[napi]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  tsfn.call(Ok(101), ThreadsafeFunctionCallMode::NonBlocking);
  tsfn.call(Ok(0), ThreadsafeFunctionCallMode::NonBlocking);
  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
    });
  }
  Ok(())
}
