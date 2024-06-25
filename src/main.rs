use serde::{Deserialize, Deserializer, Serialize};

impl<'de> Deserialize<'de> for CommandResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CommandWrapper {
            error: Option<String>,
            events: Vec<String>,
        }

        #[derive(Deserialize)]
        struct DataWrapper {
            command: CommandWrapper,
        }

        #[derive(Deserialize)]
        struct RootWrapper {
            data: DataWrapper,
        }

        let root = RootWrapper::deserialize(deserializer)?;

        Ok(CommandResult {
            events: root.data.command.events,
            error: root.data.command.error,
        })
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
