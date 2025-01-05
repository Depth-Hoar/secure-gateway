// Role and access control logic

use serde::{Deserialize, Serialize};

/// Example user roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Family,
    Guest,
}

impl Default for Role {
    fn default() -> Self {
        Role::Guest
    }
}

/// Check role-based access to certain paths.
/// In a real system, you may have a more granular ACL/Policy manager.
pub fn user_has_access(role: &Role, path: &str) -> bool {
    match role {
        Role::Admin => {
            // Admin can access everything
            true
        }
        Role::Family => {
            // Family can access jellyfin and files
            if path.starts_with("/jellyfin") || path.starts_with("/files") {
                true
            } else {
                false
            }
        }
        Role::Guest => {
            // Guest can only access /jellyfin
            if path.starts_with("/jellyfin") {
                true
            } else {
                false
            }
        }
    }
}
