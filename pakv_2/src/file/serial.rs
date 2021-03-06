use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum KvOpeE {
    KvOpeSet { k: String, v: String },
    KvOpeDel { k: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KvOpe {
    // pub opetype:u8,
    // pub time: u32,
    pub ope: KvOpeE,
}

impl KvOpe {
    pub fn from_str(str_: &str) -> serde_json::Result<KvOpe> {
        serde_json::from_str(str_)
    }
    pub fn to_line_str(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaFileStore{
    pub usertarfid:u64
}
impl MetaFileStore{
    pub fn default() -> MetaFileStore {
        MetaFileStore{
            usertarfid:1
        }
    }
}