use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomNotification {
    pub text: String,
}

impl CustomNotification {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            text: description.into(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Notifier;

impl Notifier {
    pub fn spawn(&self, notification: CustomNotification) {
        gloo::console::error!(notification.text);
    }
}

pub fn new_notifier<T>() -> Notifier {
    let _ = PhantomData::<T>;
    Notifier
}
