// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EntryBuffer;
use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkPasswordEntryBuffer")]
    pub struct PasswordEntryBuffer(Object<ffi::GtkPasswordEntryBuffer, ffi::GtkPasswordEntryBufferClass>) @extends EntryBuffer;

    match fn {
        type_ => || ffi::gtk_password_entry_buffer_get_type(),
    }
}

impl PasswordEntryBuffer {
    #[doc(alias = "gtk_password_entry_buffer_new")]
    pub fn new() -> PasswordEntryBuffer {
        assert_initialized_main_thread!();
        unsafe { EntryBuffer::from_glib_full(ffi::gtk_password_entry_buffer_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PasswordEntryBuffer`] objects.
    ///
    /// This method returns an instance of [`PasswordEntryBufferBuilder`](crate::builders::PasswordEntryBufferBuilder) which can be used to create [`PasswordEntryBuffer`] objects.
    pub fn builder() -> PasswordEntryBufferBuilder {
        PasswordEntryBufferBuilder::default()
    }
}

impl Default for PasswordEntryBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PasswordEntryBuffer`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PasswordEntryBufferBuilder {
    max_length: Option<i32>,
    text: Option<String>,
}

impl PasswordEntryBufferBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PasswordEntryBufferBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PasswordEntryBuffer`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PasswordEntryBuffer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref max_length) = self.max_length {
            properties.push(("max-length", max_length));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        glib::Object::new::<PasswordEntryBuffer>(&properties)
    }

    pub fn max_length(mut self, max_length: i32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

impl fmt::Display for PasswordEntryBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PasswordEntryBuffer")
    }
}
