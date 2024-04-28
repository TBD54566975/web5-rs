use std::collections::HashMap;

use pkarr::dns::{Packet, ResourceRecord};

use crate::document::Document;

use super::{root_record::RootRecord, ConvertError};

pub fn document_to_packet(document: &Document) -> Result<Packet<'static>, ConvertError> {
    // 0. Init root_record and empty answers array
    let mut root_record = RootRecord::new();
    let mut answers: Vec<ResourceRecord> = vec![];
    let mut vm_id_to_idx: HashMap<String, u32> = HashMap::new();

    // 1. Create answers
    // 1.1 Add all verification methods to root_record and answers
    document
        .verification_method
        .iter()
        .enumerate()
        .try_for_each(|(idx, vm)| -> Result<(), ConvertError> {
            let idx = idx as u32;

            let vm_record = vm.to_resource_record(idx)?;
            answers.push(vm_record);

            root_record.add_vm_record_name(idx);

            vm_id_to_idx.insert(vm.id, idx);

            Ok(())
        })?;

    // 1.2 Add assertion methods to root_record
    if let Some(assertion_method) = document.assertion_method {
        assertion_method.iter().for_each(|am| {
            if let Some(idx) = vm_id_to_idx.get(am) {
                root_record.add_del_record_name(*idx)
            }
        });
    }

    // 1.3 Add authentication methods to root_record
    if let Some(authentication) = document.authentication {
        authentication.iter().for_each(|auth| {
            if let Some(idx) = vm_id_to_idx.get(auth) {
                root_record.add_auth_record_name(*idx)
            }
        });
    }

    // 1.4 Add capability delegations to root_record
    if let Some(capability_delegation) = document.capability_delegation {
        capability_delegation.iter().for_each(|auth| {
            if let Some(idx) = vm_id_to_idx.get(auth) {
                root_record.add_del_record_name(*idx)
            }
        });
    }

    // 1.5 Add capability invocations to root_record
    if let Some(capability_invocation) = document.capability_invocation {
        capability_invocation.iter().for_each(|inv| {
            if let Some(idx) = vm_id_to_idx.get(inv) {
                root_record.add_inv_record_name(*idx)
            }
        });
    }

    // 1.6 Add agm methods to root_record
    if let Some(key_agreement) = document.key_agreement {
        key_agreement.iter().for_each(|agm| {
            if let Some(idx) = vm_id_to_idx.get(agm) {
                root_record.add_agm_record_name(*idx)
            }
        });
    }

    // 1.7 Add service records to root_record and answers
    if let Some(service) = document.service {
        service
            .iter()
            .enumerate()
            .try_for_each(|(idx, src)| -> Result<(), ConvertError> {
                let idx = idx as u32;
                let service_record = src.to_resource_record(idx)?;
                answers.push(service_record);
                root_record.add_srv_record_name(idx);

                Ok(())
            })?;
    }

    // 2. Create Packet from root_record and answers
    let mut packet = Packet::new_reply(0);
    packet
        .answers
        .push(root_record.to_txt_record(&document.id)?);
    packet.answers.append(&mut answers);

    Ok(packet)
}
