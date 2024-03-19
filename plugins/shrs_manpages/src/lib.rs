use shrs::prelude::*;

pub struct ManPagesPlugin;

/// Grab the current line and attempt to open man page of command
pub fn open_manpage() {
    
}

impl Plugin for ManPagesPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {

        Ok(())
    }
}
