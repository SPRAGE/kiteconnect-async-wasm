/*!
Authentication and user-related data models.

This module provides typed models for:
- Session management (login, access tokens)
- User profiles and account information  
- Margin data and segment details
*/

pub mod margins;
pub mod session;
pub mod user;

// Re-export main types for convenient access
pub use margins::*;
pub use session::*;
pub use user::*;
