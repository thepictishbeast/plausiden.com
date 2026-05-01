//! Vertical-specific landing pages.
//!
//! Each vertical gets a sibling module (`legal`, eventually `healthcare`,
//! `journalism`, etc.). One page Salesman per-vertical email campaigns
//! point at — written for someone already pre-qualified by the email,
//! not for cold organic traffic.

pub mod financial_advisors;
pub mod healthcare;
pub mod journalism;
pub mod legal;
pub mod nonprofit;
pub mod template;
