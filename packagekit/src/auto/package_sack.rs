// Generated by gir (https://github.com/gtk-rs/gir @ 1c7a6b57a5fc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 21b29d0e0c1a)
// from packagekit-gir-files
// DO NOT EDIT

#[cfg(feature = "v0_5_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
use crate::{Package};
#[cfg(feature = "v0_6_1")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_6_1")))]
use crate::{PackageSackSortType};
#[cfg(feature = "v0_6_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_6_2")))]
use crate::{InfoEnum};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "PkPackageSack")]
    pub struct PackageSack(Object<ffi::PkPackageSack, ffi::PkPackageSackClass>);

    match fn {
        type_ => || ffi::pk_package_sack_get_type(),
    }
}

impl PackageSack {
        pub const NONE: Option<&'static PackageSack> = None;
    

    ///
    /// # Returns
    ///
    /// a new [`PackageSack`][crate::PackageSack] object.
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_new")]
    pub fn new() -> PackageSack {
        unsafe {
            from_glib_full(ffi::pk_package_sack_new())
        }
    }
}

#[cfg(feature = "v0_5_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
impl Default for PackageSack {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PackageSack>> Sealed for T {}
}

/// Trait containing all [`struct@PackageSack`] methods.
///
/// # Implementors
///
/// [`PackageSack`][struct@crate::PackageSack]
pub trait PackageSackExt: IsA<PackageSack> + sealed::Sealed + 'static {
    /// Adds a package to the sack.
    /// ## `package`
    /// a valid [`Package`][crate::Package] instance
    ///
    /// # Returns
    ///
    /// [`true`] if the package was added to the sack
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_add_package")]
    fn add_package(&self, package: &impl IsA<Package>) -> bool {
        unsafe {
            from_glib(ffi::pk_package_sack_add_package(self.as_ref().to_glib_none().0, package.as_ref().to_glib_none().0))
        }
    }

    /// Adds a package reference to the sack.
    /// ## `package_id`
    /// a package_id descriptor
    ///
    /// # Returns
    ///
    /// [`true`] if the package was added to the sack
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_add_package_by_id")]
    fn add_package_by_id(&self, package_id: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_package_sack_add_package_by_id(self.as_ref().to_glib_none().0, package_id.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    /// Adds packages from package-list file to a [`PackageSack`][crate::PackageSack].
    /// ## `file`
    /// a valid package-list file
    ///
    /// # Returns
    ///
    /// [`true`] if there were no errors.
    #[doc(alias = "pk_package_sack_add_packages_from_file")]
    fn add_packages_from_file(&self, file: &impl IsA<gio::File>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_package_sack_add_packages_from_file(self.as_ref().to_glib_none().0, file.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    /// Empty all the packages from the sack
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_clear")]
    fn clear(&self) {
        unsafe {
            ffi::pk_package_sack_clear(self.as_ref().to_glib_none().0);
        }
    }

    /// Returns a new package sack which only matches packages that return [`true`]
    /// from the filter function.
    /// ## `filter_cb`
    /// a `PkPackageSackFilterFunc`, which returns [`true`] for the [`Package`][crate::Package]'s to add
    ///
    /// # Returns
    ///
    /// a new [`PackageSack`][crate::PackageSack], free with `g_object_unref()`
    #[cfg(feature = "v0_6_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6_3")))]
    #[doc(alias = "pk_package_sack_filter")]
#[must_use]
    fn filter<P: FnMut(&Package) -> bool>(&self, filter_cb: P) -> Option<PackageSack> {
        let filter_cb_data: P = filter_cb;
        unsafe extern "C" fn filter_cb_func<P: FnMut(&Package) -> bool>(package: *mut ffi::PkPackage, user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let package = from_glib_borrow(package);
            let callback = user_data as *mut P;
            (*callback)(&package)
            .into_glib()
        }
        let filter_cb = Some(filter_cb_func::<P> as _);
        let super_callback0: &P = &filter_cb_data;
        unsafe {
            from_glib_full(ffi::pk_package_sack_filter(self.as_ref().to_glib_none().0, filter_cb, super_callback0 as *const _ as *mut _))
        }
    }

    /// Returns a new package sack which only matches packages that match the
    /// specified info enum value.
    /// ## `info`
    /// a [`InfoEnum`][crate::InfoEnum] value to match
    ///
    /// # Returns
    ///
    /// a new [`PackageSack`][crate::PackageSack], free with `g_object_unref()`
    #[cfg(feature = "v0_6_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6_2")))]
    #[doc(alias = "pk_package_sack_filter_by_info")]
#[must_use]
    fn filter_by_info(&self, info: InfoEnum) -> Option<PackageSack> {
        unsafe {
            from_glib_full(ffi::pk_package_sack_filter_by_info(self.as_ref().to_glib_none().0, info.into_glib()))
        }
    }

    /// Finds a package in a sack from reference. As soon as one package is found
    /// the search is stopped.
    /// ## `package_id`
    /// a package_id descriptor
    ///
    /// # Returns
    ///
    /// the [`Package`][crate::Package] object, or [`None`] if unfound. Free with `g_object_unref()`
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_find_by_id")]
    fn find_by_id(&self, package_id: &str) -> Option<Package> {
        unsafe {
            from_glib_full(ffi::pk_package_sack_find_by_id(self.as_ref().to_glib_none().0, package_id.to_glib_none().0))
        }
    }

    /// Finds a package in a sack by package name and architecture. As soon as one
    /// package is found the search is stopped.
    /// ## `package_id`
    /// a package_id descriptor
    ///
    /// # Returns
    ///
    /// the [`Package`][crate::Package] object, or [`None`] if not found.
    #[cfg(feature = "v0_8_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_8_16")))]
    #[doc(alias = "pk_package_sack_find_by_id_name_arch")]
    fn find_by_id_name_arch(&self, package_id: &str) -> Option<Package> {
        unsafe {
            from_glib_full(ffi::pk_package_sack_find_by_id_name_arch(self.as_ref().to_glib_none().0, package_id.to_glib_none().0))
        }
    }

    /// Gets the package array from the sack
    ///
    /// # Returns
    ///
    /// A `GPtrArray`, free with `g_ptr_array_unref()`.
    #[cfg(feature = "v0_6_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6_1")))]
    #[doc(alias = "pk_package_sack_get_array")]
    #[doc(alias = "get_array")]
    fn array(&self) -> Vec<Package> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::pk_package_sack_get_array(self.as_ref().to_glib_none().0))
        }
    }

    /// Gets the properties the daemon supports.
    /// Warning: this function is synchronous, and may block. Do not use it in GUI
    /// applications.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] if the properties were set correctly
    #[cfg(feature = "v0_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6")))]
    #[doc(alias = "pk_package_sack_get_details")]
    #[doc(alias = "get_details")]
    fn details(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_package_sack_get_details(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "pk_package_sack_get_details_async")]
    //#[doc(alias = "get_details_async")]
    //fn details_async<P: Fn(&Progress, &ProgressType) + 'static, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, callback: Q) {
    //    unsafe { TODO: call ffi:pk_package_sack_get_details_async() }
    //}

    /// Returns all the Package IDs in the sack
    ///
    /// # Returns
    ///
    /// the number of packages in the sack, free with `g_strfreev()`
    #[cfg(feature = "v0_5_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
    #[doc(alias = "pk_package_sack_get_ids")]
    #[doc(alias = "get_ids")]
    fn ids(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::pk_package_sack_get_ids(self.as_ref().to_glib_none().0))
        }
    }

    /// Gets the number of packages in the sack
    ///
    /// # Returns
    ///
    /// the number of packages in the sack
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> u32 {
        unsafe {
            ffi::pk_package_sack_get_size(self.as_ref().to_glib_none().0)
        }
    }

    /// Gets the total size of the package sack in bytes.
    ///
    /// # Returns
    ///
    /// the size in bytes
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_get_total_bytes")]
    #[doc(alias = "get_total_bytes")]
    fn total_bytes(&self) -> u64 {
        unsafe {
            ffi::pk_package_sack_get_total_bytes(self.as_ref().to_glib_none().0)
        }
    }

    /// Gets the properties the daemon supports.
    /// Warning: this function is synchronous, and may block. Do not use it in GUI
    /// applications.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] if the properties were set correctly
    #[cfg(feature = "v0_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6")))]
    #[doc(alias = "pk_package_sack_get_update_detail")]
    #[doc(alias = "get_update_detail")]
    fn update_detail(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_package_sack_get_update_detail(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(feature = "v0_5_2")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    //#[doc(alias = "pk_package_sack_get_update_detail_async")]
    //#[doc(alias = "get_update_detail_async")]
    //fn update_detail_async<P: Fn(&Progress, &ProgressType) + 'static, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, callback: Q) {
    //    unsafe { TODO: call ffi:pk_package_sack_get_update_detail_async() }
    //}

    /// Removes from the package sack any packages that return [`false`] from the filter
    /// function.
    /// ## `filter_cb`
    /// a `PkPackageSackFilterFunc`, which returns [`true`] for the [`Package`][crate::Package]'s to retain
    ///
    /// # Returns
    ///
    /// [`true`] if a package was removed from the sack
    #[cfg(feature = "v0_6_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6_3")))]
    #[doc(alias = "pk_package_sack_remove_by_filter")]
    fn remove_by_filter<P: FnMut(&Package) -> bool>(&self, filter_cb: P) -> bool {
        let filter_cb_data: P = filter_cb;
        unsafe extern "C" fn filter_cb_func<P: FnMut(&Package) -> bool>(package: *mut ffi::PkPackage, user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let package = from_glib_borrow(package);
            let callback = user_data as *mut P;
            (*callback)(&package)
            .into_glib()
        }
        let filter_cb = Some(filter_cb_func::<P> as _);
        let super_callback0: &P = &filter_cb_data;
        unsafe {
            from_glib(ffi::pk_package_sack_remove_by_filter(self.as_ref().to_glib_none().0, filter_cb, super_callback0 as *const _ as *mut _))
        }
    }

    /// Removes a package reference from the sack. The pointers have to match exactly.
    /// ## `package`
    /// a valid [`Package`][crate::Package] instance
    ///
    /// # Returns
    ///
    /// [`true`] if the package was removed from the sack
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_remove_package")]
    fn remove_package(&self, package: &impl IsA<Package>) -> bool {
        unsafe {
            from_glib(ffi::pk_package_sack_remove_package(self.as_ref().to_glib_none().0, package.as_ref().to_glib_none().0))
        }
    }

    /// Removes a package reference from the sack. As soon as one package is removed
    /// the search is stopped.
    /// ## `package_id`
    /// a package_id descriptor
    ///
    /// # Returns
    ///
    /// [`true`] if the package was removed from the sack
    #[cfg(feature = "v0_5_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    #[doc(alias = "pk_package_sack_remove_package_by_id")]
    fn remove_package_by_id(&self, package_id: &str) -> bool {
        unsafe {
            from_glib(ffi::pk_package_sack_remove_package_by_id(self.as_ref().to_glib_none().0, package_id.to_glib_none().0))
        }
    }

    /// Gets the properties the daemon supports.
    /// Warning: this function is synchronous, and may block. Do not use it in GUI
    /// applications.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable] or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] if the properties were set correctly
    #[cfg(feature = "v0_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6")))]
    #[doc(alias = "pk_package_sack_resolve")]
    fn resolve(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_package_sack_resolve(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(feature = "v0_5_2")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_2")))]
    //#[doc(alias = "pk_package_sack_resolve_async")]
    //fn resolve_async<P: Fn(&Progress, &ProgressType) + 'static, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, callback: Q) {
    //    unsafe { TODO: call ffi:pk_package_sack_resolve_async() }
    //}

    /// Sorts the package sack
    /// ## `type_`
    /// the type of sorting, e.g. [`PackageSackSortType::Name`][crate::PackageSackSortType::Name]
    #[cfg(feature = "v0_6_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_6_1")))]
    #[doc(alias = "pk_package_sack_sort")]
    fn sort(&self, type_: PackageSackSortType) {
        unsafe {
            ffi::pk_package_sack_sort(self.as_ref().to_glib_none().0, type_.into_glib());
        }
    }

    /// Write the contents of a [`PackageSack`][crate::PackageSack] to a package-list file.
    /// ## `file`
    /// a valid package-list file
    ///
    /// # Returns
    ///
    /// [`true`] if there were no errors.
    #[cfg(feature = "v0_8_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_8_6")))]
    #[doc(alias = "pk_package_sack_to_file")]
    fn to_file(&self, file: &impl IsA<gio::File>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_package_sack_to_file(self.as_ref().to_glib_none().0, file.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl<O: IsA<PackageSack>> PackageSackExt for O {}
