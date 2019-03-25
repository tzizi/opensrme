use super::*;
use opensrme_common::*;


// TODO: Some kind of way to lock focus, for example, when holding down a button (e.g. scrollbar bar)
pub struct WidgetState {
  lock_focus: bool
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SizeBoundary {
  None,
  X(IScalar),
  Y(IScalar),
  Both(Vec3i)
}

pub trait Widget {
  fn input(&mut self, _event: Event) {}
  fn set_boundaries(&mut self, _size: SizeBoundary) {}
  fn get_size(&self) -> Vec3i { Vec3i::default() }
  fn get_min_size(&self) -> Option<Vec3i> { None }
  fn get_max_size(&self) -> Option<Vec3i> { None }
  fn draw(&self, _offset: Vec3i) {}
}

const DIALOG_MARGIN: IScalar = 20;
const DIALOG_MARGIN2: IScalar = DIALOG_MARGIN * 2;

const DIALOG_PADDING: IScalar = 10;

pub struct Dialog {
  boundaries: SizeBoundary,
  boundaries_size: Vec3i,
  widget: Box<Widget>,
  size: Vec3i,
}

impl Dialog {
  pub fn new(widget: Box<Widget>) -> Self {
    let size = widget.get_size() + DIALOG_PADDING * 2;

    Dialog {
      boundaries: SizeBoundary::None,
      boundaries_size: Vec3i::default(),
      widget: widget,
      size
    }
  }
}

impl Widget for Dialog {
  fn set_boundaries(&mut self, new_boundaries: SizeBoundary) {
    if new_boundaries == self.boundaries {
      return;
    }

    let size = self.get_size();

    self.boundaries = new_boundaries;

    if let SizeBoundary::Both(mut boundary) = new_boundaries {
      self.boundaries_size = boundary;

      if boundary.x - size.x < DIALOG_MARGIN2 {
        boundary.x = boundary.x - DIALOG_MARGIN2;
      }

      if boundary.y - size.y < DIALOG_MARGIN2 {
        boundary.y = boundary.y - DIALOG_MARGIN2;
      }

      self.widget.set_boundaries(SizeBoundary::Both(boundary - DIALOG_PADDING * 2));
    } else {
      // A dialog should always be top-level
      panic!("Dialog.set_boundaries() should be SizeBoundary::Both");
    }
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
    let offset = offset + (self.boundaries_size - size) / 2;

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 150 });
    context.platform.fill_rect(offset.x, offset.y,
                               size.x,
                               size.y);

    self.widget.draw(offset + DIALOG_PADDING);
  }
}

pub struct TextWidget {
  boundaries: SizeBoundary,
  text: String,
  split_text: Vec<String>,
  wrapped: Vec<String>,
  full_size: Vec3i,
  actual_size: Vec3i
}

impl TextWidget {
  pub fn new(text: &str) -> Self {
    let mut widget = TextWidget {
      boundaries: SizeBoundary::None,
      text: text.to_string(),
      split_text: vec![],
      wrapped: vec![],
      full_size: Vec3i::default(),
      actual_size: Vec3i::default()
    };

    widget.set_text(text);

    widget
  }

  fn get_split_size(lines: &Vec<String>) -> Vec3i {
    let mut size = Vec3i::default();

    for line in lines.iter() {
      let line_size = text::text_size(0, &line[..]);
      if line_size.x > size.x {
        size.x = line_size.x;
      }

      size.y += line_size.y;
    }

    size
  }

  fn update_split_text(&mut self) {
    self.split_text = vec![];

    let mut size = Vec3i::default();

    for line in self.text.lines() {
      self.split_text.push(line.to_string());
    }

    self.full_size = TextWidget::get_split_size(&self.split_text);
  }

  fn update_wrap(&mut self) {
    let x_size = match self.boundaries {
      SizeBoundary::Both(size) => { size.x },
      SizeBoundary::X(x) => { x },
      // TODO: Y?
      _ => {
        self.wrapped = self.split_text.clone();
        self.actual_size = self.full_size;
        return;
      }
    };

    self.wrapped = text::word_wrap(0, &self.text[..], x_size);
    self.actual_size = TextWidget::get_split_size(&self.wrapped);
  }

  pub fn set_text(&mut self, text: &str) {
    self.text = text.to_string();

    self.update_split_text();
    self.update_wrap();
  }
}

impl Widget for TextWidget {
  fn set_boundaries(&mut self, new_boundaries: SizeBoundary) {
    self.boundaries = new_boundaries;
    self.update_wrap();
  }

  fn get_size(&self) -> Vec3i {
    self.actual_size
  }

  fn get_max_size(&self) -> Option<Vec3i> {
    Some(self.full_size)
  }

  fn draw(&self, offset: Vec3i) {
    // TODO: clipping, alignment
    let mut offset = offset;

    for line in self.wrapped.iter() {
      let size = text::draw_text(0, line, offset);
      offset.y += size.y;
    }
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

  fn get_padding(&self) -> Vec3i {
    Vec3i::new2(
      self.padding_left + self.padding_right,
      self.padding_top + self.padding_bottom
    )
  }

  fn get_full_size(&self, item_size: Vec3i) -> Vec3i {
    Vec3i::new2(
      self.padding_left + item_size.x + self.padding_right,
      self.padding_top + item_size.y + self.padding_bottom
    )
  }
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum BoxOrientation {
  HORIZONTAL = 0,
  VERTICAL = 1
}

pub struct BoxContainer {
  orientation: BoxOrientation,
  pub items: Vec<BoxItem>,
  boundaries: SizeBoundary
}

impl BoxContainer {
  pub fn new(orientation: BoxOrientation) -> BoxContainer {
    BoxContainer {
      orientation,
      items: Vec::new(),
      boundaries: SizeBoundary::None
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

  fn quick_set_boundaries(&mut self) {
    for item in self.items.iter_mut() {
      item.widget.set_boundaries(self.boundaries);
    }
  }
}

impl Widget for BoxContainer {
  // TODO: optimize duplicate get_size() calls

  fn set_boundaries(&mut self, new_boundaries: SizeBoundary) {
    // TODO: fix padding
    if new_boundaries == self.boundaries {
      return;
    }

    self.boundaries = new_boundaries;

    let mut remaining_size = match new_boundaries {
      SizeBoundary::None => {
        return self.quick_set_boundaries();
      },
      SizeBoundary::X(x) => {
        if self.orientation == BoxOrientation::VERTICAL {
          return self.quick_set_boundaries();
        }

        x
      },
      SizeBoundary::Y(y) => {
        if self.orientation == BoxOrientation::HORIZONTAL {
          return self.quick_set_boundaries();
        }

        y
      },
      SizeBoundary::Both(size) => {
        if self.orientation == BoxOrientation::VERTICAL {
          size.y
        } else {
          size.x
        }
      }
    };

    let mut item_id = 0;
    let items_len = self.items.len();
    for item in self.items.iter_mut() {
      let item_padding = item.get_padding();
      let item_hv_size = remaining_size / (items_len - item_id) as IScalar;
      let item_boundaries = match new_boundaries {
        SizeBoundary::X(_) => {
          SizeBoundary::X(item_hv_size - item_padding.x)
        },
        SizeBoundary::Y(_) => {
          SizeBoundary::Y(item_hv_size - item_padding.y)
        },
        SizeBoundary::Both(size) => {
          if self.orientation == BoxOrientation::VERTICAL {
            SizeBoundary::Both(Vec3i::new2(
              size.x - item_padding.x,
              item_hv_size - item_padding.y
            ))
          } else {
            SizeBoundary::Both(Vec3i::new2(
              item_hv_size - item_padding.x,
              size.y - item_padding.y,
            ))
          }
        },
        _ => {
          panic!("This shouldn't happen");
        }
      };

      item.widget.set_boundaries(item_boundaries);

      let item_size = item.get_full_size(item.widget.get_size());
      remaining_size -= if self.orientation == BoxOrientation::VERTICAL {
        item_size.y
      } else {
        item_size.x
      };

      item_id += 1;
    }
  }

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

    pausemenu.boxc.add_item_ap(Box::new(TextWidget::new("The quick  brown fox")), 0);
    pausemenu.boxc.add_item_ap(Box::new(TextWidget::new("jumps")), 30);
    pausemenu.boxc.add_item_ap(Box::new(TextWidget::new("over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.")), 0);

    pausemenu.boxc.set_boundaries(SizeBoundary::X(50));

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
