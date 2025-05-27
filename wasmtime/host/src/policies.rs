
#[derive(clap::Parser, Debug)]
pub struct Config {
    #[arg(short, long)]
    policy_file: String,
    
    #[arg(short, long)]
    component: String
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Mode {
    Input,
    Output,
    InputOutput,
    StatefulOutput
}

#[derive(serde::Deserialize, Debug)]
pub struct WasiGpioEntry {
    vlabel: String,
    modes: Vec<Mode>,
    plabel: String
}

#[derive(serde::Deserialize, Debug)]
pub struct Wasi {
    gpio: Vec<WasiGpioEntry>
}

#[derive(serde::Deserialize, Debug)]
pub struct Policies {
    wasi: Wasi
}

impl Config {
    pub fn get_policies(&self) -> Policies {
        let entries = std::fs::read_to_string(&self.policy_file).unwrap();
        
        match toml::from_str(&entries) {
            Ok(e) => e,
            Err(e) => panic!("{}", e.message()),
        }
    }
    
    pub fn get_component_path(&self) -> &str {
        &self.component
    }
}

impl Policies {
    fn find(&self, vlabel: &str) -> Option<&WasiGpioEntry> {
        for entry in self.wasi.gpio.iter() {
            if vlabel.eq(&entry.vlabel) {
                return Some(entry);
            }
        }
        
        None
    }
    
    pub fn get_plabel(&self, vlabel: &str) -> Option<String> {
        self.find(vlabel).map(|entry| entry.plabel.clone())
    }
    
    pub fn is_mode_allowed(&self, vlabel: &str, mode: Mode) -> bool {
        let entry = match self.find(vlabel) {
            Some(entry) => entry,
            None => return false,
        };
        
        for allowed_mode in &entry.modes {
            if mode == *allowed_mode {
                return true;
            }
        }
        
        false
    }
}