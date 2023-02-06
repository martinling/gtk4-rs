// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TransformCategory;
use glib::translate::*;
use std::{fmt, mem};

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Transform(Shared<ffi::GskTransform>);

    match fn {
        ref => |ptr| ffi::gsk_transform_ref(ptr),
        unref => |ptr| ffi::gsk_transform_unref(ptr),
        type_ => || ffi::gsk_transform_get_type(),
    }
}

impl Transform {
    #[doc(alias = "gsk_transform_new")]
    pub fn new() -> Transform {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gsk_transform_new()) }
    }

    #[doc(alias = "gsk_transform_equal")]
    fn equal(&self, second: &Transform) -> bool {
        unsafe {
            from_glib(ffi::gsk_transform_equal(
                self.to_glib_none().0,
                second.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_transform_get_category")]
    #[doc(alias = "get_category")]
    pub fn category(&self) -> TransformCategory {
        unsafe { from_glib(ffi::gsk_transform_get_category(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_transform_matrix")]
    #[must_use]
    pub fn matrix(self, matrix: &graphene::Matrix) -> Transform {
        unsafe {
            from_glib_full(ffi::gsk_transform_matrix(
                self.into_glib_ptr(),
                matrix.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_transform_perspective")]
    #[must_use]
    pub fn perspective(self, depth: f32) -> Transform {
        unsafe { from_glib_full(ffi::gsk_transform_perspective(self.into_glib_ptr(), depth)) }
    }

    #[doc(alias = "gsk_transform_to_2d")]
    pub fn to_2d(&self) -> (f32, f32, f32, f32, f32, f32) {
        unsafe {
            let mut out_xx = mem::MaybeUninit::uninit();
            let mut out_yx = mem::MaybeUninit::uninit();
            let mut out_xy = mem::MaybeUninit::uninit();
            let mut out_yy = mem::MaybeUninit::uninit();
            let mut out_dx = mem::MaybeUninit::uninit();
            let mut out_dy = mem::MaybeUninit::uninit();
            ffi::gsk_transform_to_2d(
                self.to_glib_none().0,
                out_xx.as_mut_ptr(),
                out_yx.as_mut_ptr(),
                out_xy.as_mut_ptr(),
                out_yy.as_mut_ptr(),
                out_dx.as_mut_ptr(),
                out_dy.as_mut_ptr(),
            );
            (
                out_xx.assume_init(),
                out_yx.assume_init(),
                out_xy.assume_init(),
                out_yy.assume_init(),
                out_dx.assume_init(),
                out_dy.assume_init(),
            )
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gsk_transform_to_2d_components")]
    pub fn to_2d_components(&self) -> (f32, f32, f32, f32, f32, f32, f32) {
        unsafe {
            let mut out_skew_x = mem::MaybeUninit::uninit();
            let mut out_skew_y = mem::MaybeUninit::uninit();
            let mut out_scale_x = mem::MaybeUninit::uninit();
            let mut out_scale_y = mem::MaybeUninit::uninit();
            let mut out_angle = mem::MaybeUninit::uninit();
            let mut out_dx = mem::MaybeUninit::uninit();
            let mut out_dy = mem::MaybeUninit::uninit();
            ffi::gsk_transform_to_2d_components(
                self.to_glib_none().0,
                out_skew_x.as_mut_ptr(),
                out_skew_y.as_mut_ptr(),
                out_scale_x.as_mut_ptr(),
                out_scale_y.as_mut_ptr(),
                out_angle.as_mut_ptr(),
                out_dx.as_mut_ptr(),
                out_dy.as_mut_ptr(),
            );
            (
                out_skew_x.assume_init(),
                out_skew_y.assume_init(),
                out_scale_x.assume_init(),
                out_scale_y.assume_init(),
                out_angle.assume_init(),
                out_dx.assume_init(),
                out_dy.assume_init(),
            )
        }
    }

    #[doc(alias = "gsk_transform_to_affine")]
    pub fn to_affine(&self) -> (f32, f32, f32, f32) {
        unsafe {
            let mut out_scale_x = mem::MaybeUninit::uninit();
            let mut out_scale_y = mem::MaybeUninit::uninit();
            let mut out_dx = mem::MaybeUninit::uninit();
            let mut out_dy = mem::MaybeUninit::uninit();
            ffi::gsk_transform_to_affine(
                self.to_glib_none().0,
                out_scale_x.as_mut_ptr(),
                out_scale_y.as_mut_ptr(),
                out_dx.as_mut_ptr(),
                out_dy.as_mut_ptr(),
            );
            (
                out_scale_x.assume_init(),
                out_scale_y.assume_init(),
                out_dx.assume_init(),
                out_dy.assume_init(),
            )
        }
    }

    #[doc(alias = "gsk_transform_to_matrix")]
    pub fn to_matrix(&self) -> graphene::Matrix {
        unsafe {
            let mut out_matrix = graphene::Matrix::uninitialized();
            ffi::gsk_transform_to_matrix(self.to_glib_none().0, out_matrix.to_glib_none_mut().0);
            out_matrix
        }
    }

    #[doc(alias = "gsk_transform_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gsk_transform_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_transform_to_translate")]
    pub fn to_translate(&self) -> (f32, f32) {
        unsafe {
            let mut out_dx = mem::MaybeUninit::uninit();
            let mut out_dy = mem::MaybeUninit::uninit();
            ffi::gsk_transform_to_translate(
                self.to_glib_none().0,
                out_dx.as_mut_ptr(),
                out_dy.as_mut_ptr(),
            );
            (out_dx.assume_init(), out_dy.assume_init())
        }
    }

    #[doc(alias = "gsk_transform_transform_bounds")]
    pub fn transform_bounds(&self, rect: &graphene::Rect) -> graphene::Rect {
        unsafe {
            let mut out_rect = graphene::Rect::uninitialized();
            ffi::gsk_transform_transform_bounds(
                self.to_glib_none().0,
                rect.to_glib_none().0,
                out_rect.to_glib_none_mut().0,
            );
            out_rect
        }
    }

    #[doc(alias = "gsk_transform_transform_point")]
    pub fn transform_point(&self, point: &graphene::Point) -> graphene::Point {
        unsafe {
            let mut out_point = graphene::Point::uninitialized();
            ffi::gsk_transform_transform_point(
                self.to_glib_none().0,
                point.to_glib_none().0,
                out_point.to_glib_none_mut().0,
            );
            out_point
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Transform {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Transform {}

impl fmt::Display for Transform {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
