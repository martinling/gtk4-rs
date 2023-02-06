// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::{fmt, mem};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ContentFormats(Shared<ffi::GdkContentFormats>);

    match fn {
        ref => |ptr| ffi::gdk_content_formats_ref(ptr),
        unref => |ptr| ffi::gdk_content_formats_unref(ptr),
        type_ => || ffi::gdk_content_formats_get_type(),
    }
}

impl ContentFormats {
    #[doc(alias = "gdk_content_formats_new")]
    pub fn new(mime_types: &[&str]) -> ContentFormats {
        assert_initialized_main_thread!();
        let n_mime_types = mime_types.len() as _;
        unsafe {
            from_glib_full(ffi::gdk_content_formats_new(
                mime_types.to_glib_none().0,
                n_mime_types,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_new_for_gtype")]
    #[doc(alias = "new_for_gtype")]
    pub fn for_type(type_: glib::types::Type) -> ContentFormats {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_content_formats_new_for_gtype(type_.into_glib())) }
    }

    #[doc(alias = "gdk_content_formats_contain_gtype")]
    #[doc(alias = "contain_gtype")]
    pub fn contains_type(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_contain_gtype(
                self.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_contain_mime_type")]
    pub fn contain_mime_type(&self, mime_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_contain_mime_type(
                self.to_glib_none().0,
                mime_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_get_mime_types")]
    #[doc(alias = "get_mime_types")]
    pub fn mime_types(&self) -> Vec<glib::GString> {
        unsafe {
            let mut n_mime_types = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gdk_content_formats_get_mime_types(
                    self.to_glib_none().0,
                    n_mime_types.as_mut_ptr(),
                ),
                n_mime_types.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "gdk_content_formats_match")]
    #[doc(alias = "match")]
    pub fn match_(&self, second: &ContentFormats) -> bool {
        unsafe {
            from_glib(ffi::gdk_content_formats_match(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_match_gtype")]
    #[doc(alias = "match_gtype")]
    pub fn match_type(&self, second: &ContentFormats) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gdk_content_formats_match_gtype(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_match_mime_type")]
    pub fn match_mime_type(&self, second: &ContentFormats) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_content_formats_match_mime_type(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gdk_content_formats_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_content_formats_union")]
    #[must_use]
    pub fn union(self, second: &ContentFormats) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union(
                self.into_glib_ptr(),
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_deserialize_gtypes")]
    #[doc(alias = "union_deserialize_gtypes")]
    #[must_use]
    pub fn union_deserialize_types(self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_deserialize_gtypes(
                self.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_deserialize_mime_types")]
    #[must_use]
    pub fn union_deserialize_mime_types(self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_deserialize_mime_types(
                self.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_serialize_gtypes")]
    #[doc(alias = "union_serialize_gtypes")]
    #[must_use]
    pub fn union_serialize_types(self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_serialize_gtypes(
                self.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gdk_content_formats_union_serialize_mime_types")]
    #[must_use]
    pub fn union_serialize_mime_types(self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_formats_union_serialize_mime_types(
                self.into_glib_ptr(),
            ))
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_content_formats_parse")]
    pub fn parse(string: &str) -> Result<ContentFormats, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gdk_content_formats_parse(string.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Can't parse ContentFormats"))
        }
    }
}

impl fmt::Display for ContentFormats {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
