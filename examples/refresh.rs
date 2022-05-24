use anyhow::anyhow;
use anyhow::Result as AnyResult;
use github_scopes_rs::oauth::OAuthContext;
use std::env;
use std::process::exit;

fn main() -> AnyResult<()> {
    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) => t,
        Err(_e) => {
            println!("github token not provide");
            exit(1)
        }
    };

    let mut scope = match OAuthContext::new(token.as_ref()) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    scope.refresh(Some("new-token".to_string()))?;
    let permissions = scope.get_scope_permissions();

    if !permissions.repo.all {
        return Err(anyhow!("missing repo access"));
    }
    Ok(())
}
