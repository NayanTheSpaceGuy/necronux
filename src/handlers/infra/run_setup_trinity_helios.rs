use std::{fs::Permissions, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};
use color_eyre::eyre::{eyre, Result};
use log::debug;

pub fn trinity_helios_part_one() -> Result<()> {

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script_path = project_root.join("src/scripts/infra/trinity_helios_setup_part_one.sh");

    debug!("Attempting to change permissions of script: {:?}", script_path);
    std::fs::set_permissions(&script_path, Permissions::from_mode(0o755))?;
    debug!("Successfully changed permissions of the script");

    debug!("Preparing to run trinity_helios_part_one script");
    let script_str = script_path.to_str()
        .ok_or_else(|| eyre!("Failed to convert script path to string"))?;

    debug!("Executing script: {}", script_str);
    let execute = Command::new("sh")
        .arg("-c")
        .arg(script_str)
        .status()?;

    if !execute.success() {
        return Err(eyre!("Script execution failed with status: {}", execute));
    }

    debug!("Script executed successfully");
    Ok(())
}

pub fn trinity_helios_part_two() -> Result<()> {

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script_path = project_root.join("src/scripts/infra/trinity_helios_setup_part_two.sh");

    debug!("Attempting to change permissions of script: {:?}", script_path);
    std::fs::set_permissions(&script_path, Permissions::from_mode(0o755))?;
    debug!("Successfully changed permissions of the script");

    debug!("Preparing to run trinity_helios_part_two script");
    let script_str = script_path.to_str()
        .ok_or_else(|| eyre!("Failed to convert script path to string"))?;

    debug!("Executing script: {}", script_str);
    let execute = Command::new("sh")
        .arg("-c")
        .arg(script_str)
        .status()?;

    if !execute.success() {
        return Err(eyre!("Script execution failed with status: {}", execute));
    }

    debug!("Script executed successfully");
    Ok(())
}
