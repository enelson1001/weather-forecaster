//============================================================================
//                          Cities Settings Pane
//============================================================================
use crate::lvgl_misc;
use crate::model::CityInfo;
use cstr_core::CString;
use lvgl::font::Font;
use lvgl::style::Style;
use lvgl::widgets::{Btn, Dropdown, Keyboard, Label, Textarea};
use lvgl::{Align, Color, Obj, Part, Screen, TextAlign, Widget};
use lvgl_sys::*;

const STATES: &str = "AL\nAK\nAZ\nAR\nCA\nCO\nCT\nDE\nDC\nFL\nGA\nGU\nHI\nID\nIL\nIN\n\
    IA\nKS\nKY\nLA\nMD\nMA\nMI\nMN\nMS\nMO\nMT\nNE\nNV\nNH\nNJ\nNM\nNY\nNC\nND\nOH\nOK\n\
    OR\nPA\nPR\nRI\nSC\nSD\nTN\nTX\nUT\nVT\nVA\nVI\nWA\nWV\nWI\nWY";

const TIMEZONES: &str =
    "US/Alaska\nUS/Aleutian\nUS/Arizona\nUS/Central\nUS/EastIndiana\nUS/Eastern\n\
US/Hawaii\nUS/IndianaStarke\nUS/Michigan\nUS/Mountain\nUS/Pacific\nUS/Samoa";

pub struct CityWidgets<'a> {
    pub city_name: Textarea<'a>,
    pub state: Dropdown<'a>,
    pub zipcode: Textarea<'a>,
    pub timezone: Dropdown<'a>,
}

pub struct CitiesSettingsPane<'a> {
    pane: Obj<'a>,
    pub keyboard: Keyboard<'a>,
    pub cities_widgets: Vec<CityWidgets<'a>>,
    pub edit_btn: Btn<'a>,
    pub save_btn: Btn<'a>,
    pub exit_btn: Btn<'a>,
    cities_info: Vec<CityInfo>,
    error_message: Label<'a>,
}

impl<'a> CitiesSettingsPane<'a> {
    pub fn new(cities_settings_pane_parent: &'a mut Screen) -> Self {
        let mut pane = Obj::create(cities_settings_pane_parent).unwrap();
        pane.set_size(800, 429);
        pane.set_align(Align::TopLeft, 0, 51);
        pane.add_style(Part::Main, Box::leak(settings_pane_style()));

        let mut cities_widgets: Vec<CityWidgets> = Vec::new();
        let cities_info: Vec<CityInfo> = Vec::new();

        // Name title
        let mut label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 60, 10);
        label.add_style(Part::Main, Box::leak(font_12_color_white_style()));
        let mut text = CString::new("Name").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        // State title
        label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 250, 10);
        label.add_style(Part::Main, Box::leak(font_12_color_white_style()));
        text = CString::new("State").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        // Zipcode title
        label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 350, 10);
        label.add_style(Part::Main, Box::leak(font_12_color_white_style()));
        text = CString::new("Zipcode").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        // Timezone title
        label = Label::create(&mut pane).unwrap();
        label.set_align(Align::TopLeft, 470, 10);
        label.add_style(Part::Main, Box::leak(font_12_color_white_style()));
        text = CString::new("Timezone").unwrap();
        label.set_text(text.as_c_str()).unwrap();

        const ROW_SPACING: i32 = 40;

        for i in 0..=3 {
            // Row title
            label = Label::create(&mut pane).unwrap();
            label.set_align(Align::TopLeft, 10, 30 + (i * ROW_SPACING));
            label.add_style(Part::Main, Box::leak(font_12_color_yellow_style()));
            let row_title = format!("City {}:", i + 1);
            text = CString::new(row_title.as_str()).unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // City name text area
            let mut city_name = Textarea::create(&mut pane).unwrap();
            let _ = city_name.set_one_line(true);
            city_name.set_width(160);
            city_name.add_style(Part::Main, Box::leak(text_area_style()));
            city_name.set_align(Align::TopLeft, 60, 30 + (i * ROW_SPACING));

            // State dropdown
            let mut state = Dropdown::create(&mut pane).unwrap();
            let state_txt = CString::new(STATES).unwrap();
            let _ = state.set_options(state_txt.as_c_str());
            state.add_style(Part::Main, Box::leak(text_area_style()));
            state.set_width(80);
            state.set_align(Align::TopLeft, 250, 30 + (i * ROW_SPACING));

            // Zipcode text area
            let mut zipcode = Textarea::create(&mut pane).unwrap();
            let _ = zipcode.set_one_line(true);
            zipcode.add_style(Part::Main, Box::leak(text_area_style()));
            zipcode.set_width(100);
            zipcode.set_align(Align::TopLeft, 350, 30 + (i * ROW_SPACING));

            // Timezone dropdown
            let mut timezone = Dropdown::create(&mut pane).unwrap();
            let timezone_txt = CString::new(TIMEZONES).unwrap();
            let _ = timezone.set_options(timezone_txt.as_c_str());
            timezone.add_style(Part::Main, Box::leak(text_area_style()));
            timezone.set_align(Align::TopLeft, 470, 30 + (i * ROW_SPACING));

            cities_widgets.push(CityWidgets {
                city_name,
                state,
                zipcode,
                timezone,
            })
        }

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
            cities_widgets,
            edit_btn,
            save_btn,
            exit_btn,
            cities_info,
            error_message,
        }
    }

    pub fn show(&mut self) {
        self.clear_error_message();
        self.prepare_cities_settings();
        self.disable_editing();
        lvgl_misc::show_obj(&mut self.pane);
    }

    pub fn hide(&mut self) {
        lvgl_misc::hide_obj(&mut self.pane);
    }

    pub fn disable_editing(&mut self) {
        lvgl_misc::hide_button(&mut self.save_btn);
        lvgl_misc::hide_keyboard(&mut self.keyboard);
        for i in 0..4 {
            lvgl_misc::disable_textarea(&mut self.cities_widgets[i].city_name);
            lvgl_misc::disable_dropdown(&mut self.cities_widgets[i].state);
            lvgl_misc::disable_textarea(&mut self.cities_widgets[i].zipcode);
            lvgl_misc::disable_dropdown(&mut self.cities_widgets[i].timezone);
        }
    }
    pub fn enable_editing(&mut self) {
        lvgl_misc::show_button(&mut self.save_btn);
        lvgl_misc::show_keyboard(&mut self.keyboard);
        for i in 0..4 {
            lvgl_misc::enable_textarea(&mut self.cities_widgets[i].city_name);
            lvgl_misc::enable_dropdown(&mut self.cities_widgets[i].state);
            lvgl_misc::enable_textarea(&mut self.cities_widgets[i].zipcode);
            lvgl_misc::enable_dropdown(&mut self.cities_widgets[i].timezone);
        }
    }

    pub fn set_cities_settings(&mut self, cities_info: Vec<CityInfo>) {
        self.cities_info = cities_info.clone();
    }

    pub fn get_user_cities_settings(&mut self) -> Option<Vec<CityInfo>> {
        let mut cities_info: Vec<CityInfo> = Vec::new();
        for i in 0..4 {
            let city_name = lvgl_misc::get_textarea_string(&self.cities_widgets[i].city_name);
            let state = lvgl_misc::get_dropdown_selected_str(&self.cities_widgets[i].state);
            let zipcode = lvgl_misc::get_textarea_string(&self.cities_widgets[i].zipcode);
            let timezone = lvgl_misc::get_dropdown_selected_str(&self.cities_widgets[i].timezone);
            cities_info.push(CityInfo {
                city_name,
                state,
                zipcode,
                timezone,
            });
        }

        match check_for_errors(cities_info.clone()) {
            Ok(()) => {
                self.cities_info = cities_info.clone();
                self.clear_error_message();
                Some(cities_info)
            }
            Err(error) => {
                self.set_error_message(error);
                None
            }
        }
    }

    fn prepare_cities_settings(&mut self) {
        for i in 0..self.cities_info.len() {
            self.cities_widgets[i]
                .city_name
                .set_text(
                    CString::new(self.cities_info[i].city_name.as_str())
                        .unwrap()
                        .as_c_str(),
                )
                .unwrap();

            let p = find_state_position(STATES, self.cities_info[i].state.as_str());
            lvgl_misc::set_dropdown_selected_item(&self.cities_widgets[i].state, p);

            self.cities_widgets[i]
                .zipcode
                .set_text(
                    CString::new(self.cities_info[i].zipcode.as_str())
                        .unwrap()
                        .as_c_str(),
                )
                .unwrap();

            let p = find_timezone_position(TIMEZONES, self.cities_info[i].timezone.as_str());
            lvgl_misc::set_dropdown_selected_item(&self.cities_widgets[i].timezone, p);
        }
    }

    fn set_error_message(&mut self, msg: String) {
        self.error_message
            .set_text(CString::new(msg.as_str()).unwrap().as_c_str())
            .unwrap();
    }

    fn clear_error_message(&mut self) {
        self.error_message
            .set_text(CString::new("").unwrap().as_c_str())
            .unwrap();
    }
}

fn check_for_errors(cities_info: Vec<CityInfo>) -> Result<(), String> {
    for (i, city) in cities_info.iter().enumerate() {
        let zipcode = city.zipcode.as_str();
        is_zipcode_valid(zipcode).map_err(|e| format!("City {} - {}", i + 1, e))?;
    }

    core::result::Result::Ok(())
}

fn is_zipcode_valid(city_zipcode: &str) -> Result<(), String> {
    // 1st check if zipcode is a number
    // 2nd check if zipcode has 5 digits
    // 3rd check if zipcode is within zipcode numbers assigned by USPS
    let zipcode_vec: Vec<char> = city_zipcode.chars().collect();
    if let Ok(zipcode) = city_zipcode.to_string().parse::<u32>() {
        if zipcode_vec.len() == 5 {
            if let 501..=99950 = zipcode {
                Ok(())
            } else {
                Err("Zipcode must be from 00501 to 99950".to_string())
            }
        } else {
            Err("Zipcode must be 5 digits".to_string())
        }
    } else {
        Err("Zipcode is not a number".to_string())
    }
}

fn find_state_position(states: &str, state: &str) -> u16 {
    // Finds the position of the state is in states string, if state is not found return position 0
    // Then divide position by 3 because every state word is 3 bytes long (two bytes for state, one byte for \n)
    let position: u16 = states.find(state).unwrap_or(0usize).try_into().unwrap();
    position / 3
}

fn find_timezone_position(timezones: &str, timezone: &str) -> u16 {
    let tz_vec: Vec<&str> = timezones.split('\n').collect();
    tz_vec
        .iter()
        .position(|&tz| tz == timezone)
        .unwrap_or(0usize)
        .try_into()
        .unwrap()
}

//*****************************************************************************
//                Sytles for Cities Settings Pane
//*****************************************************************************

fn settings_pane_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_bg_color(Color::from_rgb((0, 0, 0)));
    style.set_radius(0);
    style.set_border_width(0);
    //style.set_border_color(Color::from_rgb((0, 255, 0)));
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
