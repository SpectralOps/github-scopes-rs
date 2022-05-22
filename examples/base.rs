use anyhow::anyhow;
use anyhow::Result as AnyResult;
use github_scopes_rs::oauth::OAuthContext;

fn main() -> AnyResult<()> {
    let permissions = match OAuthContext::new("some-token".to_string()) {
        Ok(s) => s.get_scope_permissions(),
        Err(e) => return Err(e),
    };

    if !permissions.repo.all {
        return Err(anyhow!("token has not full repo access"));
    }
    Ok(())
}
