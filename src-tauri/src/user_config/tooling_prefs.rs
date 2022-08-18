use chrono::prelude::*;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tool {
    tool_name: String,
    extensions_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ToolingPrefs {
    tools_supported: HashMap<String, Tool>,
    tools_installed: HashMap<String, Tool>,
}

// generic helper functions
impl ToolingPrefs {
    fn get_tools_supported() -> HashMap<String, Tool> {
        let mut tools_supported: HashMap<String, Tool> = HashMap::new();

        let tools_names = vec!["JavaScript", "Java", "Python", "Cpp"];

        for tool_name in tools_names {
            tools_supported.insert(
                tool_name.to_lowercase(),
                Tool {
                    tool_name: tool_name.to_string(),
                    extensions_list: vec![],
                },
            );
        }

        tools_supported
    }
}

// methods
impl ToolingPrefs {
    fn new() -> Self {
        Self {
            tools_supported: Self::get_tools_supported(),
            tools_installed: HashMap::new(),
        }
    }

    fn get_tool_details(self: &Self, tool_id: &str) -> Result<&Tool, ()> {
        if self.tools_supported.contains_key(tool_id) == true {
            Ok(self.tools_supported.get(tool_id).unwrap())
        } else {
            // TODO: throw some error message here, with my global error handling
            Err(())
        }
    }
}

// config load and store
impl ToolingPrefs {
    fn load_config(self: &mut Self) -> Result<(), ConfyError> {
        let cfg: ToolingPrefs = confy::load("tooling_prefs")?;

        self.tools_installed = cfg.tools_installed;
        Ok(())
    }

    fn store_config(self: &Self) -> Result<(), ConfyError> {
        let older_config_result: Result<ToolingPrefs, ConfyError> = confy::load("nebula");

        if let Ok(older_config) = older_config_result {
            if (older_config.tools_supported.is_empty() != false) {
                // this is not defaulted to empty hashmap, meaning there was some older config present

                let local_time = Local::now();

                // backing up older configs
                confy::store(
                    &format!(
                        // !ISSUE: confy does not allow for backups as directories..
                        /*
                        when we try to add name here as "abc/my_config",
                        it tries to create directory abc and inside that,
                        a directory called my_config and then adds the file named as "abc/my_config/my_config.toml"
                        */
                        "nebula_backup_{}",
                        local_time.format("%d_%m_%Y_%H-%M-%S").to_string()
                    ),
                    older_config,
                )?;
            }
        }

        confy::store("nebula", self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_store_tooling_pref_config() {}
}
