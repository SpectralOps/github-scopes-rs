//! Scopes for OAuth Apps, more details [here](https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps)
//!
//!
use super::transform::{
    GithubScopeAdminLevel, GithubScopeEnterprise, GithubScopeLevel, GithubScopeRepo,
    GithubScopeUser, GithubTokenScope,
};
use anyhow::anyhow;
use anyhow::Result as AnyResult;
use log::debug;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::time::Duration;

/// maximum wait time until kill the request.
const MAX_REQUEST_TIME: Duration = Duration::from_secs(1);

/// lists the scopes your token has authorized.
const HEADER_SCOPE_KEY: &str = "x-oauth-scopes";

/// base github api domain
const GITHUB_API_DOMAIN: &str = "https://api.github.com";

/// Scope context
pub struct OAuthContext {
    client: reqwest::blocking::Client,
    token: String,
    domain: String,
    pub scope: Vec<String>,
}

impl OAuthContext {
    /// Create a `OAuthContext`
    pub fn new(token: &str) -> AnyResult<Self> {
        OAuthContext::create(token, GITHUB_API_DOMAIN)
    }

    /// Create a `OAuthContext` with token and domain
    pub fn with_domain(token: &str, domain: &str) -> AnyResult<Self> {
        OAuthContext::create(token, domain)
    }

    fn create(token: &str, domain: &str) -> AnyResult<Self> {
        let client = Client::builder()
            .user_agent(env!("CARGO_PKG_NAME"))
            .timeout(MAX_REQUEST_TIME)
            .build()
            .unwrap();

        let mut scope = OAuthContext {
            client,
            token: token.to_string(),
            domain: domain.to_string(),
            scope: Vec::new(),
        };
        scope.refresh(None)?;
        Ok(scope)
    }

    /// refresh OAuthContext with new token or when token permission changed in GitHub
    pub fn refresh(&mut self, token: Option<String>) -> AnyResult<()> {
        let mut headers = HeaderMap::new();

        if let Some(t) = token {
            self.token = t;
            debug!("refresh token request from given token");
        }

        headers.insert(
            AUTHORIZATION,
            self.add_authorization_token(self.token.as_ref()),
        );

        let response = self
            .client
            .get(format!("{}/rate_limit", self.domain))
            .headers(headers)
            .send()?;

        debug!(
            "github rate limit response. statusCode:{}, headers:, {:?}",
            response.status(),
            response.headers(),
        );

        if !response.status().is_success() {
            debug!("could not get token. StatusCode: {}", response.status());
            return Err(anyhow!("request failed"));
        }

        let headers_response = response.headers();
        self.scope = match headers_response.get(HEADER_SCOPE_KEY) {
            Some(v) => v
                .to_str()
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
            None => {
                return Err(anyhow!(format!(
                    "could not get {} from headers response",
                    HEADER_SCOPE_KEY
                )))
            }
        };
        Ok(())
    }

    /// return GitHub token scope as struct
    pub fn get_scope_permissions(&self) -> GithubTokenScope {
        GithubTokenScope {
            repo: GithubScopeRepo {
                all: self.scope.contains(&"repo".to_owned()),
                status: self.scope.contains(&"repo".to_owned())
                    || self.scope.contains(&"repo:status".to_owned()),
                deployment: self.scope.contains(&"repo".to_owned())
                    || self.scope.contains(&"repo_deployment".to_owned()),
                public_repo: self.scope.contains(&"repo".to_owned())
                    || self.scope.contains(&"public_repo".to_owned()),
                invite: self.scope.contains(&"repo".to_owned())
                    || self.scope.contains(&"repo:invite".to_owned()),
                security_events: self.scope.contains(&"repo".to_owned())
                    || self.scope.contains(&"security_events".to_owned()),
            },
            workflow: self.scope.contains(&"workflow".to_owned()),
            packages: GithubScopeLevel {
                write: self.scope.contains(&"write:packages".to_owned()),
                read: self.scope.contains(&"write:packages".to_owned())
                    || self.scope.contains(&"read:packages".to_owned()),
            },
            delete_packages: self.scope.contains(&"delete:packages".to_owned()),
            org: GithubScopeAdminLevel {
                admin: self.scope.contains(&"admin:org".to_owned()),
                write: self.scope.contains(&"admin:org".to_owned())
                    || self.scope.contains(&"write:org".to_owned()),
                read: self.scope.contains(&"admin:org".to_owned())
                    || self.scope.contains(&"write:org".to_owned())
                    || self.scope.contains(&"read:org".to_owned()),
            },
            public_key: GithubScopeAdminLevel {
                admin: self.scope.contains(&"admin:public_key".to_owned()),
                write: self.scope.contains(&"admin:public_key".to_owned())
                    || self.scope.contains(&"write:public_key".to_owned()),
                read: self.scope.contains(&"admin:public_key".to_owned())
                    || self.scope.contains(&"write:public_key".to_owned())
                    || self.scope.contains(&"read:public_key".to_owned()),
            },
            repo_hook: GithubScopeAdminLevel {
                admin: self.scope.contains(&"admin:repo_hook".to_owned()),
                write: self.scope.contains(&"admin:repo_hook".to_owned())
                    || self.scope.contains(&"write:repo_hook".to_owned()),
                read: self.scope.contains(&"admin:repo_hook".to_owned())
                    || self.scope.contains(&"write:repo_hook".to_owned())
                    || self.scope.contains(&"read:repo_hook".to_owned()),
            },
            org_hook: self.scope.contains(&"admin:org_hook".to_owned()),
            gist: self.scope.contains(&"gist".to_owned()),
            notifications: self.scope.contains(&"notifications".to_owned()),
            user: GithubScopeUser {
                all: self.scope.contains(&"user".to_owned()),
                read: self.scope.contains(&"user".to_owned())
                    || self.scope.contains(&"read:user".to_owned()),
                email: self.scope.contains(&"user".to_owned())
                    || self.scope.contains(&"user:email".to_owned()),
                follow: self.scope.contains(&"user".to_owned())
                    || self.scope.contains(&"user:follow".to_owned()),
            },
            delete_repo: self.scope.contains(&"delete_repo".to_owned()),
            discussion: GithubScopeLevel {
                write: self.scope.contains(&"write:discussion".to_owned()),
                read: self.scope.contains(&"write:discussion".to_owned())
                    || self.scope.contains(&"read:discussion".to_owned()),
            },
            enterprise: GithubScopeEnterprise {
                all: self.scope.contains(&"admin:enterprise".to_owned()),
                manage_runners: self.scope.contains(&"admin:enterprise".to_owned())
                    || self.scope.contains(&"manage_runners:enterprise".to_owned()),
                manage_billing: self.scope.contains(&"admin:enterprise".to_owned())
                    || self.scope.contains(&"manage_billing:enterprise".to_owned()),
                read: self.scope.contains(&"admin:enterprise".to_owned())
                    || self.scope.contains(&"manage_billing:enterprise".to_owned())
                    || self.scope.contains(&"read:enterprise".to_owned()),
            },
            gpg_key: GithubScopeAdminLevel {
                admin: self.scope.contains(&"admin:gpg_key".to_owned())
                    || self.scope.contains(&"manage_runners:enterprise".to_owned()),
                write: self.scope.contains(&"admin:gpg_key".to_owned())
                    || self.scope.contains(&"write:gpg_key".to_owned()),
                read: self.scope.contains(&"admin:gpg_key".to_owned())
                    || self.scope.contains(&"write:gpg_key".to_owned())
                    || self.scope.contains(&"read:gpg_key".to_owned()),
            },
            ssh_signing_key: GithubScopeAdminLevel {
                admin: self.scope.contains(&"admin:ssh_signing_key".to_owned()),
                write: self.scope.contains(&"admin:ssh_signing_key".to_owned())
                    || self.scope.contains(&"write:ssh_signing_key".to_owned()),
                read: self.scope.contains(&"admin:ssh_signing_key".to_owned())
                    || self.scope.contains(&"write:ssh_signing_key".to_owned())
                    || self.scope.contains(&"read:ssh_signing_key".to_owned()),
            },
        }
    }

    fn add_authorization_token(&self, token: &str) -> HeaderValue {
        HeaderValue::from_str(format!("token {}", token).as_ref()).unwrap()
    }
}
