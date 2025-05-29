
#[derive(clap::Parser, Debug)]
pub struct Config {
    #[arg(short, long)]
    policy_file: String,
    
    #[arg(short, long)]
    component: String,

    #[arg(short, long)]
    pi_type: u8
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    DigitalInput,
    DigitalOutput,
    StatefulDigitalOutput,
    DigitalInputOutput,
    AnalogInput,
    AnalogOutput,
    AnalogInputOutput
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

    pub fn get_pi_type(&self) -> u8 {
        self.pi_type
    }
}

impl Policies {
    fn validate(&self) {
        for entry in self.wasi.gpio.iter() {
            for mode in entry.modes.iter() {
                match mode {
                    Mode::AnalogOutput => {
                        match entry.plabel.as_str() {
                            "PWM0" | "PWM1" | "PWM2" | "PWM3" => {},
                            _ => panic!("Invalid PWM channel: {}", entry.plabel)
                        }
                    },
                    Mode::AnalogInput | Mode::AnalogInputOutput => panic!("Analog input not supported on Raspberry Pi"),
                    _ => {}
                }
            }
        }
    }

    fn find(&self, vlabel: &str) -> Option<&WasiGpioEntry> {
        for entry in self.wasi.gpio.iter() {
            if vlabel.eq(&entry.vlabel) {
                return Some(entry);
            }
        }
        
        None
    }

    pub fn get_plabel(&self, vlabel: &str) -> Option<String> {
        let plabel = match self.find(vlabel).map(|entry| entry.plabel.clone()) {
            Some(plabel) => plabel,
            None => return None,
        };

        if plabel.starts_with("GPIO") {
            return Some(plabel)
        }

        None
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