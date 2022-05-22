# github-scopes-rs

discover GitHub token scope permission and return you an easy interface for checking token permission before querying GitHub.

## Example
```rs
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
```
