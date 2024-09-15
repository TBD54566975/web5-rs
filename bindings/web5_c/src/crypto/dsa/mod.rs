pub mod ed25519;

#[repr(C)]
pub struct CSigner {
    pub sign: extern "C" fn(payload: *const u8, payload_len: usize) -> *mut u8,
}

// todo temporary
#[no_mangle]
pub extern "C" fn proof_of_concept(signer: *const CSigner) {
    if signer.is_null() {
        return;
    }

    let signer = unsafe { &*signer };
    let payload = b"Test message";

    (signer.sign)(payload.as_ptr(), payload.len());
}
