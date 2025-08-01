use serde::ser::SerializeStruct;
use std::{
    ops::Deref, process::{Command, Output}, str
};

use crate::errors;

pub struct CommandOutput(Output);

impl Deref for CommandOutput {
    type Target = Output;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[tauri::command]
pub async fn execute(cmd: &str, args: Vec<&str>) -> Result<CommandOutput, errors::Error> {
    let res = Command::new(cmd).args(args).output()?;
    Ok(CommandOutput(res))
}

#[tauri::command]
pub async fn terminate(process: &str) -> Result<(), errors::Error> {
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .arg("/IM")
            .arg(format!("{process}.exe"))
            .arg("/F")
            .status()?;
    }
    #[cfg(not(windows))]
    {
        let _ = Command::new("pkill").args(["-f", process]).status()?;
    }
    Ok(())
}

impl serde::Serialize for CommandOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let out = &self.0;
        let mut res = serializer.serialize_struct("Output", 3)?;
        res.serialize_field("status", &out.status.code())?;
        res.serialize_field("stdout", &out.stdout)?;
        res.serialize_field("stderr", &out.stderr)?;
        res.end()
    }
}
