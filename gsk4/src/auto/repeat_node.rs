// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskRepeatNode")]
    pub struct RepeatNode(Shared<ffi::GskRepeatNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for RepeatNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_repeat_node_get_type()) }
    }
}

impl RepeatNode {
    #[doc(alias = "gsk_repeat_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        child: impl AsRef<RenderNode>,
        child_bounds: Option<&graphene::Rect>,
    ) -> RepeatNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_repeat_node_new(
                bounds.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                child_bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_repeat_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_repeat_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_repeat_node_get_child_bounds")]
    #[doc(alias = "get_child_bounds")]
    pub fn child_bounds(&self) -> graphene::Rect {
        unsafe { from_glib_none(ffi::gsk_repeat_node_get_child_bounds(self.to_glib_none().0)) }
    }
}

impl fmt::Display for RepeatNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepeatNode")
    }
}
