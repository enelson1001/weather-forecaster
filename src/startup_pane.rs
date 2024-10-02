//============================================================================
//                              Startup Pane
//============================================================================
use crate::lvgl_misc;
use cstr_core::CString;
use lvgl::font::Font;
use lvgl::style::Style;
use lvgl::widgets::Label;
use lvgl::{Align, Color, Obj, Part, Screen, TextAlign, Widget};
use lvgl_sys::*;

pub struct StartupPane<'a> {
    pane: Obj<'a>,
    message_label: Label<'a>,
}

impl<'a> StartupPane<'a> {
    pub fn new(startup_pane_parent: &'a mut Screen) -> Self {
        let mut pane = Obj::create(startup_pane_parent).unwrap();
        pane.set_size(800, 480);
        pane.set_align(Align::TopLeft, 0, 0);
        pane.add_style(Part::Main, Box::leak(startup_pane_style()));

        let mut message_label = Label::create(&mut pane).unwrap();
        message_label.add_style(Part::Main, Box::leak(message_label_style()));
        message_label.set_align(Align::Center, 0, 0);
        message_label
            .set_text(CString::new("").unwrap().as_c_str())
            .unwrap();

        Self {
            pane,
            message_label,
        }
    }

    pub fn set_message(&mut self, msg: String) {
        self.message_label
            .set_text(CString::new(msg.as_str()).unwrap().as_c_str())
            .unwrap();
    }

    pub fn show(&mut self) {
        lvgl_misc::show_obj(&mut self.pane);
    }

    pub fn hide(&mut self) {
        lvgl_misc::hide_obj(&mut self.pane);
    }
}

//*****************************************************************************
//                      Sytles for Startup Pane
//*****************************************************************************
pub fn startup_pane_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_bg_color(Color::from_rgb((0, 0, 139))); // blue
    style.set_radius(0);
    style.set_border_width(0);
    style.set_pad_top(0);
    style.set_pad_bottom(0);
    style.set_pad_left(0);
    style.set_pad_right(0);

    Box::new(style)
}

pub fn message_label_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((255, 255, 255))); // black
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_24) });

    Box::new(style)
}
