//============================================================================
//                           Wifi Settings Pane
//============================================================================
use crate::lvgl_misc;
use cstr_core::CString;
use lvgl::font::Font;
use lvgl::style::Style;
use lvgl::widgets::{Btn, Keyboard, Label, Textarea};
use lvgl::{Align, Color, NativeObject, Obj, Part, Screen, TextAlign, Widget};
use lvgl_sys::*;

pub struct WifiSettingsPane<'a> {
    pane: Obj<'a>,
    pub keyboard: Keyboard<'a>,
    pub ssid_ta: Textarea<'a>,
    pub pswd_ta: Textarea<'a>,
    pub error_message: Label<'a>,
    pub edit_btn: Btn<'a>,
    pub save_btn: Btn<'a>,
    pub exit_btn: Btn<'a>,
    ssid: String,
    pswd: String,
}

impl<'a> WifiSettingsPane<'a> {
    pub fn new(wifi_settings_pane_parent: &'a mut Screen) -> Self {
        let mut pane = Obj::create(wifi_settings_pane_parent).unwrap();
        pane.set_size(800, 429);
        pane.set_align(Align::TopLeft, 0, 51);
        pane.add_style(Part::Main, Box::leak(settings_pane_style()));

        let ssid = String::from("");
        let pswd = String::from("");

        // SSID title
        let mut label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 70, 10);
        label.add_style(Part::Main, Box::leak(font_12_color_white_style()));
        let mut text = CString::new("Wifi Network Name").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        // Password title
        label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 260, 10);
        label.add_style(Part::Main, Box::leak(font_12_color_white_style()));
        text = CString::new("Password").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        // Row title
        label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 10, 30);
        label.add_style(Part::Main, Box::leak(font_12_color_yellow_style()));
        text = CString::new("Settings").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        // SSID text area
        let mut ssid_ta = Textarea::create(&mut pane).unwrap();
        let _ = ssid_ta.set_one_line(true);
        ssid_ta.set_width(160);
        ssid_ta.add_style(Part::Main, Box::leak(text_area_style()));
        ssid_ta.set_align(Align::TopLeft, 70, 30);

        // Password text area
        let mut pswd_ta = Textarea::create(&mut pane).unwrap();
        let _ = pswd_ta.set_one_line(true);
        pswd_ta.set_width(160);
        pswd_ta.add_style(Part::Main, Box::leak(text_area_style()));
        pswd_ta.set_align(Align::TopLeft, 260, 30);

        // Error message
        let mut error_message = Label::create(&mut pane).unwrap();
        error_message.add_style(Part::Main, Box::leak(error_message_style()));
        error_message.set_width(740);
        error_message.set_align(Align::TopLeft, 10, 182);
        text = CString::new("").unwrap();
        error_message.set_text(text.as_c_str()).unwrap();

        // Edit button
        let mut edit_btn = Btn::create(&mut pane).unwrap();
        edit_btn.set_size(70, 40);
        edit_btn.add_style(Part::Main, Box::leak(button_style()));
        edit_btn.set_align(Align::TopRight, -50, 30);
        let mut btn_label = Label::create(&mut edit_btn).unwrap();
        btn_label.set_align(Align::Center, 0, 0);
        text = CString::new("Edit").unwrap();
        btn_label.set_text(text.as_c_str()).unwrap();

        // Save button
        let mut save_btn = Btn::create(&mut pane).unwrap();
        save_btn.set_size(70, 40);
        save_btn.add_style(Part::Main, Box::leak(button_style()));
        save_btn.set_align(Align::TopRight, -50, 80);
        btn_label = Label::create(&mut save_btn).unwrap();
        btn_label.set_align(Align::Center, 0, 0);
        text = CString::new("Save").unwrap();
        btn_label.set_text(text.as_c_str()).unwrap();

        // Exit button
        let mut exit_btn = Btn::create(&mut pane).unwrap();
        exit_btn.set_size(70, 40);
        exit_btn.add_style(Part::Main, Box::leak(button_style()));
        exit_btn.set_align(Align::TopRight, -50, 130);
        btn_label = Label::create(&mut exit_btn).unwrap();
        btn_label.set_align(Align::Center, 0, 0);
        text = CString::new("Exit").unwrap();
        btn_label.set_text(text.as_c_str()).unwrap();

        // Create keyboard
        let mut keyboard = Keyboard::create(&mut pane).unwrap();
        keyboard.set_size(740, 200);
        keyboard.set_align(Align::Center, 0, 100);
        keyboard.add_style(Part::Main, Box::leak(keyboard_style()));

        Self {
            pane,
            keyboard,
            ssid_ta,
            pswd_ta,
            error_message,
            edit_btn,
            save_btn,
            exit_btn,
            ssid,
            pswd,
        }
    }

    pub fn show_keyboard(&mut self) {
        unsafe {
            lv_obj_clear_flag(
                self.keyboard.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
                lvgl_sys::LV_OBJ_FLAG_HIDDEN,
            );
        }
    }

    pub fn hide_keyboard(&mut self) {
        unsafe {
            lv_obj_add_flag(
                self.keyboard.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
                lvgl_sys::LV_OBJ_FLAG_HIDDEN,
            );
        }
    }

    pub fn show(&mut self) {
        self.disable_editing();
        //self.clear_error_message();
        lvgl_misc::show_obj(&mut self.pane);
    }

    pub fn hide(&mut self) {
        self.reset_wifi_creds_entries();
        lvgl_misc::hide_obj(&mut self.pane);
    }

    pub fn disable_editing(&mut self) {
        lvgl_misc::hide_button(&mut self.save_btn);
        lvgl_misc::hide_keyboard(&mut self.keyboard);
        lvgl_misc::disable_textarea(&mut self.ssid_ta);
        lvgl_misc::disable_textarea(&mut self.pswd_ta);
    }
    pub fn enable_editing(&mut self) {
        lvgl_misc::show_button(&mut self.save_btn);
        lvgl_misc::show_keyboard(&mut self.keyboard);
        lvgl_misc::enable_textarea(&mut self.ssid_ta);
        lvgl_misc::enable_textarea(&mut self.pswd_ta);
    }

    pub fn set_wifi_credentials(&mut self, ssid: String, pswd: String) {
        self.ssid = ssid.clone();
        self.pswd = pswd.clone();

        self.ssid_ta
            .set_text(CString::new(ssid.as_str()).unwrap().as_c_str())
            .unwrap();

        self.pswd_ta
            .set_text(CString::new(pswd.as_str()).unwrap().as_c_str())
            .unwrap();
    }

    pub fn get_user_wifi_creds_entries(&mut self) -> (String, String) {
        let ssid = lvgl_misc::get_textarea_string(&self.ssid_ta);
        let pswd = lvgl_misc::get_textarea_string(&self.pswd_ta);

        // Save user settings locally
        self.ssid = ssid.clone();
        self.pswd = pswd.clone();

        (ssid, pswd)
    }

    fn reset_wifi_creds_entries(&mut self) {
        self.ssid_ta
            .set_text(CString::new(self.ssid.as_str()).unwrap().as_c_str())
            .unwrap();

        self.pswd_ta
            .set_text(CString::new(self.pswd.as_str()).unwrap().as_c_str())
            .unwrap();
    }

    pub fn set_error_message(&mut self, msg: String) {
        self.error_message
            .set_text(CString::new(msg.as_str()).unwrap().as_c_str())
            .unwrap();
    }

    pub fn clear_error_message(&mut self) {
        self.error_message
            .set_text(CString::new("").unwrap().as_c_str())
            .unwrap();
    }
}

//*****************************************************************************
//                      Sytles for Wifi Settings Pane
//*****************************************************************************

fn settings_pane_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_bg_color(Color::from_rgb((0, 0, 0))); // black
    style.set_radius(0);
    style.set_border_width(0);
    style.set_pad_top(4);
    style.set_pad_bottom(4);
    style.set_pad_left(12);
    style.set_pad_right(12);

    Box::new(style)
}

fn text_area_style() -> Box<Style> {
    let mut style = Style::default();
    //style.set_bg_color(Color::from_rgb((0, 0, 0)));
    style.set_radius(0);
    style.set_border_width(0);
    style.set_pad_top(4);
    style.set_pad_bottom(4);
    style.set_pad_left(4);
    style.set_pad_right(4);

    Box::new(style)
}

fn keyboard_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 0))); // black
    style.set_bg_color(Color::from_rgb((0, 0, 139))); // navy blue
    style.set_text_align(TextAlign::Right);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_16) });

    Box::new(style)
}

fn font_12_color_white_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((255, 255, 255))); // white
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_12) });

    Box::new(style)
}

fn font_12_color_yellow_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((255, 215, 0))); // gold
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_12) });

    Box::new(style)
}

fn button_style() -> Box<Style> {
    let mut style = Style::default();
    //style.set_text_color(Color::from_rgb((255, 255, 255))); // white
    style.set_text_color(Color::from_rgb((0, 0, 0))); // black
    style.set_text_align(TextAlign::Center);
    style.set_bg_color(Color::from_rgb((30, 144, 255))); // dodger blue 1
    style.set_radius(6);
    style.set_border_width(0);
    style.set_shadow_width(0);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_12) });

    Box::new(style)
}

fn error_message_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((255, 0, 0))); // red
    style.set_text_align(TextAlign::Left);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_14) });

    Box::new(style)
}
