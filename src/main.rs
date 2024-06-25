use serde::{Deserialize, Deserializer, Serialize};

impl<'de> Deserialize<'de> for CommandResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn derived_impl<'de, D>(deserializer: D) -> Result<CommandResult, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            <CommandResult>::deserialize(deserializer)
        }

        #[derive(Deserialize)]
        struct DataWrapper {
            data: CommandWrapper,
        }

        #[derive(Deserialize)]
        struct CommandWrapper {
            #[serde(deserialize_with = "derived_impl")]
            command: CommandResult,
        }

        let wrapper = DataWrapper::deserialize(deserializer)?;

        Ok(wrapper.data.command)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandResult {
    pub events: Vec<String>,
    pub error: Option<String>,
}

fn main() {
    let json = r#"{
      "data": {
        "command": {
          "error": null,
          "events": ["placeholder"]
        }
      }
    }"#;
    let result = serde_json::from_str::<CommandResult>(json);
    println!("{:?}", result);
}
