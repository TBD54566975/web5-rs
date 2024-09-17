use super::CSigner;

#[no_mangle]
pub extern "C" fn poc_signer_from_foreign(signer: *const CSigner) {
    if signer.is_null() {
        return;
    }

    let signer = unsafe { &*signer };
    let payload = b"Test message";

    let mut out_len: usize = 0;
    (signer.sign)(
        signer.signer_id,
        payload.as_ptr(),
        payload.len(),
        &mut out_len,
    );
}
