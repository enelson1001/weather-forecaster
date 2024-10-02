// Helper functions for lv binding rust

use core::mem;
use cstr_core::CStr;
use cstr_core::CString;
use lvgl::widgets::{Btn, Btnmatrix, Dropdown, Keyboard, Label, Textarea};
use lvgl::NativeObject;
use lvgl::{Color, Obj};
use lvgl_sys::*;

// ---------- Object Functions ----------
pub fn hide_obj(obj: &mut Obj) {
    unsafe {
        lv_obj_add_flag(
            obj.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_HIDDEN,
        );
    }
}

pub fn show_obj(obj: &mut Obj) {
    unsafe {
        lv_obj_clear_flag(
            obj.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_HIDDEN,
        );
    }
}

// ---------- Keyboard Functions ----------
pub fn hide_keyboard(kybd: &mut Keyboard) {
    unsafe {
        lv_obj_add_flag(
            kybd.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_HIDDEN,
        );
    }
}

pub fn show_keyboard(kybd: &mut Keyboard) {
    unsafe {
        lv_obj_clear_flag(
            kybd.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_HIDDEN,
        );
    }
}

// ---------- Button Functions ----------
pub fn hide_button(btn: &mut Btn) {
    unsafe {
        lv_obj_add_flag(
            btn.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_HIDDEN,
        );
    }
}

pub fn show_button(btn: &mut Btn) {
    unsafe {
        lv_obj_clear_flag(
            btn.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_HIDDEN,
        );
    }
}

pub fn set_btn_bg_color(btn: &mut Btn, color: Color) {
    unsafe {
        lv_obj_set_style_bg_color(
            btn.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            color.into(),
            lvgl_sys::LV_PART_MAIN,
        );
    }
}

// ---------- Textarea Functions ----------
pub fn get_textarea_string(ta: &Textarea) -> String {
    unsafe {
        let ptr = lv_textarea_get_text(ta.raw().as_ptr() as *const lvgl_sys::lv_obj_t);
        String::from(CStr::from_ptr(ptr).to_str().unwrap())
    }
}

pub fn enable_textarea(ta: &mut Textarea) {
    unsafe {
        lv_obj_add_flag(
            ta.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_CLICKABLE,
        );
    }
}

pub fn disable_textarea(ta: &mut Textarea) {
    unsafe {
        lv_obj_clear_flag(
            ta.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_CLICKABLE,
        );
    }
}

// ---------- Label Functions ----------
pub fn set_label_bg_color(label: &mut Label, color: Color) {
    unsafe {
        lv_obj_set_style_bg_color(
            label.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            color.into(),
            lvgl_sys::LV_PART_MAIN,
        );
    }
}

// ---------- Dropdown Functions ----------
pub fn get_dropdown_selected_str(dd: &Dropdown) -> String {
    let mut buf: [u8; 32] = [0; 32];
    unsafe {
        lv_dropdown_get_selected_str(
            dd.raw().as_ptr() as *const lvgl_sys::lv_obj_t,
            buf.as_mut_ptr() as *mut i8,
            buf.len().try_into().unwrap(),
        )
    }

    std::str::from_utf8(&buf)
        .unwrap()
        .trim_end_matches('\0')
        .into()
}

pub fn set_dropdown_selected_item(dd: &Dropdown, id: u16) {
    unsafe {
        lv_dropdown_set_selected(dd.raw().as_mut() as *mut lvgl_sys::lv_obj_t, id);
    }
}

pub fn enable_dropdown(dd: &mut Dropdown) {
    unsafe {
        lv_obj_add_flag(
            dd.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_CLICKABLE,
        );
    }
}

pub fn disable_dropdown(dd: &mut Dropdown) {
    unsafe {
        lv_obj_clear_flag(
            dd.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_OBJ_FLAG_CLICKABLE,
        );
    }
}

// ---------- Button Matrix Functions ----------
pub fn btnmatrix_set_map(btnm: &mut Btnmatrix, btn_matrix_vec: Vec<CString>) {
    // Turning each null-terminated string into a pointer.
    // `into_raw` takes ownershop, gives us the pointer and does NOT drop the data.
    let mut btnmatrix_map = btn_matrix_vec
        .into_iter()
        .map(|s| s.into_raw())
        .collect::<Vec<_>>();

    // Make sure we're not wasting space.
    btnmatrix_map.shrink_to_fit();
    assert!(btnmatrix_map.len() == btnmatrix_map.capacity());

    // Get pointer to the btnmatrix map
    let ptr = btnmatrix_map.as_mut_ptr();

    // Tell Rust not to free up the memory for btnmatrix_map
    mem::forget(btnmatrix_map);

    unsafe {
        lv_btnmatrix_set_map(
            btnm.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            ptr as *mut *const i8,
        );
    }
}

pub fn btnmatrix_set_btn_ctrl_all(btnm: &mut Btnmatrix) {
    unsafe {
        lv_btnmatrix_set_btn_ctrl_all(
            btnm.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            lvgl_sys::LV_BTNMATRIX_CTRL_CHECKABLE.try_into().unwrap(),
        );
    }
}

pub fn btnmatrix_set_btn_ctrl(btnm: &mut Btnmatrix, btn_id: u16) {
    unsafe {
        lv_btnmatrix_set_btn_ctrl(
            btnm.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            btn_id,
            lvgl_sys::LV_BTNMATRIX_CTRL_CHECKED.try_into().unwrap(),
        );
    }
}

pub fn btnmatrix_clear_btn_ctrl(btnm: &mut Btnmatrix, btn_id: u16) {
    unsafe {
        lv_btnmatrix_clear_btn_ctrl(
            btnm.raw().as_mut() as *mut lvgl_sys::lv_obj_t,
            btn_id,
            lvgl_sys::LV_BTNMATRIX_CTRL_CHECKABLE.try_into().unwrap(),
        );
    }
}

pub fn btnmatrix_set_one_checked(btnm: &mut Btnmatrix) {
    unsafe {
        lv_btnmatrix_set_one_checked(btnm.raw().as_mut() as *mut lvgl_sys::lv_obj_t, true);
    }
}

pub fn btnmatrix_get_selected_btn(btnm: &Btnmatrix) -> u16 {
    unsafe { lv_btnmatrix_get_selected_btn(btnm.raw().as_ptr() as *const lvgl_sys::lv_obj_t) }
}
