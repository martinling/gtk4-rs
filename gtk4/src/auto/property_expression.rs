// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Expression;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkPropertyExpression")]
    pub struct PropertyExpression(Shared<ffi::GtkPropertyExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression),
        unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
    }
}

impl glib::StaticType for PropertyExpression {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_property_expression_get_type()) }
    }
}

impl PropertyExpression {
    #[doc(alias = "gtk_property_expression_new")]
    pub fn new(
        this_type: glib::types::Type,
        expression: Option<impl AsRef<Expression>>,
        property_name: &str,
    ) -> PropertyExpression {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_property_expression_new(
                this_type.into_glib(),
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
                property_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_new_for_pspec")]
    #[doc(alias = "new_for_pspec")]
    pub fn for_pspec(
        expression: Option<impl AsRef<Expression>>,
        pspec: impl AsRef<glib::ParamSpec>,
    ) -> PropertyExpression {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_property_expression_new_for_pspec(
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
                pspec.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe {
            from_glib_none(ffi::gtk_property_expression_get_expression(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_property_expression_get_pspec")]
    #[doc(alias = "get_pspec")]
    pub fn pspec(&self) -> glib::ParamSpec {
        unsafe {
            from_glib_none(ffi::gtk_property_expression_get_pspec(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for PropertyExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PropertyExpression")
    }
}
