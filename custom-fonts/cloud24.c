/*******************************************************************************
 * Size: 24 px
 * Bpp: 2
 * Opts: 
 ******************************************************************************/

#ifdef LV_LVGL_H_INCLUDE_SIMPLE
#include "lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif

#ifndef CLOUD24
#define CLOUD24 1
#endif

#if CLOUD24

/*-----------------
 *    BITMAPS
 *----------------*/

/*Store the image of the glyphs*/
static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {
    /* U+F0C2 "" */
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x7, 0xff, 0xd0, 0x0, 0x0, 0x0, 0x0,
    0x2, 0xff, 0xff, 0x80, 0x0, 0x0, 0x0, 0x0,
    0xff, 0xff, 0xff, 0x0, 0x0, 0x0, 0x0, 0x1f,
    0xff, 0xff, 0xfb, 0xfe, 0x0, 0x0, 0x3, 0xff,
    0xff, 0xff, 0xff, 0xf8, 0x0, 0x0, 0x7f, 0xff,
    0xff, 0xff, 0xff, 0xc0, 0x0, 0x7, 0xff, 0xff,
    0xff, 0xff, 0xfd, 0x0, 0x0, 0xbf, 0xff, 0xff,
    0xff, 0xff, 0xd0, 0x0, 0x7f, 0xff, 0xff, 0xff,
    0xff, 0xfd, 0x0, 0x1f, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xf8, 0x3, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xf4, 0xbf, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfb,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x3f,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xd1, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xf8, 0x3, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xfe, 0x0, 0x6, 0xaa,
    0xaa, 0xaa, 0xaa, 0xaa, 0x0
};


/*---------------------
 *  GLYPH DESCRIPTION
 *--------------------*/

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 480, .box_w = 30, .box_h = 22, .ofs_x = 0, .ofs_y = -2}
};

/*---------------------
 *  CHARACTER MAPPING
 *--------------------*/



/*Collect the unicode lists and glyph_id offsets*/
static const lv_font_fmt_txt_cmap_t cmaps[] =
{
    {
        .range_start = 61634, .range_length = 1, .glyph_id_start = 1,
        .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0, .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY
    }
};



/*--------------------
 *  ALL CUSTOM DATA
 *--------------------*/

#if LVGL_VERSION_MAJOR == 8
/*Store all the custom data of the font*/
static  lv_font_fmt_txt_glyph_cache_t cache;
#endif

#if LVGL_VERSION_MAJOR >= 8
static const lv_font_fmt_txt_dsc_t font_dsc = {
#else
static lv_font_fmt_txt_dsc_t font_dsc = {
#endif
    .glyph_bitmap = glyph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 1,
    .bpp = 2,
    .kern_classes = 0,
    .bitmap_format = 0,
#if LVGL_VERSION_MAJOR == 8
    .cache = &cache
#endif
};


/*-----------------
 *  PUBLIC FONT
 *----------------*/

/*Initialize a public general font descriptor*/
#if LVGL_VERSION_MAJOR >= 8
const lv_font_t cloud24 = {
#else
lv_font_t cloud24 = {
#endif
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,    /*Function pointer to get glyph's data*/
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,    /*Function pointer to get glyph's bitmap*/
    .line_height = 22,          /*The maximum line height required by the font*/
    .base_line = 2,             /*Baseline measured from the bottom of the line*/
#if !(LVGL_VERSION_MAJOR == 6 && LVGL_VERSION_MINOR == 0)
    .subpx = LV_FONT_SUBPX_NONE,
#endif
#if LV_VERSION_CHECK(7, 4, 0) || LVGL_VERSION_MAJOR >= 8
    .underline_position = -9,
    .underline_thickness = 1,
#endif
    .dsc = &font_dsc,          /*The custom font data. Will be accessed by `get_glyph_bitmap/dsc` */
    .fallback = NULL,
    .user_data = NULL
};



#endif /*#if CLOUD24*/

