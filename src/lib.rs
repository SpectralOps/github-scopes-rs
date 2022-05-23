//! `github-scopes-rs` Discover GitHub token scope permission and return you an easy interface for checking token permission before querying GitHub.
//!
//! In many cases, you try to do actions to GitHub, but you get unclear permissions errors. This project allows you to get which permission your token has before,
//! called GitHub, and if you don’t have the right permissions, you can tell the user the exact permission the user needs.
//!
//! ## How it works
//! We called Github api with the given token and get which permissions scope the token has in order the access to the API. Then, the permissions are being converted to a simple object that you can work with.
//! click [here](https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps) read GitHub documentation.
//!
//! ## Get token permission
//! ```rust
//! use github_scopes_rs::oauth::OAuthContext;
//!
//! # fn run()  {
//! let permissions = OAuthContext::new("token".to_string()).unwrap().get_scope_permissions();
//!
//! if !permissions.repo.all {
//!     // do something
//! }
//! # }
//! ```
extern crate env_logger;
pub mod oauth;
pub mod transform;

#[cfg(test)]
mod tests {

    use super::oauth::OAuthContext;
    use mockito::mock;
    use std::env;

    #[test]
    fn can_get_oauth_scope() {
        let m = mock("GET", "/rate_limit")
            .match_header("user-agent", env!("CARGO_PKG_NAME"))
            .match_header("authorization", "token dummy-token")
            .with_status(200)
            .with_header("x-oauth-scopes", "repo")
            .expect_at_most(1)
            .create();

        let scope = OAuthContext::new("dummy-token".to_string());
        m.assert();
        assert!(scope.is_ok());

        let permissions = scope.unwrap().get_scope_permissions();
        assert!(permissions.repo.all);
    }

    #[test]
    fn cat_refresh_oauth_scope_with_new_token() {
        let mock_first_request = mock("GET", "/rate_limit")
            .match_header("user-agent", env!("CARGO_PKG_NAME"))
            .match_header("authorization", "token dummy-token")
            .with_status(200)
            .with_header("x-oauth-scopes", "")
            .expect_at_most(1)
            .create();

        let mock_refresh_request = mock("GET", "/rate_limit")
            .match_header("user-agent", env!("CARGO_PKG_NAME"))
            .match_header("authorization", "token dummy-token2")
            .with_status(200)
            .with_header("x-oauth-scopes", "repo")
            .expect_at_most(1)
            .create();

        let mut scope = OAuthContext::new("dummy-token".to_string()).unwrap();
        mock_first_request.assert();

        let permissions = scope.get_scope_permissions();
        assert!(!permissions.repo.all);

        let refresh = scope.refresh(Some("dummy-token2".to_string()));
        mock_refresh_request.assert();
        assert!(refresh.is_ok());

        let permissions = scope.get_scope_permissions();
        assert!(permissions.repo.all);
    }

    #[test]
    fn invalid_status_code() {
        let m = mock("GET", "/rate_limit")
            .match_header("user-agent", env!("CARGO_PKG_NAME"))
            .match_header("authorization", "token dummy-token")
            .with_status(500)
            .with_header("x-oauth-scopes", "repo")
            .expect_at_most(1)
            .create();

        let scope = OAuthContext::new("dummy-token".to_string());
        m.assert();
        assert!(scope.is_err());
    }

    #[test]
    fn scope_header_not_found() {
        let m = mock("GET", "/rate_limit")
            .match_header("user-agent", env!("CARGO_PKG_NAME"))
            .match_header("authorization", "token dummy-token")
            .with_status(200)
            .expect_at_most(1)
            .create();

        let scope = OAuthContext::new("dummy-token".to_string());
        m.assert();
        assert!(scope.is_err());
    }
}
