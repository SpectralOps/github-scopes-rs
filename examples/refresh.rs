use anyhow::anyhow;
use anyhow::Result as AnyResult;
use github_scopes_rs::oauth::OAuthContext;

fn main() -> AnyResult<()> {
    let mut scope = match OAuthContext::new("some-token".to_string()) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    scope.refresh(Some("new-token".to_string()))?;
    let permissions = scope.get_scope_permissions();

    if !permissions.repo.all {
        return Err(anyhow!("token has not full repo access"));
    }
    Ok(())
}
