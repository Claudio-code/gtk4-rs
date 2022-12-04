// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
#[cfg(any(feature = "v4_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
use crate::ContentFit;
use crate::LayoutManager;
use crate::Overflow;
use crate::Widget;
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
    #[doc(alias = "GtkPicture")]
    pub struct Picture(Object<ffi::GtkPicture, ffi::GtkPictureClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_picture_get_type(),
    }
}

impl Picture {
    #[doc(alias = "gtk_picture_new")]
    pub fn new() -> Picture {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_picture_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_picture_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file(file: &impl IsA<gio::File>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_file(
                file.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_filename")]
    #[doc(alias = "new_for_filename")]
    pub fn for_filename(filename: impl AsRef<std::path::Path>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_filename(
                filename.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_paintable")]
    #[doc(alias = "new_for_paintable")]
    pub fn for_paintable(paintable: &impl IsA<gdk::Paintable>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_paintable(
                paintable.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_pixbuf")]
    #[doc(alias = "new_for_pixbuf")]
    pub fn for_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_resource")]
    #[doc(alias = "new_for_resource")]
    pub fn for_resource(resource_path: &str) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Picture`] objects.
    ///
    /// This method returns an instance of [`PictureBuilder`](crate::builders::PictureBuilder) which can be used to create [`Picture`] objects.
    pub fn builder() -> PictureBuilder {
        PictureBuilder::default()
    }

    #[doc(alias = "gtk_picture_get_alternative_text")]
    #[doc(alias = "get_alternative_text")]
    pub fn alternative_text(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_picture_get_alternative_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_picture_get_can_shrink")]
    #[doc(alias = "get_can_shrink")]
    pub fn can_shrink(&self) -> bool {
        unsafe { from_glib(ffi::gtk_picture_get_can_shrink(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_picture_get_content_fit")]
    #[doc(alias = "get_content_fit")]
    pub fn content_fit(&self) -> ContentFit {
        unsafe { from_glib(ffi::gtk_picture_get_content_fit(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_picture_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_picture_get_file(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_picture_get_keep_aspect_ratio")]
    #[doc(alias = "get_keep_aspect_ratio")]
    pub fn is_keep_aspect_ratio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_picture_get_keep_aspect_ratio(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_picture_get_paintable")]
    #[doc(alias = "get_paintable")]
    pub fn paintable(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::gtk_picture_get_paintable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_picture_set_alternative_text")]
    pub fn set_alternative_text(&self, alternative_text: Option<&str>) {
        unsafe {
            ffi::gtk_picture_set_alternative_text(
                self.to_glib_none().0,
                alternative_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_picture_set_can_shrink")]
    pub fn set_can_shrink(&self, can_shrink: bool) {
        unsafe {
            ffi::gtk_picture_set_can_shrink(self.to_glib_none().0, can_shrink.into_glib());
        }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_picture_set_content_fit")]
    pub fn set_content_fit(&self, content_fit: ContentFit) {
        unsafe {
            ffi::gtk_picture_set_content_fit(self.to_glib_none().0, content_fit.into_glib());
        }
    }

    #[doc(alias = "gtk_picture_set_file")]
    pub fn set_file(&self, file: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_picture_set_file(
                self.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_picture_set_filename")]
    pub fn set_filename(&self, filename: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::gtk_picture_set_filename(
                self.to_glib_none().0,
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_picture_set_keep_aspect_ratio")]
    pub fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        unsafe {
            ffi::gtk_picture_set_keep_aspect_ratio(
                self.to_glib_none().0,
                keep_aspect_ratio.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_picture_set_paintable")]
    pub fn set_paintable(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::gtk_picture_set_paintable(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_picture_set_pixbuf")]
    pub fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_picture_set_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_picture_set_resource")]
    pub fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_picture_set_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[doc(alias = "alternative-text")]
    pub fn connect_alternative_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alternative_text_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::alternative-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alternative_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-shrink")]
    pub fn connect_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_shrink_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::can-shrink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_shrink_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "content-fit")]
    pub fn connect_content_fit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_fit_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::content-fit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_fit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[doc(alias = "keep-aspect-ratio")]
    pub fn connect_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_keep_aspect_ratio_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::keep-aspect-ratio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_keep_aspect_ratio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "paintable")]
    pub fn connect_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::paintable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_paintable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Picture {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Picture`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PictureBuilder {
    alternative_text: Option<String>,
    can_shrink: Option<bool>,
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    content_fit: Option<ContentFit>,
    file: Option<gio::File>,
    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    keep_aspect_ratio: Option<bool>,
    paintable: Option<gdk::Paintable>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
}

impl PictureBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PictureBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Picture`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Picture {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref alternative_text) = self.alternative_text {
            properties.push(("alternative-text", alternative_text));
        }
        if let Some(ref can_shrink) = self.can_shrink {
            properties.push(("can-shrink", can_shrink));
        }
        #[cfg(any(feature = "v4_8", feature = "dox"))]
        if let Some(ref content_fit) = self.content_fit {
            properties.push(("content-fit", content_fit));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref keep_aspect_ratio) = self.keep_aspect_ratio {
            properties.push(("keep-aspect-ratio", keep_aspect_ratio));
        }
        if let Some(ref paintable) = self.paintable {
            properties.push(("paintable", paintable));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        glib::Object::new::<Picture>(&properties)
    }

    pub fn alternative_text(mut self, alternative_text: &str) -> Self {
        self.alternative_text = Some(alternative_text.to_string());
        self
    }

    pub fn can_shrink(mut self, can_shrink: bool) -> Self {
        self.can_shrink = Some(can_shrink);
        self
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    pub fn content_fit(mut self, content_fit: ContentFit) -> Self {
        self.content_fit = Some(content_fit);
        self
    }

    pub fn file(mut self, file: &impl IsA<gio::File>) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    pub fn keep_aspect_ratio(mut self, keep_aspect_ratio: bool) -> Self {
        self.keep_aspect_ratio = Some(keep_aspect_ratio);
        self
    }

    pub fn paintable(mut self, paintable: &impl IsA<gdk::Paintable>) -> Self {
        self.paintable = Some(paintable.clone().upcast());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Picture")
    }
}
