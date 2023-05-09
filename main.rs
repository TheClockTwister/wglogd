use serde::{Serialize, Deserialize};
use std::process::Command;
use std::collections::HashSet;
use std::{thread, time};
use std::env;
use std::fs;
use std::process::exit;
use toml;
use std::io::Write;


#[derive(Serialize, Deserialize, Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Clone)]
struct WgDump {
    interface: String,
    public_key: String,
    endpoint_ip: String,
    endpoint_port: u16,
    allowed_ips: String,
}

#[derive(Deserialize)]
struct Config {
    log_file: String,
    interval_ms: u64,
}


/// Returns any changes in peers
fn get_difference(old_peers: &mut HashSet<WgDump> , new_peers: &mut HashSet<WgDump>) -> HashSet<WgDump> {

    let output = Command::new("wg").arg("show").arg("all").arg("dump").output().expect("Failed to execute command");
    let output_string = String::from_utf8_lossy(&output.stdout).to_string();

    // parse all WireGuard peer lines
    for line in output_string.lines() {

        // sanitize the line and check if it contains the 9 elements
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 9 { continue; }

        let endpoint = String::from(parts[3].to_string());
        
        
        // let (ip, port) = endpoint.split_once(":").unwrap();

        let (ip, port) = match endpoint.split_once(":") {
            Some((a, b)) => (a.to_string(), b.to_string().parse::<u16>().unwrap()),
            None => (String::from("(none)"), 0),
        };

        // parse peer and add to list
        let wg_dump = WgDump {
            interface: parts[0].to_string(),
            public_key: parts[1].to_string(),
            endpoint_ip: ip,
            endpoint_port: port,
            allowed_ips: parts[4].to_string()
        };
        // result.push(wg_dump);
        new_peers.insert(wg_dump);
    }

    let diff = new_peers.difference(&old_peers).cloned().collect();

    old_peers.drain();
    old_peers.extend(new_peers.drain());

    return diff;
}


fn load_config() -> Config {

    let config_path = env::var("WGLOGD_CONF_PATH").unwrap_or(String::from("/etc/wglogd/config.toml"));
    let contents = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config file `{}`", config_path);
            exit(1);
        }
    };

    let config: Config = toml::from_str(&contents).unwrap();
    return config;
}


fn main() {

    let config = load_config();

    let mut file = match fs::OpenOptions::new().create(true).write(true).append(true).open(config.log_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not write to log file!");
            exit(1);
        }
    };

    let mut old_peers: HashSet<WgDump> = HashSet::new();
    let mut new_peers: HashSet<WgDump> = HashSet::new();

    // execute it once to set old_peers to current state
    get_difference(&mut old_peers, &mut new_peers);

    loop {
        let diff: HashSet<WgDump> = get_difference(&mut old_peers, &mut new_peers);
        for x in diff {
            let json = serde_json::to_string(&x).unwrap();
            if let Err(e) = writeln!(file, "{}", json) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        
        thread::sleep(time::Duration::from_millis(config.interval_ms));
    }
}
