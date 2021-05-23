// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FontChooserLevel;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkFontChooser")]
    pub struct FontChooser(Interface<ffi::GtkFontChooser, ffi::GtkFontChooserIface>);

    match fn {
        type_ => || ffi::gtk_font_chooser_get_type(),
    }
}

pub const NONE_FONT_CHOOSER: Option<&FontChooser> = None;

pub trait FontChooserExt: 'static {
    #[doc(alias = "gtk_font_chooser_get_font")]
    #[doc(alias = "get_font")]
    fn font(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_font_desc")]
    #[doc(alias = "get_font_desc")]
    fn font_desc(&self) -> Option<pango::FontDescription>;

    #[doc(alias = "gtk_font_chooser_get_font_face")]
    #[doc(alias = "get_font_face")]
    fn font_face(&self) -> Option<pango::FontFace>;

    #[doc(alias = "gtk_font_chooser_get_font_family")]
    #[doc(alias = "get_font_family")]
    fn font_family(&self) -> Option<pango::FontFamily>;

    #[doc(alias = "gtk_font_chooser_get_font_features")]
    #[doc(alias = "get_font_features")]
    fn font_features(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_font_map")]
    #[doc(alias = "get_font_map")]
    fn font_map(&self) -> Option<pango::FontMap>;

    #[doc(alias = "gtk_font_chooser_get_font_size")]
    #[doc(alias = "get_font_size")]
    fn font_size(&self) -> i32;

    #[doc(alias = "gtk_font_chooser_get_language")]
    #[doc(alias = "get_language")]
    fn language(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_level")]
    #[doc(alias = "get_level")]
    fn level(&self) -> FontChooserLevel;

    #[doc(alias = "gtk_font_chooser_get_preview_text")]
    #[doc(alias = "get_preview_text")]
    fn preview_text(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_font_chooser_get_show_preview_entry")]
    #[doc(alias = "get_show_preview_entry")]
    fn shows_preview_entry(&self) -> bool;

    #[doc(alias = "gtk_font_chooser_set_filter_func")]
    fn set_filter_func<P: Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>(
        &self,
        filter: P,
    );

    #[doc(alias = "gtk_font_chooser_set_font")]
    fn set_font(&self, fontname: &str);

    #[doc(alias = "gtk_font_chooser_set_font_desc")]
    fn set_font_desc(&self, font_desc: &pango::FontDescription);

    #[doc(alias = "gtk_font_chooser_set_font_map")]
    fn set_font_map(&self, fontmap: Option<&impl IsA<pango::FontMap>>);

    #[doc(alias = "gtk_font_chooser_set_language")]
    fn set_language(&self, language: &str);

    #[doc(alias = "gtk_font_chooser_set_level")]
    fn set_level(&self, level: FontChooserLevel);

    #[doc(alias = "gtk_font_chooser_set_preview_text")]
    fn set_preview_text(&self, text: &str);

    #[doc(alias = "gtk_font_chooser_set_show_preview_entry")]
    fn set_show_preview_entry(&self, show_preview_entry: bool);

    #[doc(alias = "font-activated")]
    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "font")]
    fn connect_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "font-desc")]
    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "font-features")]
    fn connect_font_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "language")]
    fn connect_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "level")]
    fn connect_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "preview-text")]
    fn connect_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-preview-entry")]
    fn connect_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FontChooser>> FontChooserExt for O {
    fn font(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn font_face(&self) -> Option<pango::FontFace> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_face(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn font_family(&self) -> Option<pango::FontFamily> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn font_features(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_features(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn font_map(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_map(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn font_size(&self) -> i32 {
        unsafe { ffi::gtk_font_chooser_get_font_size(self.as_ref().to_glib_none().0) }
    }

    fn language(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn level(&self) -> FontChooserLevel {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preview_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_preview_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_preview_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_show_preview_entry(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_filter_func<P: Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static>(
        &self,
        filter: P,
    ) {
        let filter_data: Box_<P> = Box_::new(filter);
        unsafe extern "C" fn filter_func<
            P: Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static,
        >(
            family: *const pango::ffi::PangoFontFamily,
            face: *const pango::ffi::PangoFontFace,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let family = from_glib_borrow(family);
            let face = from_glib_borrow(face);
            let callback: &P = &*(data as *mut _);
            let res = (*callback)(&family, &face);
            res.into_glib()
        }
        let filter = Some(filter_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&pango::FontFamily, &pango::FontFace) -> bool + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = filter_data;
        unsafe {
            ffi::gtk_font_chooser_set_filter_func(
                self.as_ref().to_glib_none().0,
                filter,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_font(&self, fontname: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(
                self.as_ref().to_glib_none().0,
                fontname.to_glib_none().0,
            );
        }
    }

    fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            ffi::gtk_font_chooser_set_font_desc(
                self.as_ref().to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    fn set_font_map(&self, fontmap: Option<&impl IsA<pango::FontMap>>) {
        unsafe {
            ffi::gtk_font_chooser_set_font_map(
                self.as_ref().to_glib_none().0,
                fontmap.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_language(&self, language: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_language(
                self.as_ref().to_glib_none().0,
                language.to_glib_none().0,
            );
        }
    }

    fn set_level(&self, level: FontChooserLevel) {
        unsafe {
            ffi::gtk_font_chooser_set_level(self.as_ref().to_glib_none().0, level.into_glib());
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe {
            ffi::gtk_font_chooser_set_show_preview_entry(
                self.as_ref().to_glib_none().0,
                show_preview_entry.into_glib(),
            );
        }
    }

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn font_activated_trampoline<
            P: IsA<FontChooser>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::GtkFontChooser,
            fontname: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FontChooser::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(fontname),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"font-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    font_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_trampoline<P: IsA<FontChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<
            P: IsA<FontChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-desc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_desc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_font_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_features_trampoline<
            P: IsA<FontChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-features\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_features_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<
            P: IsA<FontChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::language\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_language_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_level_trampoline<P: IsA<FontChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_preview_text_trampoline<
            P: IsA<FontChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::preview-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_preview_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_preview_entry_trampoline<
            P: IsA<FontChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFontChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-preview-entry\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_preview_entry_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FontChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontChooser")
    }
}
