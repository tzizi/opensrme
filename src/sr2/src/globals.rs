use super::types::*;

static mut global_context: Option<Context> = None;

pub fn get_context() -> &'static mut Context {
  unsafe {
    if let Some(ref mut context) = &mut global_context {
      context
    } else {
      panic!("Context uninitialized");
    }
  }
}

pub fn set_context(context: Context) {
  unsafe {
    global_context = Some(context);
  }
}

/*pub fn get_level() -> &'static Level {
  let context = get_context();

  //context.data
}*/
