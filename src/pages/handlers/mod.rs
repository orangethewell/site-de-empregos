// HTTP Defaults
pub mod not_found;
pub use not_found::NotFound;

pub mod login;
pub use login::Login;

pub mod register;
pub use register::Register;

pub mod mail_confirm;
pub use mail_confirm::MailConfirmation;