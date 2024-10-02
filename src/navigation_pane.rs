//============================================================================
//                              Navigation Pane
//============================================================================
use crate::lvgl_misc::*;
use cstr_core::CString;
use lvgl::font::Font;
use lvgl::style::{Opacity, Style};
use lvgl::widgets::{Btnmatrix, Label};
use lvgl::{Align, Color, Obj, Part, Screen, TextAlign, Widget};
use lvgl_sys::*;

//use log::info;

pub struct NavigationPane<'a> {
    pane: Obj<'a>,
    pub btn_matrix: Btnmatrix<'a>,
    prev_btn_selected: u16,
}

impl<'a> NavigationPane<'a> {
    pub fn new(navigation_pane_parent: &'a mut Screen) -> Self {
        let mut pane = Obj::create(navigation_pane_parent).unwrap();
        pane.set_size(800, 49);
        pane.set_align(Align::TopLeft, 0, 0);
        pane.add_style(Part::Main, Box::leak(navigation_pane_style()));

        // Create the btn matrix vector that holds the button's name,  last entry of "" signifys no more buttons
        let btnmatrix_vec = vec![
            CString::new("Home").unwrap(),
            CString::new("Wifi Settings").unwrap(),
            CString::new("Cities Settings").unwrap(),
            CString::new("").unwrap(),
        ];

        // Create button matrix
        let mut btn_matrix = Btnmatrix::create(&mut pane).unwrap();
        btn_matrix.set_size(400, 40);
        btn_matrix.set_align(Align::TopLeft, 10, 6);
        btn_matrix.add_style(Part::Main, Box::leak(btn_matrix_style()));
        btnmatrix_set_map(&mut btn_matrix, btnmatrix_vec);
        btnmatrix_set_btn_ctrl_all(&mut btn_matrix);
        btnmatrix_clear_btn_ctrl(&mut btn_matrix, 1);
        btnmatrix_clear_btn_ctrl(&mut btn_matrix, 2);
        btnmatrix_set_one_checked(&mut btn_matrix);
        btnmatrix_set_btn_ctrl(&mut btn_matrix, 0);

        // App title
        let mut app_title = Label::create(&mut pane).unwrap();
        app_title.add_style(Part::Main, Box::leak(app_title_style()));
        app_title.set_width(320);
        app_title.set_align(Align::TopLeft, 420, 4);
        app_title
            .set_text(CString::new("Weather Forecaster").unwrap().as_c_str())
            .unwrap();

        let mut app_source = Label::create(&mut pane).unwrap();
        app_source.add_style(Part::Main, Box::leak(app_source_style()));
        app_source.set_width(320);
        app_source.set_align(Align::TopLeft, 420, 32);
        app_source
            .set_text(CString::new("uses weatherapi.com").unwrap().as_c_str())
            .unwrap();

        Self {
            pane,
            btn_matrix,
            prev_btn_selected: 0u16,
        }
    }

    pub fn show(&mut self) {
        show_obj(&mut self.pane);
    }

    pub fn hide(&mut self) {
        hide_obj(&mut self.pane);
    }

    pub fn get_btn_selected(&self) -> u16 {
        btnmatrix_get_selected_btn(&self.btn_matrix)
    }

    pub fn set_home_btn_selected(&mut self) {
        self.set_new_btn_selected(0u16);
    }

    pub fn set_new_btn_selected(&mut self, new_btn_selected: u16) {
        btnmatrix_clear_btn_ctrl(&mut self.btn_matrix, self.prev_btn_selected);
        btnmatrix_set_btn_ctrl(&mut self.btn_matrix, new_btn_selected);
        self.prev_btn_selected = new_btn_selected;
        lvgl::task_handler();
    }
}

//*****************************************************************************
//                      Sytles for Navigation Pane
//*****************************************************************************
fn navigation_pane_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_bg_color(Color::from_rgb((0, 0, 0))); // black
    style.set_radius(0);
    style.set_border_width(0);
    //style.set_border_color(Color::from_rgb((255, 0, 0)));
    style.set_pad_top(0);
    style.set_pad_bottom(0);
    style.set_pad_left(0);
    style.set_pad_right(0);

    Box::new(style)
}

fn btn_matrix_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_border_width(0);
    style.set_shadow_width(0);
    style.set_bg_opa(Opacity::OPA_TRANSP);
    style.set_pad_top(2);
    style.set_pad_bottom(2);
    style.set_pad_left(2);
    style.set_pad_right(2);

    Box::new(style)
}

fn app_title_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((255, 215, 0))); // gold
    style.set_text_align(TextAlign::Right);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_24) });

    Box::new(style)
}

fn app_source_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((255, 215, 0))); // gold
    style.set_text_align(TextAlign::Right);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_12) });

    Box::new(style)
}
