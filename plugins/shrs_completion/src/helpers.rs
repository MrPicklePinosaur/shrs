use std::{fs::File, io::BufReader, path::Path};

use ssh2_config::{ParseRule, SshConfig};

/// Get known hosts from ssh config file
pub fn known_hosts(config_path: &Path) -> anyhow::Result<()> {
    let config_file = File::open(config_path)?;
    let mut reader = BufReader::new(config_file);
    let conf = SshConfig::default().parse(&mut reader, ParseRule::ALLOW_UNKNOWN_FIELDS)?;
    for host in conf.get_hosts() {
        for pat in &host.pattern {
            println!("{}", pat.pattern);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::known_hosts;

    #[test]
    fn test_known_hosts() {
        let mut config_path = dirs::home_dir().unwrap();
        config_path.push(".ssh/config");
        known_hosts(&config_path).unwrap();
    }
}
