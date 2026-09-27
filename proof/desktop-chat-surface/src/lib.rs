#![forbid(unsafe_code)]
#![allow(clippy::needless_return)]

//! Compatibility proof for ores-chat-desktop-app.rs#13.
//!
//! The `canonical` module below is an exact semantic fixture of the
//! `ChatSurface` slice pinned by the production desktop app at
//! ores-chat-interfaces@7d470b58aa062d5291f54e99cc7d6f005e68486b.

mod canonical {
    pub const CUSTOMER_REALM: &str = "ores-chat-customer";
    pub const CUSTOMER_API_AUDIENCE: &str = "ores-chat-api";
    pub const ADMIN_REALM: &str = "ores-chat-admin";
    pub const ADMIN_API_AUDIENCE: &str = "ores-chat-admin-api";

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ChatSurface {
        Public,
        User,
        Admin,
    }

    impl ChatSurface {
        #[must_use]
        pub const fn required_realm(self) -> Option<&'static str> {
            return match self {
                Self::Public => None,
                Self::User => Some(CUSTOMER_REALM),
                Self::Admin => Some(ADMIN_REALM),
            };
        }

        #[must_use]
        pub const fn required_token_audience(self) -> Option<&'static str> {
            return match self {
                Self::Public => None,
                Self::User => Some(CUSTOMER_API_AUDIENCE),
                Self::Admin => Some(ADMIN_API_AUDIENCE),
            };
        }
    }
}

use canonical::ChatSurface;

pub type DesktopAudience = ChatSurface;

pub trait DesktopAudiencePresentation {
    fn theme_id(self) -> &'static str;
    fn required_audience(self) -> Option<&'static str>;
}

impl DesktopAudiencePresentation for DesktopAudience {
    fn theme_id(self) -> &'static str {
        return match self {
            Self::Public => "ores-external-light",
            Self::User => "ores-user-deep-ocean",
            Self::Admin => "ores-admin-graphite-amber",
        };
    }

    fn required_audience(self) -> Option<&'static str> {
        return self.required_token_audience();
    }
}

#[must_use]
pub fn may_enter(audience: DesktopAudience, verified_realm: Option<&str>) -> bool {
    return match (audience.required_realm(), verified_realm) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(expected), Some(actual)) => expected == actual,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use canonical::{ADMIN_API_AUDIENCE, ADMIN_REALM, CUSTOMER_API_AUDIENCE, CUSTOMER_REALM};

    #[test]
    fn public_surface_has_no_realm_or_token_audience() {
        assert_eq!(DesktopAudience::Public.required_realm(), None);
        assert_eq!(DesktopAudience::Public.required_audience(), None);
        assert!(may_enter(DesktopAudience::Public, None));
        assert!(!may_enter(DesktopAudience::Public, Some(CUSTOMER_REALM)));
    }

    #[test]
    fn customer_and_admin_surfaces_preserve_exact_boundaries() {
        assert_eq!(DesktopAudience::User.required_realm(), Some(CUSTOMER_REALM));
        assert_eq!(
            DesktopAudience::User.required_audience(),
            Some(CUSTOMER_API_AUDIENCE)
        );
        assert_eq!(DesktopAudience::Admin.required_realm(), Some(ADMIN_REALM));
        assert_eq!(
            DesktopAudience::Admin.required_audience(),
            Some(ADMIN_API_AUDIENCE)
        );
        assert!(may_enter(DesktopAudience::User, Some(CUSTOMER_REALM)));
        assert!(!may_enter(DesktopAudience::Admin, Some(CUSTOMER_REALM)));
    }

    #[test]
    fn themes_remain_presentation_only_and_distinct() {
        let themes = [
            DesktopAudience::Public.theme_id(),
            DesktopAudience::User.theme_id(),
            DesktopAudience::Admin.theme_id(),
        ];
        assert_ne!(themes[0], themes[1]);
        assert_ne!(themes[1], themes[2]);
        assert_ne!(themes[0], themes[2]);
    }
}
