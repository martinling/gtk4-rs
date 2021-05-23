// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::IconLookupFlags;
use crate::IconPaintable;
use crate::TextDirection;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkIconTheme")]
    pub struct IconTheme(Object<ffi::GtkIconTheme>);

    match fn {
        type_ => || ffi::gtk_icon_theme_get_type(),
    }
}

impl IconTheme {
    #[doc(alias = "gtk_icon_theme_new")]
    pub fn new() -> IconTheme {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_icon_theme_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`IconTheme`] objects.
    ///
    /// This method returns an instance of [`IconThemeBuilder`] which can be used to create [`IconTheme`] objects.
    pub fn builder() -> IconThemeBuilder {
        IconThemeBuilder::default()
    }

    #[doc(alias = "gtk_icon_theme_add_resource_path")]
    pub fn add_resource_path(&self, path: &str) {
        unsafe {
            ffi::gtk_icon_theme_add_resource_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_icon_theme_add_search_path")]
    pub fn add_search_path(&self, path: impl AsRef<std::path::Path>) {
        unsafe {
            ffi::gtk_icon_theme_add_search_path(
                self.to_glib_none().0,
                path.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_icon_theme_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<gdk::Display> {
        unsafe { from_glib_none(ffi::gtk_icon_theme_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_icon_theme_get_icon_names")]
    #[doc(alias = "get_icon_names")]
    pub fn icon_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_theme_get_icon_names(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_get_resource_path")]
    #[doc(alias = "get_resource_path")]
    pub fn resource_path(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_theme_get_resource_path(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_get_search_path")]
    #[doc(alias = "get_search_path")]
    pub fn search_path(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_theme_get_search_path(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_get_theme_name")]
    #[doc(alias = "get_theme_name")]
    pub fn theme_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_icon_theme_get_theme_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gtk_icon_theme_has_gicon")]
    pub fn has_gicon(&self, gicon: &impl IsA<gio::Icon>) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_theme_has_gicon(
                self.to_glib_none().0,
                gicon.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_has_icon")]
    pub fn has_icon(&self, icon_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_theme_has_icon(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_lookup_by_gicon")]
    pub fn lookup_by_gicon(
        &self,
        icon: &impl IsA<gio::Icon>,
        size: i32,
        scale: i32,
        direction: TextDirection,
        flags: IconLookupFlags,
    ) -> Option<IconPaintable> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_lookup_by_gicon(
                self.to_glib_none().0,
                icon.as_ref().to_glib_none().0,
                size,
                scale,
                direction.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_lookup_icon")]
    pub fn lookup_icon(
        &self,
        icon_name: &str,
        fallbacks: &[&str],
        size: i32,
        scale: i32,
        direction: TextDirection,
        flags: IconLookupFlags,
    ) -> Option<IconPaintable> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_lookup_icon(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
                fallbacks.to_glib_none().0,
                size,
                scale,
                direction.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_icon_theme_set_resource_path")]
    pub fn set_resource_path(&self, path: &[&str]) {
        unsafe {
            ffi::gtk_icon_theme_set_resource_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_icon_theme_set_search_path")]
    pub fn set_search_path(&self, path: &[&std::path::Path]) {
        unsafe {
            ffi::gtk_icon_theme_set_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_icon_theme_set_theme_name")]
    pub fn set_theme_name(&self, theme_name: Option<&str>) {
        unsafe {
            ffi::gtk_icon_theme_set_theme_name(self.to_glib_none().0, theme_name.to_glib_none().0);
        }
    }

    pub fn set_display<P: IsA<gdk::Display>>(&self, display: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"display\0".as_ptr() as *const _,
                display.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_icon_theme_get_for_display")]
    #[doc(alias = "get_for_display")]
    pub fn for_display(display: &impl IsA<gdk::Display>) -> Option<IconTheme> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_icon_theme_get_for_display(
                display.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&IconTheme) + 'static>(
            this: *mut ffi::GtkIconTheme,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "display")]
    pub fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&IconTheme) + 'static>(
            this: *mut ffi::GtkIconTheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-names")]
    pub fn connect_icon_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_names_trampoline<F: Fn(&IconTheme) + 'static>(
            this: *mut ffi::GtkIconTheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_names_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resource-path")]
    pub fn connect_resource_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resource_path_trampoline<F: Fn(&IconTheme) + 'static>(
            this: *mut ffi::GtkIconTheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resource-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resource_path_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "search-path")]
    pub fn connect_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_path_trampoline<F: Fn(&IconTheme) + 'static>(
            this: *mut ffi::GtkIconTheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_path_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "theme-name")]
    pub fn connect_theme_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_theme_name_trampoline<F: Fn(&IconTheme) + 'static>(
            this: *mut ffi::GtkIconTheme,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::theme-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_theme_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for IconTheme {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`IconTheme`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct IconThemeBuilder {
    display: Option<gdk::Display>,
    resource_path: Option<Vec<String>>,
    search_path: Option<Vec<String>>,
    theme_name: Option<String>,
}

impl IconThemeBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`IconThemeBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`IconTheme`].
    pub fn build(self) -> IconTheme {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref resource_path) = self.resource_path {
            properties.push(("resource-path", resource_path));
        }
        if let Some(ref search_path) = self.search_path {
            properties.push(("search-path", search_path));
        }
        if let Some(ref theme_name) = self.theme_name {
            properties.push(("theme-name", theme_name));
        }
        glib::Object::new::<IconTheme>(&properties)
            .expect("Failed to create an instance of IconTheme")
    }

    pub fn display(mut self, display: &impl IsA<gdk::Display>) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }

    pub fn resource_path(mut self, resource_path: Vec<String>) -> Self {
        self.resource_path = Some(resource_path);
        self
    }

    pub fn search_path(mut self, search_path: Vec<String>) -> Self {
        self.search_path = Some(search_path);
        self
    }

    pub fn theme_name(mut self, theme_name: &str) -> Self {
        self.theme_name = Some(theme_name.to_string());
        self
    }
}

impl fmt::Display for IconTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IconTheme")
    }
}
