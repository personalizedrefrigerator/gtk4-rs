// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Application;
use Error;
use Widget;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Builder(Object<gtk_sys::GtkBuilder, gtk_sys::GtkBuilderClass, BuilderClass>);

    match fn {
        get_type => || gtk_sys::gtk_builder_get_type(),
    }
}

impl Builder {
    pub fn new() -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_builder_new())
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_builder_new_from_resource(resource_path.to_glib_none().0))
        }
    }

    pub fn new_from_string(string: &str) -> Builder {
        assert_initialized_main_thread!();
        let length = string.len() as isize;
        unsafe {
            from_glib_full(gtk_sys::gtk_builder_new_from_string(string.to_glib_none().0, length))
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BUILDER: Option<&Builder> = None;

pub trait BuilderExt: 'static {
    //fn add_callback_symbol<P: FnOnce() + 'static>(&self, callback_name: &str, callback_symbol: P);

    //fn add_callback_symbols<P: FnOnce() + 'static>(&self, first_callback_name: &str, first_callback_symbol: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn add_from_resource(&self, resource_path: &str) -> Result<(), Error>;

    fn add_from_string(&self, buffer: &str) -> Result<(), Error>;

    fn add_objects_from_resource(&self, resource_path: &str, object_ids: &[&str]) -> Result<(), Error>;

    fn add_objects_from_string(&self, buffer: &str, object_ids: &[&str]) -> Result<(), Error>;

    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //fn connect_signals_full(&self, func: /*Unimplemented*/FnMut(&Builder, &glib::Object, &str, &str, Option<&glib::Object>, /*Ignored*/glib::ConnectFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn expose_object<P: IsA<glib::Object>>(&self, name: &str, object: &P);

    fn extend_with_template<P: IsA<Widget>>(&self, widget: &P, template_type: glib::types::Type, buffer: &str) -> Result<(), Error>;

    fn get_application(&self) -> Option<Application>;

    fn get_objects(&self) -> Vec<glib::Object>;

    fn get_translation_domain(&self) -> Option<GString>;

    fn get_type_from_name(&self, type_name: &str) -> glib::types::Type;

    //fn lookup_callback_symbol(&self, callback_name: &str) -> Option<Box<dyn Fn() + 'static>>;

    fn set_application<P: IsA<Application>>(&self, application: &P);

    fn set_translation_domain(&self, domain: Option<&str>);

    //fn value_from_string(&self, pspec: /*Ignored*/&glib::ParamSpec, string: &str) -> Result<glib::Value, Error>;

    fn value_from_string_type(&self, type_: glib::types::Type, string: &str) -> Result<glib::Value, Error>;

    fn connect_property_translation_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Builder>> BuilderExt for O {
    //fn add_callback_symbol<P: FnOnce() + 'static>(&self, callback_name: &str, callback_symbol: P) {
    //    unsafe { TODO: call gtk_sys:gtk_builder_add_callback_symbol() }
    //}

    //fn add_callback_symbols<P: FnOnce() + 'static>(&self, first_callback_name: &str, first_callback_symbol: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_builder_add_callback_symbols() }
    //}

    fn add_from_resource(&self, resource_path: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_builder_add_from_resource(self.as_ref().to_glib_none().0, resource_path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_from_string(&self, buffer: &str) -> Result<(), Error> {
        let length = buffer.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_builder_add_from_string(self.as_ref().to_glib_none().0, buffer.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_objects_from_resource(&self, resource_path: &str, object_ids: &[&str]) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_builder_add_objects_from_resource(self.as_ref().to_glib_none().0, resource_path.to_glib_none().0, object_ids.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_objects_from_string(&self, buffer: &str, object_ids: &[&str]) -> Result<(), Error> {
        let length = buffer.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_builder_add_objects_from_string(self.as_ref().to_glib_none().0, buffer.to_glib_none().0, length, object_ids.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_sys:gtk_builder_connect_signals() }
    //}

    //fn connect_signals_full(&self, func: /*Unimplemented*/FnMut(&Builder, &glib::Object, &str, &str, Option<&glib::Object>, /*Ignored*/glib::ConnectFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_sys:gtk_builder_connect_signals_full() }
    //}

    fn expose_object<P: IsA<glib::Object>>(&self, name: &str, object: &P) {
        unsafe {
            gtk_sys::gtk_builder_expose_object(self.as_ref().to_glib_none().0, name.to_glib_none().0, object.as_ref().to_glib_none().0);
        }
    }

    fn extend_with_template<P: IsA<Widget>>(&self, widget: &P, template_type: glib::types::Type, buffer: &str) -> Result<(), Error> {
        let length = buffer.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_builder_extend_with_template(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, template_type.to_glib(), buffer.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_application(&self) -> Option<Application> {
        unsafe {
            from_glib_none(gtk_sys::gtk_builder_get_application(self.as_ref().to_glib_none().0))
        }
    }

    fn get_objects(&self) -> Vec<glib::Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_builder_get_objects(self.as_ref().to_glib_none().0))
        }
    }

    fn get_translation_domain(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_builder_get_translation_domain(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_from_name(&self, type_name: &str) -> glib::types::Type {
        unsafe {
            from_glib(gtk_sys::gtk_builder_get_type_from_name(self.as_ref().to_glib_none().0, type_name.to_glib_none().0))
        }
    }

    //fn lookup_callback_symbol(&self, callback_name: &str) -> Option<Box<dyn Fn() + 'static>> {
    //    unsafe { TODO: call gtk_sys:gtk_builder_lookup_callback_symbol() }
    //}

    fn set_application<P: IsA<Application>>(&self, application: &P) {
        unsafe {
            gtk_sys::gtk_builder_set_application(self.as_ref().to_glib_none().0, application.as_ref().to_glib_none().0);
        }
    }

    fn set_translation_domain(&self, domain: Option<&str>) {
        unsafe {
            gtk_sys::gtk_builder_set_translation_domain(self.as_ref().to_glib_none().0, domain.to_glib_none().0);
        }
    }

    //fn value_from_string(&self, pspec: /*Ignored*/&glib::ParamSpec, string: &str) -> Result<glib::Value, Error> {
    //    unsafe { TODO: call gtk_sys:gtk_builder_value_from_string() }
    //}

    fn value_from_string_type(&self, type_: glib::types::Type, string: &str) -> Result<glib::Value, Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_builder_value_from_string_type(self.as_ref().to_glib_none().0, type_.to_glib(), string.to_glib_none().0, value.to_glib_none_mut().0, &mut error);
            if error.is_null() { Ok(value) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_translation_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::translation-domain\0".as_ptr() as *const _,
                Some(transmute(notify_translation_domain_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_translation_domain_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkBuilder, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Builder> {
    let f: &F = &*(f as *const F);
    f(&Builder::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Builder")
    }
}
