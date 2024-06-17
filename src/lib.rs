use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        version
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![&format!(
            "type depot > /dev/null || pkgx install depot@{}",
            version
        )])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("build")?
        .pkgx()?
        .with_packages(vec!["depot"])?
        .with_exec(vec!["depot", "build", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn bake(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("bake")?
        .pkgx()?
        .with_packages(vec!["depot"])?
        .with_exec(vec!["depot", "bake", &args])?
        .stdout()?;
    Ok(stdout)
}
