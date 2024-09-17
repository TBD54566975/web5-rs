use super::{call_sign, CSigner};
use crate::c::free_bytes;

#[no_mangle]
pub extern "C" fn poc_signer_from_foreign(signer: *const CSigner) {
    if signer.is_null() {
        return;
    }

    let signer = unsafe { &*signer };
    let payload = b"Test message";

    let mut out_len: usize = 0;
    let signature = call_sign(signer, payload.as_ptr(), payload.len(), &mut out_len);

    free_bytes(signature);
}
