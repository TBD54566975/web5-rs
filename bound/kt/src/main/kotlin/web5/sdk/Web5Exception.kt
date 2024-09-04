package web5.sdk

class Web5Exception(
    val variant: String,
    override val message: String
) : Exception(message) {
    companion object {
        internal fun fromRustCore(e: web5.sdk.rust.Web5Exception.Exception): Web5Exception {
            return Web5Exception(e.variant, e.msg)
        }
    }
}