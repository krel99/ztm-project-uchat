pub mod register;

pub use register::Register;

pub use router::*;

pub mod route {
   pub const ACCOUNT_REGISTER: &str = "/account/register";
}