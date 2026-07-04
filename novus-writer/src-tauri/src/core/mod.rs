//! Core module - Domain logic and business rules
//! 
//! This module follows Clean Architecture principles with:
//! - domain: Enterprise business rules
//! - application: Application-specific business rules  
//! - infrastructure: External concerns (DB, file system)

pub mod domain;
pub mod application;
pub mod infrastructure;
