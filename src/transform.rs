//! Transform GitHub permission to structs

/// All token scope data
pub struct GithubTokenScope {
    pub repo: GithubScopeRepo,
    pub workflow: bool,
    pub packages: GithubScopeLevel,
    pub delete_packages: bool,
    pub org: GithubScopeAdminLevel,
    pub public_key: GithubScopeAdminLevel,
    pub repo_hook: GithubScopeAdminLevel,
    pub org_hook: bool,
    pub gist: bool,
    pub notifications: bool,
    pub user: GithubScopeUser,
    pub delete_repo: bool,
    pub discussion: GithubScopeLevel,
    pub enterprise: GithubScopeEnterprise,
    pub gpg_key: GithubScopeAdminLevel,
}

/// GitHub scope repo
pub struct GithubScopeRepo {
    pub all: bool,
    pub status: bool,
    pub deployment: bool,
    pub public_repo: bool,
    pub invite: bool,
    pub security_events: bool,
}

/// GitHub admin scope with admin, write and read access
pub struct GithubScopeAdminLevel {
    pub admin: bool,
    pub write: bool,
    pub read: bool,
}

/// GitHub scope level with write and read access
pub struct GithubScopeLevel {
    pub write: bool,
    pub read: bool,
}

/// Github user scope
pub struct GithubScopeUser {
    pub all: bool,
    pub email: bool,
    pub follow: bool,
    pub read: bool,
}

/// GitHub enterprise scope
pub struct GithubScopeEnterprise {
    pub all: bool,
    pub manage_runners: bool,
    pub manage_billing: bool,
    pub read: bool,
}
