use std::cell::RefCell;
use std::rc::Rc;

use serde_json::Value;
use thiserror::Error;

use super::default_input_text_plugin::{DefaultInputTextPlugin, DefaultInputTextPluginSetupErr};
use super::prolonged_soundmark_input_text_plugin::ProlongedSoundMarkInputTextPlugin;
use crate::config::Config;
use crate::dictionary_lib::grammar::Grammar;
use crate::utf8_input_text_builder::{ReplaceErr, UTF8InputTextBuilder};

pub trait InputTextPlugin<G = Rc<RefCell<Grammar>>> {
  fn setup(&mut self) -> Result<(), InputTextPluginSetupErr>;
  fn rewrite(&self, builder: &mut UTF8InputTextBuilder<G>)
    -> Result<(), InputTextPluginReplaceErr>;
}

#[derive(Error, Debug)]
pub enum InputTextPluginGetErr {
  #[error("{0} is invalid InputTextPlugin class")]
  InvalidClassErr(String),
  #[error("config file is invalid format")]
  InvalidFormatErr,
}

#[derive(Error, Debug)]
pub enum InputTextPluginSetupErr {
  #[error("{self:?}")]
  DefaultInputTextPluginSetupErr(#[from] DefaultInputTextPluginSetupErr),
}

#[derive(Error, Debug)]
pub enum InputTextPluginReplaceErr {
  #[error("{self:?}")]
  ReplaceErr(#[from] ReplaceErr),
}

fn get_input_text_plugin(
  config: &Config,
  json_obj: &Value,
) -> Result<Box<dyn InputTextPlugin>, InputTextPluginGetErr> {
  if let Some(Value::String(class)) = json_obj.get("class") {
    if class == "sudachipy.plugin.input_text.DefaultInputTextPlugin" {
      Ok(Box::new(DefaultInputTextPlugin::new(config)))
    } else if class == "sudachipy.plugin.input_text.ProlongedSoundMarkInputTextPlugin" {
      Ok(Box::new(ProlongedSoundMarkInputTextPlugin::new(json_obj)))
    } else {
      Err(InputTextPluginGetErr::InvalidClassErr(class.to_string()))
    }
  } else {
    Err(InputTextPluginGetErr::InvalidFormatErr)
  }
}

pub fn get_input_text_plugins(
  config: &Config,
) -> Result<Vec<Box<dyn InputTextPlugin>>, InputTextPluginGetErr> {
  let mut plugins = vec![];
  if let Some(Value::Array(arr)) = config.settings.get("inputTextPlugin") {
    for v in arr {
      plugins.push(get_input_text_plugin(config, v)?);
    }
  }
  Ok(plugins)
}