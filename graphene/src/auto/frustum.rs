// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Box;
use crate::Point3D;
use crate::Sphere;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug)]
    pub struct Frustum(BoxedInline<ffi::graphene_frustum_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_frustum_get_type(), ptr as *mut _) as *mut ffi::graphene_frustum_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_frustum_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_frustum_get_type(),
    }
}

impl Frustum {
    #[doc(alias = "graphene_frustum_contains_point")]
    pub fn contains_point(&self, point: &Point3D) -> bool {
        unsafe {
            ffi::graphene_frustum_contains_point(self.to_glib_none().0, point.to_glib_none().0)
        }
    }

    #[doc(alias = "graphene_frustum_equal")]
    fn equal(&self, b: &Frustum) -> bool {
        unsafe { ffi::graphene_frustum_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_frustum_intersects_box")]
    pub fn intersects_box(&self, box_: &Box) -> bool {
        unsafe {
            ffi::graphene_frustum_intersects_box(self.to_glib_none().0, box_.to_glib_none().0)
        }
    }

    #[doc(alias = "graphene_frustum_intersects_sphere")]
    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        unsafe {
            ffi::graphene_frustum_intersects_sphere(self.to_glib_none().0, sphere.to_glib_none().0)
        }
    }
}

impl PartialEq for Frustum {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Frustum {}
