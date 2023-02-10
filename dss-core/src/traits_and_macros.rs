use crate::imports::*;
use bincode::{deserialize, serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

/// assumes heat flow from source -> sink is positive
/// calculates flow variable values
#[macro_export]
macro_rules! connect_states {
    ($sys: ident, ($($s0: ident, $s1: ident, $c: ident), +), $dt: ident) => {
        // update flow variables
        $(
            $sys.$c.set_flow(&$sys.$s0.state, &$sys.$s1.state);
        )+
    };
}

#[macro_export]
macro_rules! update_states {
    ($sys: ident, ($($s0: ident, $s1: ident, $c: ident), +), $dt: ident) => {
        // update state variables
        $(
            $sys.$s0.state.step_pot(-$sys.$c.flow() * $dt / $sys.$s0.c);
            $sys.$s1.state.step_pot($sys.$c.flow() * $dt / $sys.$s1.c);
        )+
    };
}

pub trait Potential {
    /// sets value `val` of potential variable (e.g. temperature, pressure, voltage)
    fn set_pot(&mut self, val: f64);
    /// returns value of potential variable (e.g. temperature, pressure, voltage)
    fn pot(&self) -> f64;
    /// increments value of potential by `val`
    fn step_pot(&mut self, val: f64) {
        self.set_pot(self.pot() + val);
    }
}

pub trait Flow {
    /// Sets flow variable based on difference between two potential variables
    fn set_flow(&mut self, p0: &dyn Potential, p1: &dyn Potential);
    /// returns value of flow variable (e.g. heat transfer, fluid flow rate, electrical current)
    fn flow(&self) -> f64;
}

pub trait Diff {
    fn diff(&self) -> Vec<f64>;
}

impl Diff for Vec<f64> {
    fn diff(&self) -> Vec<f64> {
        self.windows(2)
            .map(|vs| {
                let [x, y] = vs else {unreachable!()};
                y - x
            })
            .collect()
    }
}

pub trait SerdeAPI: Serialize + for<'a> Deserialize<'a> {
    #[allow(clippy::wrong_self_convention)]
    /// Save current data structure to file. Method adaptively calls serialization methods
    /// dependent on the suffix of the file given as str.
    ///
    /// # Argument:
    ///
    /// * `filename`: a `str` storing the targeted file name. Currently `.json` and `.yaml` suffixes are
    /// supported
    ///
    /// # Returns:
    ///
    /// A Rust Result
    fn to_file(&self, filename: &str) -> Result<(), anyhow::Error> {
        let file = PathBuf::from(filename);
        let extension = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");
        let res = match extension {
            "json" => {
                serde_json::to_writer(&File::create(file)?, self)?;
                Ok(())
            }
            "yaml" => {
                serde_yaml::to_writer(&File::create(file)?, self)?;
                Ok(())
            }
            _ => Err(anyhow!("Unsupported file extension {}", extension)),
        };
        res
    }

    /// Read from file and return instantiated struct. Method adaptively calls deserialization
    /// methods dependent on the suffix of the file name given as str.
    /// Function returns a dynamic Error Result if it fails.
    ///
    /// # Argument:
    ///
    /// * `filename`: a `str` storing the targeted file name. Currently `.json` and `.yaml` suffixes are
    /// supported
    ///
    /// # Returns:
    ///
    /// A Rust Result wrapping data structure if method is called successfully; otherwise a dynamic
    /// Error.
    fn from_file(filename: &str) -> Result<Self, anyhow::Error>
    where
        Self: std::marker::Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let extension = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        let file = File::open(filename)?;
        match extension {
            "yaml" => Ok(serde_yaml::from_reader(file)?),
            "json" => Ok(serde_json::from_reader(file)?),
            _ => Err(anyhow!("Unsupported file extension {}", extension)),
        }
    }

    /// json serialization method.
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// json deserialization method.
    fn from_json(json_str: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_str(json_str)?)
    }

    /// yaml serialization method.
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// yaml deserialization method.
    fn from_yaml(yaml_str: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_yaml::from_str(yaml_str)?)
    }

    /// bincode serialization method.
    fn to_bincode(&self) -> Vec<u8> {
        serialize(&self).unwrap()
    }

    /// bincode deserialization method.
    fn from_bincode(encoded: &[u8]) -> Result<Self, anyhow::Error> {
        Ok(deserialize(encoded)?)
    }
}

impl<T> SerdeAPI for T where T: Serialize + for<'a> Deserialize<'a> {}
