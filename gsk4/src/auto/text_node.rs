// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "GskTextNode")]
    pub struct TextNode(Shared<ffi::GskTextNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for TextNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_text_node_get_type()) }
    }
}

impl TextNode {
    #[doc(alias = "gsk_text_node_new")]
    pub fn new(
        font: &impl IsA<pango::Font>,
        glyphs: &pango::GlyphString,
        color: &gdk::RGBA,
        offset: &graphene::Point,
    ) -> Option<TextNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_text_node_new(
                font.as_ref().to_glib_none().0,
                mut_override(glyphs.to_glib_none().0),
                color.to_glib_none().0,
                offset.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_text_node_get_color")]
    #[doc(alias = "get_color")]
    pub fn color(&self) -> gdk::RGBA {
        unsafe { from_glib_none(ffi::gsk_text_node_get_color(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_text_node_get_font")]
    #[doc(alias = "get_font")]
    pub fn font(&self) -> pango::Font {
        unsafe { from_glib_none(ffi::gsk_text_node_get_font(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_text_node_get_glyphs")]
    #[doc(alias = "get_glyphs")]
    pub fn glyphs(&self) -> Vec<pango::GlyphInfo> {
        unsafe {
            let mut n_glyphs = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gsk_text_node_get_glyphs(self.to_glib_none().0, n_glyphs.as_mut_ptr()),
                n_glyphs.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gsk_text_node_get_num_glyphs")]
    #[doc(alias = "get_num_glyphs")]
    pub fn num_glyphs(&self) -> u32 {
        unsafe { ffi::gsk_text_node_get_num_glyphs(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_text_node_get_offset")]
    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> graphene::Point {
        unsafe { from_glib_none(ffi::gsk_text_node_get_offset(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gsk_text_node_has_color_glyphs")]
    pub fn has_color_glyphs(&self) -> bool {
        unsafe { from_glib(ffi::gsk_text_node_has_color_glyphs(self.to_glib_none().0)) }
    }
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextNode")
    }
}
