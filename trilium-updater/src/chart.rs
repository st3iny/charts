use std::{fs::File, io::Write, path::Path};

use anyhow::Result;
use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    api_version: String,

    pub version: Version,
    pub app_version: Version,

    #[serde(flatten)]
    _rest: IndexMap<Value, Value>,
}

impl Chart {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let chart_file = File::open(path)?;
        let chart: Chart = serde_yaml::from_reader(chart_file)?;
        Ok(chart)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"---\n")?;
        serde_yaml::to_writer(&mut file, self)?;
        Ok(())
    }
}
