use super::*;


pub trait Widget {
  fn input(&mut self, _event: input::InputKey) {}
  fn set_boundaries(&mut self, _size: Vec3i) {}
  fn get_size(&self) -> Vec3i { Vec3i::default() }
  fn draw(&self, _offset: Vec3i) {}
}

const DIALOG_MARGIN: IScalar = 20;
const DIALOG_MARGIN2: IScalar = DIALOG_MARGIN * 2;

const DIALOG_PADDING: IScalar = 10;

pub struct Dialog {
  boundaries: Vec3i,
  widget: Box<Widget>,
  size: Vec3i,
}

impl Dialog {
  pub fn new(widget: Box<Widget>) -> Self {
    let size = widget.get_size() + DIALOG_PADDING * 2;

    Dialog {
      boundaries: Vec3i::default(),
      widget: widget,
      size
    }
  }
}

impl Widget for Dialog {
  fn set_boundaries(&mut self, new_boundaries: Vec3i) {
    if new_boundaries == self.boundaries {
      return;
    }

    self.boundaries = new_boundaries;

    if self.boundaries.x - self.size.x < DIALOG_MARGIN2 {
      self.size.x = self.boundaries.x - DIALOG_MARGIN2;
    }

    if self.boundaries.y - self.size.y < DIALOG_MARGIN2 {
      self.size.y = self.boundaries.y - DIALOG_MARGIN2;
    }

    // TODO: expand

    self.widget.set_boundaries(self.size - DIALOG_PADDING * 2);
  }

  fn get_size(&self) -> Vec3i {
    self.size + DIALOG_PADDING * 2
  }

  fn draw(&self, offset: Vec3i) {
    let context = globals::get_context();

    let offset = offset + (self.boundaries - self.size) / 2;

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 150 });
    context.platform.fill_rect(offset.x, offset.y,
                               self.size.x,
                               self.size.y);

    self.widget.draw(offset + DIALOG_PADDING);
  }
}

pub struct TextWidget {
  boundaries: Vec3i,
  text: String
}

impl TextWidget {
  pub fn new(text: &str) -> Self {
    TextWidget {
      boundaries: Vec3i::default(),
      text: text.to_string()
    }
  }

  pub fn set_text(&mut self, text: &str) {
    self.text = text.to_string();
  }
}

impl Widget for TextWidget {
  fn set_boundaries(&mut self, new_boundaries: Vec3i) {
    self.boundaries = new_boundaries;
  }

  fn get_size(&self) -> Vec3i {
    text::text_size(0, &self.text[..])
  }

  fn draw(&self, offset: Vec3i) {
    // TODO: clipping, wrapping, alignment
    text::draw_text(0, &self.text[..], offset);
  }
}

pub struct PauseMenu {
  textwidget: TextWidget
}

impl PauseMenu {
  pub fn new() -> Self {
    PauseMenu {
      textwidget: TextWidget::new("The quick brown fox jumps over the lazy dog.")
    }
  }
}

impl Widget for PauseMenu {
  fn get_size(&self) -> Vec3i {
    //Vec3i::new2(50, 50)
    self.textwidget.get_size()
  }

  fn draw(&self, offset: Vec3i) {
    self.textwidget.draw(offset);
  }
}
