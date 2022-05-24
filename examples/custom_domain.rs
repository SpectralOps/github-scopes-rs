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
    let permissions = match OAuthContext::with_domain(token.as_ref(), "https://example.com") {
        Ok(s) => s.get_scope_permissions(),
        Err(e) => return Err(e),
    };

    if !permissions.repo.all {
        return Err(anyhow!("missing repo access"));
    }
    Ok(())
}
