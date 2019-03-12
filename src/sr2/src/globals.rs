use super::*;
use opensrme_common::*;
use std::collections::HashMap;

pub struct Context {
  pub running: bool,
  pub archive: Box<Archive>,
  pub platform: Box<Platform>,
  pub realtime: Time,
  pub time: Time,
  pub delta: Time,
  pub data: DataContext,
  pub palette_images: Vec<Vec<PlatformId>>,
  pub font_images: Vec<PlatformId>,
  pub levels: HashMap<LevelId, Level>,
  pub game: *mut screen::GameScreen,
  pub screen: Option<Box<screen::Screen>>,
  pub input: input::InputContext
}


// https://doc.rust-lang.org/std/mem/fn.transmute.html
struct R<'a, T>(&'a mut T);
unsafe fn extend_lifetime<'b, T>(r: R<'b, T>) -> R<'static, T> {
    std::mem::transmute::<R<'b, T>, R<'static, T>>(r)
}

static mut GLOBAL_CONTEXT: Option<Context> = None;

pub fn get_context() -> &'static mut Context {
  unsafe {
    if let Some(ref mut context) = &mut GLOBAL_CONTEXT {
      context
    } else {
      panic!("Context uninitialized");
    }
  }
}

pub fn set_context(context: Context) {
  unsafe {
    GLOBAL_CONTEXT = Some(context);
  }
}

pub fn get_game() -> &'static mut super::screen::GameScreen {
  let context = get_context();

  unsafe {
    if context.game == std::ptr::null_mut() {
      panic!("GameScreen is null");
    } else {
      extend_lifetime(R(&mut (*context.game))).0
    }
  }
}

pub fn set_screen<T: screen::Screen>(screen: T) where T: Sized+'static {
  let context = get_context();

  context.screen = Some(Box::new(screen));
  context.game = std::ptr::null_mut();
}

pub fn set_game(screen: screen::GameScreen) {
  let context = get_context();

  let mut our_box = Box::new(screen);
  context.game = &mut (*our_box) as *mut screen::GameScreen;
  context.screen = Some(our_box);
}
