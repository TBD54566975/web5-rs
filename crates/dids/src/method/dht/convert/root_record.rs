use pkarr::dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{ConvertError, DEFAULT_TTL};

#[derive(Debug)]
pub struct RootRecord {
    vm: Vec<String>,
    srv: Vec<String>,
    inv: Vec<String>,
    del: Vec<String>,
    auth: Vec<String>,
    agm: Vec<String>,
    asm: Vec<String>,
}

impl RootRecord {
    pub fn new() -> Self {
        RootRecord {
            vm: Vec::new(),
            srv: Vec::new(),
            inv: Vec::new(),
            del: Vec::new(),
            auth: Vec::new(),
            agm: Vec::new(),
            asm: Vec::new(),
        }
    }

    pub fn add_vm_record_name(&mut self, idx: u32) {
        self.vm.push(format!("k{}", idx));
    }

    pub fn add_asm_record_name(&mut self, idx: u32) {
        self.asm.push(format!("k{}", idx));
    }

    pub fn add_inv_record_name(&mut self, idx: u32) {
        self.inv.push(format!("k{}", idx));
    }

    pub fn add_del_record_name(&mut self, idx: u32) {
        self.del.push(format!("k{}", idx));
    }

    pub fn add_auth_record_name(&mut self, idx: u32) {
        self.auth.push(format!("k{}", idx));
    }

    pub fn add_agm_record_name(&mut self, idx: u32) {
        self.agm.push(format!("k{}", idx));
    }

    pub fn add_srv_record_name(&mut self, idx: u32) {
        self.srv.push(format!("s{}", idx));
    }

    pub fn to_txt_record(&self, did_id: &str) -> Result<ResourceRecord, ConvertError> {
        let fields = [
            ("vm", &self.vm),
            ("asm", &self.asm),
            ("inv", &self.inv),
            ("del", &self.del),
            ("auth", &self.auth),
            ("agm", &self.agm),
            ("srv", &self.srv),
        ];

        let parts = fields
            .iter()
            .filter_map(|(name, values)| {
                if !values.is_empty() {
                    Some(format!("{}={}", name, values.join(",")))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(";");

        let name = Name::new_unchecked(&format!("_did.{}", did_id)).into_owned();

        let txt_record = TXT::new().with_string(&parts)?.into_owned();

        Ok(ResourceRecord::new(
            name,
            pkarr::dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }
}
