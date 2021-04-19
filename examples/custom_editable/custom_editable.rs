use crate::custom_tag::CustomTag;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use glib::ParamSpec;
    use once_cell::sync::Lazy;
    use std::cell::{Cell, RefCell};

    pub struct CustomEditable {
        text: gtk::Text,
        pub spinner: RefCell<Option<gtk::Spinner>>,
        pub show_spinner: Cell<bool>,
    }

    impl Default for CustomEditable {
        fn default() -> Self {
            Self {
                text: gtk::Text::new(),
                spinner: RefCell::default(),
                show_spinner: Cell::new(false),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomEditable {
        const NAME: &'static str = "CustomEditable";
        type Type = super::CustomEditable;
        type ParentType = gtk::Widget;
        type Interfaces = (gtk::Editable,);

        fn class_init(klass: &mut Self::Class) {
            // The call to `gtk_editable_install_properties` at `class_init` is automatically done for you.
            klass.set_layout_manager_type::<gtk::BoxLayout>();
            klass.set_css_name("entry");
            klass.set_accessible_role(gtk::AccessibleRole::TextBox);
        }
    }

    impl ObjectImpl for CustomEditable {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpec::new_boolean(
                    "show-spinner",
                    "Spinner shown",
                    "Whether the editable has a visible spinner",
                    false,
                    glib::ParamFlags::READWRITE,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            editable: &Self::Type,
            id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            // In case this is a property that's automatically added for Editable implementations.
            if !self.delegate_set_property(editable, id, value, pspec) {
                match pspec.name() {
                    "show-spinner" => {
                        editable.set_show_spinner(value.get().unwrap());
                    }
                    _ => unimplemented!(),
                }
            }
        }

        fn property(
            &self,
            editable: &Self::Type,
            id: usize,
            pspec: &glib::ParamSpec,
        ) -> glib::Value {
            // In case this is a property that's automatically added for Editable implementations.
            if let Some(value) = self.delegate_get_property(editable, id, pspec) {
                value
            } else {
                match pspec.name() {
                    "show-spinner" => self.show_spinner.get().to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        fn constructed(&self, editable: &Self::Type) {
            // Most of the times when implementing Editable, you just want to embed something like
            // `gtk::Text` inside a more complex widget. In such case, your implementation most forward the `gtk::Text`
            // or any `Editable` to the delegate. That starts by returning at `EditableImpl::get_delegate`.
            //
            // In such case, the user has to initialize the delegate at `constructed` and stop it at `dispose`.
            // It mostly consists of wiring up/down some signals from the delegate (`gtk::Text` in this case) to the internal editable
            // implementation.
            editable.init_delegate();
            self.text.set_hexpand(true);
            self.text.set_vexpand(true);

            self.text.set_parent(editable);
            editable.add_css_class("tagged");
            editable.set_enable_undo(true);
        }

        fn dispose(&self, editable: &Self::Type) {
            // Wire down the delegate signals machinery
            editable.finish_delegate();
            self.text.unparent();
            while let Some(child) = editable.first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for CustomEditable {
        fn grab_focus(&self, _widget: &Self::Type) -> bool {
            self.text.grab_focus()
        }
    }
    impl EditableImpl for CustomEditable {
        fn delegate(&self, _editable: &Self::Type) -> Option<gtk::Editable> {
            Some(self.text.clone().upcast())
        }
    }
}

glib::wrapper! {
    pub struct CustomEditable(ObjectSubclass<imp::CustomEditable>) @extends gtk::Widget, @implements gtk::Editable;
}

impl Default for CustomEditable {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomEditable {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a CustomEditable")
    }

    pub fn add_tag(&self, tag: &CustomTag) {
        tag.set_parent(self);
    }

    pub fn remove_tag(&self, tag: &CustomTag) {
        tag.unparent();
    }

    pub fn set_show_spinner(&self, show_spinner: bool) {
        let self_ = imp::CustomEditable::from_instance(self);
        if self_.show_spinner.get() == show_spinner {
            return;
        }

        if show_spinner {
            let spinner = gtk::SpinnerBuilder::new()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .build();
            spinner.start();

            spinner.set_parent(self);
            self_.spinner.replace(Some(spinner));
        } else if let Some(spinner) = self_.spinner.borrow_mut().take() {
            spinner.unparent();
        }
        self_.show_spinner.set(show_spinner);
    }
}
