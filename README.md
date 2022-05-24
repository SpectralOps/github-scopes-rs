# github-scopes-rs

Discover GitHub token scope permission and return you an easy interface for checking token permission before querying GitHub.
In many cases, you try to do actions to GitHub, but you get unclear permissions errors. This project allows you to get which permission your token has before, called GitHub, and if you don’t have the right permissions, you can tell the user the exact permission the user needs.

## How it works
We called Github api with the given token and get which permissions scope the token has in order the access to the API. Then, the permissions are being converted to a simple object that you can work with.
click [here](https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps) read GitHub documentation.


## Usage
Add this to Cargo.toml:
```toml
[dependencies]
github-scopes-rs = { version = "1.0.0" }
```

Here's a simple example
```rs
fn main() -> AnyResult<()> {
    let permissions = match OAuthContext::new("token") {
        Ok(s) => s.get_scope_permissions(),
        Err(e) => return Err(e),
    };

    if !permissions.repo.all {
        return Err(anyhow!("`repo` permission is mandatory"));
    }
    Ok(())
}
```

You can run it by cloning this repo, and then:
```sh
GITHUB_TOKEN=<TOKEN> cargo run --example base
```