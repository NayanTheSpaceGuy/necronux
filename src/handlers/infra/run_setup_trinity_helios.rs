use std::{io::Error, process::Command};

pub fn trinity_helios_part_one() -> Result<(), Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("../../scripts/infra/trinity_helios_part_one.sh")
        .status()?;
    Ok(())
}

pub fn trinity_helios_part_two() -> Result<(), Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("../../scripts/infra/trinity_helios_part_two.sh")
        .status()?;
    Ok(())
}
