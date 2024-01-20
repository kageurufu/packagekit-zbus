// Generated by gir (https://github.com/gtk-rs/gir @ 1c7a6b57a5fc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 21b29d0e0c1a)
// from packagekit-gir-files
// DO NOT EDIT

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
use glib::{bitflags::bitflags,prelude::*,translate::*};

#[cfg(feature = "v1_2_5")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "PkOfflineFlags")]
    pub struct OfflineFlags: u32 {
        #[doc(alias = "PK_OFFLINE_FLAGS_NONE")]
        const NONE = ffi::PK_OFFLINE_FLAGS_NONE as _;
        #[doc(alias = "PK_OFFLINE_FLAGS_INTERACTIVE")]
        const INTERACTIVE = ffi::PK_OFFLINE_FLAGS_INTERACTIVE as _;
    }
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
#[doc(hidden)]
impl IntoGlib for OfflineFlags {
    type GlibType = ffi::PkOfflineFlags;

    #[inline]
    fn into_glib(self) -> ffi::PkOfflineFlags {
        self.bits()
    }
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
#[doc(hidden)]
impl FromGlib<ffi::PkOfflineFlags> for OfflineFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::PkOfflineFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
impl StaticType for OfflineFlags {
                #[inline]
    #[doc(alias = "pk_offline_flags_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::pk_offline_flags_get_type()) }
                }
            }

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
impl glib::HasParamSpec for OfflineFlags {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder
                }
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
impl glib::value::ValueType for OfflineFlags {
    type Type = Self;
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
unsafe impl<'a> glib::value::FromValue<'a> for OfflineFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
impl ToValue for OfflineFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2_5")))]
impl From<OfflineFlags> for glib::Value {
    #[inline]
    fn from(v: OfflineFlags) -> Self {
        ToValue::to_value(&v)
    }
}

