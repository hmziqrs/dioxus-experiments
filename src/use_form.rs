use std::collections::HashMap;

use dioxus::prelude::*;

use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Validate, Clone)]
pub struct LoginForm {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 4, message = "Password must be at least 4 characters"))]
    pub password: String,
}

impl LoginForm {
    pub fn new() -> Self {
        LoginForm {
            email: String::new(),
            password: String::new(),
        }
    }

    pub fn from_map(map: HashMap<String, String>) -> Self {
        let email = map.get("email").unwrap();
        let password = map.get("password").unwrap();

        LoginForm {
            email: email.clone(),
            password: password.clone(),
        }
    }
}

impl FormModel for LoginForm {
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("email".to_string(), self.email.clone());
        map.insert("password".to_string(), self.password.clone());
        map
    }

    fn update_field(&mut self, name: &str, value: &str) {
        match name {
            "email" => self.email = value.to_string(),
            "password" => self.password = value.to_string(),
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSlice {
    name: String,
    value: String,
    error: Option<String>,
    default_value: String,

    focused: bool,
    touched: bool,
    dirty: bool,
}

impl FieldSlice {
    pub fn new(name: String, value: String) -> Self {
        FieldSlice {
            name,
            value: value.clone(),
            default_value: value,

            error: None,
            focused: false,
            touched: false,
            dirty: false,
        }
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }
}

pub trait FormModel: Validate + Clone {
    fn to_map(&self) -> HashMap<String, String>;
    fn update_field(&mut self, name: &str, value: &str);
}

#[derive(Debug, Clone)]
pub struct FormSlice<T: FormModel> {
    data: T,
    fields: HashMap<String, FieldSlice>,
    has_errors: bool,
    submit_count: u32,
}

impl<T: FormModel> FormSlice<T> {
    pub fn new(data: T) -> Self {
        let initial_map = data.to_map();
        let mut fields = HashMap::new();

        for (name, value) in &initial_map {
            fields.insert(name.clone(), FieldSlice::new(name.clone(), value.clone()));
        }

        FormSlice {
            data,
            fields,
            has_errors: false,
            submit_count: 0,
        }
    }

    pub fn update_field(&mut self, name: &str, value: &str) {
        // Update the field slice
        if let Some(field) = self.fields.get_mut(name) {
            field.value = value.to_string();
            field.dirty = field.value != field.default_value;
        }

        // Update the underlying data model
        self.data.update_field(name, value);

        // Validate after update
        self.validate();
    }

    pub fn is_dirty(&self) -> bool {
        self.fields.values().any(|field| field.dirty)
    }

    pub fn is_valid(&self) -> bool {
        !self.has_errors
    }

    pub fn get_field(&self, name: &str) -> Option<&FieldSlice> {
        self.fields.get(name)
    }

    pub fn validate(&mut self) -> bool {
        for field in self.fields.values_mut() {
            field.set_error(None);
        }

        // Validate using validator crate
        match self.data.validate() {
            Ok(()) => {
                self.has_errors = false;
                true
            }
            Err(errors) => {
                self.apply_validation_errors(errors);
                self.has_errors = true;
                false
            }
        }
    }

    fn apply_validation_errors(&mut self, errors: ValidationErrors) {
        for (field_name, field_errors) in errors.field_errors() {
            if let Some(field) = self.fields.get_mut(field_name) {
                if let Some(first_error) = field_errors.first() {
                    // Get the error message
                    let message = match &first_error.message {
                        Some(msg) => msg.to_string(),
                        None => format!("Invalid {}", field_name),
                    };
                    field.set_error(Some(message));
                }
            }
        }
    }

    fn on_submit(&mut self, callback: impl Fn(T)) {
        if self.validate() {
            callback(self.data.clone());
        }
        self.submit_count += 1;
    }
}

pub fn use_form(initial_state: LoginForm) -> Signal<FormSlice<LoginForm>> {
    let form_slice = use_signal(|| FormSlice::new(initial_state));

    form_slice
}
