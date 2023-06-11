use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmartContractStanderd {
    ESC20,
    ESC721,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SmartContractApi {
    ESC20 {
        publisher: String,
        total_suply: u128,
        transfer: fn(from: String, to: String, amount: u128) -> Result<(), String>,
    },
}

impl SmartContractStanderd {
    pub fn from(str: &str) -> SmartContractStanderd {
        match str {
            "ESC-20" | "esc-20" | "ESC20" | "esc20" => SmartContractStanderd::ESC20,
            "ESC-721" | "esc-721" | "ESC721" | "esc721" => SmartContractStanderd::ESC721,
            _ => unreachable!(),
        }
    }

    pub fn to(&self) -> &str {
        match self {
            SmartContractStanderd::ESC20 => "ESC20",
            SmartContractStanderd::ESC721 => "ESC721",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmartContract {
    pub r#type: SmartContractStanderd,
    pub api: SmartContractApi,
}

impl SmartContract {
    pub fn new(r#type: SmartContractStanderd, api: SmartContractApi) -> SmartContract {
        SmartContract { r#type, api }
    }

    pub fn execute_fn(&mut self, fun: &str, params: Vec<&str>) {
        match fun {
            "transfer" => match self.api {
                SmartContractApi::ESC20 { transfer, .. } => {
                    transfer(
                        params[0].to_string(),
                        params[1].to_string(),
                        params[2].parse().unwrap(),
                    )
                    .unwrap();
                }
            },
            _ => unreachable!(),
        }
    }
}
