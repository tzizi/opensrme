use super::*;
use opensrme_common::*;


// TODO: Some kind of way to lock focus, for example, when holding down a button (e.g. scrollbar bar)
pub struct WidgetState {
  lock_focus: bool
}

pub trait Widget {
  fn input(&mut self, _event: Event) {}
  fn set_boundaries(&mut self, _size: Vec3i) {}
  fn get_size(&self) -> Vec3i { Vec3i::default() }
  fn get_min_size(&self) -> Option<Vec3i> { None }
  fn get_max_size(&self) -> Option<Vec3i> { None }
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
    self.widget.get_size() + DIALOG_PADDING * 2
  }

  fn get_min_size(&self) -> Option<Vec3i> {
    Some(Vec3i::new2(DIALOG_MARGIN2, DIALOG_MARGIN2) + self.widget.get_min_size().unwrap_or(Vec3i::default()))
  }

  fn draw(&self, offset: Vec3i) {
    let context = globals::get_context();

    let size = self.get_size();
    let offset = offset + (self.boundaries - size) / 2;

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 150 });
    context.platform.fill_rect(offset.x, offset.y,
                               size.x,
                               size.y);

    self.widget.draw(offset + DIALOG_PADDING);
  }
}

pub struct TextWidget {
  boundaries: Vec3i,
  text: String
}

impl TextWidget {
  pub fn new(text: &str) -> Self {
    let mut widget = TextWidget {
      boundaries: Vec3i::default(),
      text: text.to_string()
    };

    let max_size = widget.get_max_size().unwrap();
    widget.set_boundaries(max_size);

    widget
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
    self.boundaries
  }

  fn get_max_size(&self) -> Option<Vec3i> {
    Some(text::text_size(0, &self.text[..]))
  }

  fn draw(&self, offset: Vec3i) {
    // TODO: clipping, wrapping, alignment
    text::draw_text(0, &self.text[..], offset);
  }
}

pub struct BoxItem {
  widget: Box<Widget>,
  padding_top: IScalar,
  padding_bottom: IScalar,
  padding_left: IScalar,
  padding_right: IScalar
}

impl BoxItem {
  pub fn new(widget: Box<Widget>,
             top: IScalar,
             bottom: IScalar,
             left: IScalar,
             right: IScalar) -> BoxItem {
    BoxItem {
      widget,
      padding_top: top,
      padding_bottom: bottom,
      padding_left: left,
      padding_right: right
    }
  }

  fn get_full_size(&self, item_size: Vec3i) -> Vec3i {
    Vec3i::new2(
      self.padding_left + item_size.x + self.padding_right,
      self.padding_top + item_size.y + self.padding_bottom
    )
  }
}

pub enum BoxOrientation {
  HORIZONTAL = 0,
  VERTICAL = 1
}

pub struct BoxContainer {
  orientation: BoxOrientation,
  pub items: Vec<BoxItem>
}

impl BoxContainer {
  pub fn new(orientation: BoxOrientation) -> BoxContainer {
    BoxContainer {
      orientation,
      items: Vec::new()
    }
  }

  pub fn add_item(&mut self, widget: Box<Widget>,
              top: IScalar,
              bottom: IScalar,
              left: IScalar,
              right: IScalar) {
    self.items.push(BoxItem::new(widget, top, bottom, left, right))
  }

  pub fn add_item_ap(&mut self, widget: Box<Widget>, all: IScalar) {
    self.add_item(widget, all, all, all, all)
  }

  fn calc_size_increase(&self, size: Vec3i, item_size: Vec3i) -> Vec3i {
    match self.orientation {
      BoxOrientation::HORIZONTAL => {
        Vec3i::new2(
          item_size.x,
          if item_size.y > size.y {
            item_size.y - size.y
          } else {
            0
          }
        )
      },
      BoxOrientation::VERTICAL => {
        Vec3i::new2(
          if item_size.x > size.x {
            item_size.x - size.x
          } else {
            0
          },
          item_size.y
        )
      }
    }
  }
}

impl Widget for BoxContainer {
  // TODO: optimize duplicate get_size() calls
  // TODO: set_boundaries, and proper sizing for children elements (min/max?)

  fn get_size(&self) -> Vec3i {
    let mut size = Vec3i::default();

    for item in self.items.iter() {
      let item_size = item.widget.get_size();
      let full_item_size = item.get_full_size(item_size);

      size = size + self.calc_size_increase(size, full_item_size);
    }

    size
  }

  fn get_max_size(&self) -> Option<Vec3i> {
    let mut size = Vec3i::default();

    for item in self.items.iter() {
      let item_size = item.widget.get_max_size();

      match item_size {
        Some(item_size) => {
          let full_item_size = item.get_full_size(item_size);

          size = size + self.calc_size_increase(size, full_item_size);
        },
        None => {
          return None;
        }
      }
    }

    Some(size)
  }

  fn get_min_size(&self) -> Option<Vec3i> {
    let mut size = Vec3i::default();

    for item in self.items.iter() {
      let item_size = item.widget.get_max_size().unwrap_or(Vec3i::default());
      let full_item_size = item.get_full_size(item_size);

      size = size + self.calc_size_increase(size, full_item_size);
    }

    Some(size)
  }

  fn draw(&self, base_offset: Vec3i) {
    let mut offset = Vec3i::default();

    for item in self.items.iter() {
      let item_size = item.widget.get_size();
      let mut append_size = Vec3i::default();

      match self.orientation {
        BoxOrientation::HORIZONTAL => {
          offset.x += item.padding_left;
          offset.y = item.padding_top;

          append_size.x = item_size.x + item.padding_right;
        },
        BoxOrientation::VERTICAL => {
          offset.x = item.padding_left;
          offset.y += item.padding_top;

          append_size.y = item_size.y + item.padding_bottom;
        }
      }

      item.widget.draw(base_offset + offset);

      offset = offset + append_size;
    }
  }
}

pub struct PauseMenu {
  boxc: BoxContainer
}

impl PauseMenu {
  pub fn new() -> Self {
    let mut pausemenu = PauseMenu {
      boxc: BoxContainer::new(BoxOrientation::VERTICAL)
    };

    pausemenu.boxc.add_item_ap(Box::new(TextWidget::new("The quick brown fox")), 0);
    pausemenu.boxc.add_item_ap(Box::new(TextWidget::new("jumps")), 30);
    pausemenu.boxc.add_item_ap(Box::new(TextWidget::new("over the lazy dog.")), 0);

    pausemenu
  }
}

impl Widget for PauseMenu {
  fn get_size(&self) -> Vec3i {
    self.boxc.get_size()
  }

  fn get_max_size(&self) -> Option<Vec3i> {
    self.boxc.get_max_size()
  }

  fn get_min_size(&self) -> Option<Vec3i> {
    self.boxc.get_min_size()
  }

  fn draw(&self, offset: Vec3i) {
    self.boxc.draw(offset);
  }
}
