use anyhow::Result;
use self_update::cargo_crate_version;

/// Check for and apply updates from GitHub.
pub fn update() -> Result<()> {
    println!("Checking for updates...");

    let mut builder = self_update::backends::github::Update::configure();
    builder
        .repo_owner("impulia")
        .repo_name("neuroflow")
        .bin_name("neflo")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .no_confirm(true)
        .target("macos");

    if let Ok(token) =
        std::env::var("NEFLO_GITHUB_TOKEN").or_else(|_| std::env::var("GITHUB_TOKEN"))
    {
        builder.auth_token(&token);
    }

    let status = builder.build()?.update()?;

    if status.updated() {
        println!("Successfully updated to version {}!", status.version());
    } else {
        println!("Already up to date (version {})!", status.version());
    }

    Ok(())
}
