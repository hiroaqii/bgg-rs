use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::errors::BggError;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Item {
    objectid: String,
    subtype: String,
    primaryname: String,
    nameid: String,
    yearpublished: Number,
    ordtitle: String,
    rep_imageid: u32,
    objecttype: String,
    name: String,
    sortindex: String,
    r#type: String,
    id: String,
    href: String,
}

pub type BggResult<T> = core::result::Result<T, BggError>;
