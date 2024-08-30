use std::process::Command;
use color_eyre::eyre::Result;

pub fn trinity_helios_part_one() -> Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg("../../scripts/infra/trinity_helios_part_one.sh")
        .status()?;

    Ok(())
}

pub fn trinity_helios_part_two() -> Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg("../../scripts/infra/trinity_helios_part_two.sh")
        .status()?;

    Ok(())
}
