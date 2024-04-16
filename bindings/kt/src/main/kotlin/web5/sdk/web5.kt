// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

@file:Suppress("NAME_SHADOWING")

package web5.sdk;

// Common helper code.
//
// Ideally this would live in a separate .kt file where it can be unittested etc
// in isolation, and perhaps even published as a re-useable package.
//
// However, it's important that the details of how this helper code works (e.g. the
// way that different builtin types are passed across the FFI) exactly match what's
// expected by the Rust code on the other side of the interface. In practice right
// now that means coming from the exact some version of `uniffi` that was used to
// compile the Rust component. The easiest way to ensure this is to bundle the Kotlin
// helpers directly inline like we're doing here.

import com.sun.jna.Library
import com.sun.jna.IntegerType
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.Callback
import com.sun.jna.ptr.*
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.CharBuffer
import java.nio.charset.CodingErrorAction
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong

// This is a helper for safely working with byte buffers returned from the Rust code.
// A rust-owned buffer is represented by its capacity, its current length, and a
// pointer to the underlying data.

@Structure.FieldOrder("capacity", "len", "data")
open class RustBuffer : Structure() {
    @JvmField var capacity: Int = 0
    @JvmField var len: Int = 0
    @JvmField var data: Pointer? = null

    class ByValue: RustBuffer(), Structure.ByValue
    class ByReference: RustBuffer(), Structure.ByReference

    companion object {
        internal fun alloc(size: Int = 0) = rustCall() { status ->
            _UniFFILib.INSTANCE.ffi_web5_rustbuffer_alloc(size, status)
        }.also {
            if(it.data == null) {
               throw RuntimeException("RustBuffer.alloc() returned null data pointer (size=${size})")
           }
        }

        internal fun create(capacity: Int, len: Int, data: Pointer?): RustBuffer.ByValue {
            var buf = RustBuffer.ByValue()
            buf.capacity = capacity
            buf.len = len
            buf.data = data
            return buf
        }

        internal fun free(buf: RustBuffer.ByValue) = rustCall() { status ->
            _UniFFILib.INSTANCE.ffi_web5_rustbuffer_free(buf, status)
        }
    }

    @Suppress("TooGenericExceptionThrown")
    fun asByteBuffer() =
        this.data?.getByteBuffer(0, this.len.toLong())?.also {
            it.order(ByteOrder.BIG_ENDIAN)
        }
}

/**
 * The equivalent of the `*mut RustBuffer` type.
 * Required for callbacks taking in an out pointer.
 *
 * Size is the sum of all values in the struct.
 */
class RustBufferByReference : ByReference(16) {
    /**
     * Set the pointed-to `RustBuffer` to the given value.
     */
    fun setValue(value: RustBuffer.ByValue) {
        // NOTE: The offsets are as they are in the C-like struct.
        val pointer = getPointer()
        pointer.setInt(0, value.capacity)
        pointer.setInt(4, value.len)
        pointer.setPointer(8, value.data)
    }

    /**
     * Get a `RustBuffer.ByValue` from this reference.
     */
    fun getValue(): RustBuffer.ByValue {
        val pointer = getPointer()
        val value = RustBuffer.ByValue()
        value.writeField("capacity", pointer.getInt(0))
        value.writeField("len", pointer.getInt(4))
        value.writeField("data", pointer.getPointer(8))

        return value
    }
}

// This is a helper for safely passing byte references into the rust code.
// It's not actually used at the moment, because there aren't many things that you
// can take a direct pointer to in the JVM, and if we're going to copy something
// then we might as well copy it into a `RustBuffer`. But it's here for API
// completeness.

@Structure.FieldOrder("len", "data")
open class ForeignBytes : Structure() {
    @JvmField var len: Int = 0
    @JvmField var data: Pointer? = null

    class ByValue : ForeignBytes(), Structure.ByValue
}
// The FfiConverter interface handles converter types to and from the FFI
//
// All implementing objects should be public to support external types.  When a
// type is external we need to import it's FfiConverter.
public interface FfiConverter<KotlinType, FfiType> {
    // Convert an FFI type to a Kotlin type
    fun lift(value: FfiType): KotlinType

    // Convert an Kotlin type to an FFI type
    fun lower(value: KotlinType): FfiType

    // Read a Kotlin type from a `ByteBuffer`
    fun read(buf: ByteBuffer): KotlinType

    // Calculate bytes to allocate when creating a `RustBuffer`
    //
    // This must return at least as many bytes as the write() function will
    // write. It can return more bytes than needed, for example when writing
    // Strings we can't know the exact bytes needed until we the UTF-8
    // encoding, so we pessimistically allocate the largest size possible (3
    // bytes per codepoint).  Allocating extra bytes is not really a big deal
    // because the `RustBuffer` is short-lived.
    fun allocationSize(value: KotlinType): Int

    // Write a Kotlin type to a `ByteBuffer`
    fun write(value: KotlinType, buf: ByteBuffer)

    // Lower a value into a `RustBuffer`
    //
    // This method lowers a value into a `RustBuffer` rather than the normal
    // FfiType.  It's used by the callback interface code.  Callback interface
    // returns are always serialized into a `RustBuffer` regardless of their
    // normal FFI type.
    fun lowerIntoRustBuffer(value: KotlinType): RustBuffer.ByValue {
        val rbuf = RustBuffer.alloc(allocationSize(value))
        try {
            val bbuf = rbuf.data!!.getByteBuffer(0, rbuf.capacity.toLong()).also {
                it.order(ByteOrder.BIG_ENDIAN)
            }
            write(value, bbuf)
            rbuf.writeField("len", bbuf.position())
            return rbuf
        } catch (e: Throwable) {
            RustBuffer.free(rbuf)
            throw e
        }
    }

    // Lift a value from a `RustBuffer`.
    //
    // This here mostly because of the symmetry with `lowerIntoRustBuffer()`.
    // It's currently only used by the `FfiConverterRustBuffer` class below.
    fun liftFromRustBuffer(rbuf: RustBuffer.ByValue): KotlinType {
        val byteBuf = rbuf.asByteBuffer()!!
        try {
           val item = read(byteBuf)
           if (byteBuf.hasRemaining()) {
               throw RuntimeException("junk remaining in buffer after lifting, something is very wrong!!")
           }
           return item
        } finally {
            RustBuffer.free(rbuf)
        }
    }
}

// FfiConverter that uses `RustBuffer` as the FfiType
public interface FfiConverterRustBuffer<KotlinType>: FfiConverter<KotlinType, RustBuffer.ByValue> {
    override fun lift(value: RustBuffer.ByValue) = liftFromRustBuffer(value)
    override fun lower(value: KotlinType) = lowerIntoRustBuffer(value)
}
// A handful of classes and functions to support the generated data structures.
// This would be a good candidate for isolating in its own ffi-support lib.
// Error runtime.
@Structure.FieldOrder("code", "error_buf")
internal open class RustCallStatus : Structure() {
    @JvmField var code: Byte = 0
    @JvmField var error_buf: RustBuffer.ByValue = RustBuffer.ByValue()

    class ByValue: RustCallStatus(), Structure.ByValue

    fun isSuccess(): Boolean {
        return code == 0.toByte()
    }

    fun isError(): Boolean {
        return code == 1.toByte()
    }

    fun isPanic(): Boolean {
        return code == 2.toByte()
    }
}

class InternalException(message: String) : Exception(message)

// Each top-level error class has a companion object that can lift the error from the call status's rust buffer
interface CallStatusErrorHandler<E> {
    fun lift(error_buf: RustBuffer.ByValue): E;
}

// Helpers for calling Rust
// In practice we usually need to be synchronized to call this safely, so it doesn't
// synchronize itself

// Call a rust function that returns a Result<>.  Pass in the Error class companion that corresponds to the Err
private inline fun <U, E: Exception> rustCallWithError(errorHandler: CallStatusErrorHandler<E>, callback: (RustCallStatus) -> U): U {
    var status = RustCallStatus();
    val return_value = callback(status)
    checkCallStatus(errorHandler, status)
    return return_value
}

// Check RustCallStatus and throw an error if the call wasn't successful
private fun<E: Exception> checkCallStatus(errorHandler: CallStatusErrorHandler<E>, status: RustCallStatus) {
    if (status.isSuccess()) {
        return
    } else if (status.isError()) {
        throw errorHandler.lift(status.error_buf)
    } else if (status.isPanic()) {
        // when the rust code sees a panic, it tries to construct a rustbuffer
        // with the message.  but if that code panics, then it just sends back
        // an empty buffer.
        if (status.error_buf.len > 0) {
            throw InternalException(FfiConverterString.lift(status.error_buf))
        } else {
            throw InternalException("Rust panic")
        }
    } else {
        throw InternalException("Unknown rust call status: $status.code")
    }
}

// CallStatusErrorHandler implementation for times when we don't expect a CALL_ERROR
object NullCallStatusErrorHandler: CallStatusErrorHandler<InternalException> {
    override fun lift(error_buf: RustBuffer.ByValue): InternalException {
        RustBuffer.free(error_buf)
        return InternalException("Unexpected CALL_ERROR")
    }
}

// Call a rust function that returns a plain value
private inline fun <U> rustCall(callback: (RustCallStatus) -> U): U {
    return rustCallWithError(NullCallStatusErrorHandler, callback);
}

// IntegerType that matches Rust's `usize` / C's `size_t`
public class USize(value: Long = 0) : IntegerType(Native.SIZE_T_SIZE, value, true) {
    // This is needed to fill in the gaps of IntegerType's implementation of Number for Kotlin.
    override fun toByte() = toInt().toByte()
    // Needed until https://youtrack.jetbrains.com/issue/KT-47902 is fixed.
    @Deprecated("`toInt().toChar()` is deprecated")
    override fun toChar() = toInt().toChar()
    override fun toShort() = toInt().toShort()

    fun writeToBuffer(buf: ByteBuffer) {
        // Make sure we always write usize integers using native byte-order, since they may be
        // casted to pointer values
        buf.order(ByteOrder.nativeOrder())
        try {
            when (Native.SIZE_T_SIZE) {
                4 -> buf.putInt(toInt())
                8 -> buf.putLong(toLong())
                else -> throw RuntimeException("Invalid SIZE_T_SIZE: ${Native.SIZE_T_SIZE}")
            }
        } finally {
            buf.order(ByteOrder.BIG_ENDIAN)
        }
    }

    companion object {
        val size: Int
            get() = Native.SIZE_T_SIZE

        fun readFromBuffer(buf: ByteBuffer) : USize {
            // Make sure we always read usize integers using native byte-order, since they may be
            // casted from pointer values
            buf.order(ByteOrder.nativeOrder())
            try {
                return when (Native.SIZE_T_SIZE) {
                    4 -> USize(buf.getInt().toLong())
                    8 -> USize(buf.getLong())
                    else -> throw RuntimeException("Invalid SIZE_T_SIZE: ${Native.SIZE_T_SIZE}")
                }
            } finally {
                buf.order(ByteOrder.BIG_ENDIAN)
            }
        }
    }
}


// Map handles to objects
//
// This is used when the Rust code expects an opaque pointer to represent some foreign object.
// Normally we would pass a pointer to the object, but JNA doesn't support getting a pointer from an
// object reference , nor does it support leaking a reference to Rust.
//
// Instead, this class maps USize values to objects so that we can pass a pointer-sized type to
// Rust when it needs an opaque pointer.
//
// TODO: refactor callbacks to use this class
internal class UniFfiHandleMap<T: Any> {
    private val map = ConcurrentHashMap<USize, T>()
    // Use AtomicInteger for our counter, since we may be on a 32-bit system.  4 billion possible
    // values seems like enough. If somehow we generate 4 billion handles, then this will wrap
    // around back to zero and we can assume the first handle generated will have been dropped by
    // then.
    private val counter = java.util.concurrent.atomic.AtomicInteger(0)

    val size: Int
        get() = map.size

    fun insert(obj: T): USize {
        val handle = USize(counter.getAndAdd(1).toLong())
        map.put(handle, obj)
        return handle
    }

    fun get(handle: USize): T? {
        return map.get(handle)
    }

    fun remove(handle: USize): T? {
        return map.remove(handle)
    }
}

// FFI type for Rust future continuations
internal interface UniFffiRustFutureContinuationCallbackType : com.sun.jna.Callback {
    fun callback(continuationHandle: USize, pollResult: Short);
}

// Contains loading, initialization code,
// and the FFI Function declarations in a com.sun.jna.Library.
@Synchronized
private fun findLibraryName(componentName: String): String {
    val libOverride = System.getProperty("uniffi.component.$componentName.libraryOverride")
    if (libOverride != null) {
        return libOverride
    }
    return "web5"
}

private inline fun <reified Lib : Library> loadIndirect(
    componentName: String
): Lib {
    return Native.load<Lib>(findLibraryName(componentName), Lib::class.java)
}

// A JNA Library to expose the extern-C FFI definitions.
// This is an implementation detail which will be called internally by the public API.

internal interface _UniFFILib : Library {
    companion object {
        internal val INSTANCE: _UniFFILib by lazy {
            loadIndirect<_UniFFILib>(componentName = "web5")
            .also { lib: _UniFFILib ->
                uniffiCheckContractApiVersion(lib)
                uniffiCheckApiChecksums(lib)
                }
        }
    }

    fun uniffi_web5_fn_free_jwk(`ptr`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Unit
    fun uniffi_web5_fn_constructor_jwk_new(`alg`: RustBuffer.ByValue,`kty`: RustBuffer.ByValue,`crv`: RustBuffer.ByValue,`d`: RustBuffer.ByValue,`x`: RustBuffer.ByValue,`y`: RustBuffer.ByValue,_uniffi_out_err: RustCallStatus, 
    ): Pointer
    fun uniffi_web5_fn_method_jwk_compute_thumbprint(`ptr`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): RustBuffer.ByValue
    fun ffi_web5_rustbuffer_alloc(`size`: Int,_uniffi_out_err: RustCallStatus, 
    ): RustBuffer.ByValue
    fun ffi_web5_rustbuffer_from_bytes(`bytes`: ForeignBytes.ByValue,_uniffi_out_err: RustCallStatus, 
    ): RustBuffer.ByValue
    fun ffi_web5_rustbuffer_free(`buf`: RustBuffer.ByValue,_uniffi_out_err: RustCallStatus, 
    ): Unit
    fun ffi_web5_rustbuffer_reserve(`buf`: RustBuffer.ByValue,`additional`: Int,_uniffi_out_err: RustCallStatus, 
    ): RustBuffer.ByValue
    fun ffi_web5_rust_future_poll_u8(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_u8(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_u8(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_u8(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Byte
    fun ffi_web5_rust_future_poll_i8(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_i8(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_i8(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_i8(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Byte
    fun ffi_web5_rust_future_poll_u16(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_u16(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_u16(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_u16(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Short
    fun ffi_web5_rust_future_poll_i16(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_i16(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_i16(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_i16(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Short
    fun ffi_web5_rust_future_poll_u32(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_u32(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_u32(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_u32(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Int
    fun ffi_web5_rust_future_poll_i32(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_i32(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_i32(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_i32(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Int
    fun ffi_web5_rust_future_poll_u64(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_u64(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_u64(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_u64(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Long
    fun ffi_web5_rust_future_poll_i64(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_i64(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_i64(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_i64(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Long
    fun ffi_web5_rust_future_poll_f32(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_f32(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_f32(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_f32(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Float
    fun ffi_web5_rust_future_poll_f64(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_f64(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_f64(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_f64(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Double
    fun ffi_web5_rust_future_poll_pointer(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_pointer(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_pointer(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_pointer(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Pointer
    fun ffi_web5_rust_future_poll_rust_buffer(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_rust_buffer(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_rust_buffer(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_rust_buffer(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): RustBuffer.ByValue
    fun ffi_web5_rust_future_poll_void(`handle`: Pointer,`callback`: UniFffiRustFutureContinuationCallbackType,`callbackData`: USize,
    ): Unit
    fun ffi_web5_rust_future_cancel_void(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_free_void(`handle`: Pointer,
    ): Unit
    fun ffi_web5_rust_future_complete_void(`handle`: Pointer,_uniffi_out_err: RustCallStatus, 
    ): Unit
    fun uniffi_web5_checksum_method_jwk_compute_thumbprint(
    ): Short
    fun uniffi_web5_checksum_constructor_jwk_new(
    ): Short
    fun ffi_web5_uniffi_contract_version(
    ): Int
    
}

private fun uniffiCheckContractApiVersion(lib: _UniFFILib) {
    // Get the bindings contract version from our ComponentInterface
    val bindings_contract_version = 25
    // Get the scaffolding contract version by calling the into the dylib
    val scaffolding_contract_version = lib.ffi_web5_uniffi_contract_version()
    if (bindings_contract_version != scaffolding_contract_version) {
        throw RuntimeException("UniFFI contract version mismatch: try cleaning and rebuilding your project")
    }
}

@Suppress("UNUSED_PARAMETER")
private fun uniffiCheckApiChecksums(lib: _UniFFILib) {
    if (lib.uniffi_web5_checksum_method_jwk_compute_thumbprint() != 9735.toShort()) {
        throw RuntimeException("UniFFI API checksum mismatch: try cleaning and rebuilding your project")
    }
    if (lib.uniffi_web5_checksum_constructor_jwk_new() != 31971.toShort()) {
        throw RuntimeException("UniFFI API checksum mismatch: try cleaning and rebuilding your project")
    }
}

// Async support

// Public interface members begin here.


public object FfiConverterString: FfiConverter<String, RustBuffer.ByValue> {
    // Note: we don't inherit from FfiConverterRustBuffer, because we use a
    // special encoding when lowering/lifting.  We can use `RustBuffer.len` to
    // store our length and avoid writing it out to the buffer.
    override fun lift(value: RustBuffer.ByValue): String {
        try {
            val byteArr = ByteArray(value.len)
            value.asByteBuffer()!!.get(byteArr)
            return byteArr.toString(Charsets.UTF_8)
        } finally {
            RustBuffer.free(value)
        }
    }

    override fun read(buf: ByteBuffer): String {
        val len = buf.getInt()
        val byteArr = ByteArray(len)
        buf.get(byteArr)
        return byteArr.toString(Charsets.UTF_8)
    }

    fun toUtf8(value: String): ByteBuffer {
        // Make sure we don't have invalid UTF-16, check for lone surrogates.
        return Charsets.UTF_8.newEncoder().run {
            onMalformedInput(CodingErrorAction.REPORT)
            encode(CharBuffer.wrap(value))
        }
    }

    override fun lower(value: String): RustBuffer.ByValue {
        val byteBuf = toUtf8(value)
        // Ideally we'd pass these bytes to `ffi_bytebuffer_from_bytes`, but doing so would require us
        // to copy them into a JNA `Memory`. So we might as well directly copy them into a `RustBuffer`.
        val rbuf = RustBuffer.alloc(byteBuf.limit())
        rbuf.asByteBuffer()!!.put(byteBuf)
        return rbuf
    }

    // We aren't sure exactly how many bytes our string will be once it's UTF-8
    // encoded.  Allocate 3 bytes per UTF-16 code unit which will always be
    // enough.
    override fun allocationSize(value: String): Int {
        val sizeForLength = 4
        val sizeForString = value.length * 3
        return sizeForLength + sizeForString
    }

    override fun write(value: String, buf: ByteBuffer) {
        val byteBuf = toUtf8(value)
        buf.putInt(byteBuf.limit())
        buf.put(byteBuf)
    }
}



// Interface implemented by anything that can contain an object reference.
//
// Such types expose a `destroy()` method that must be called to cleanly
// dispose of the contained objects. Failure to call this method may result
// in memory leaks.
//
// The easiest way to ensure this method is called is to use the `.use`
// helper method to execute a block and destroy the object at the end.
interface Disposable {
    fun destroy()
    companion object {
        fun destroy(vararg args: Any?) {
            args.filterIsInstance<Disposable>()
                .forEach(Disposable::destroy)
        }
    }
}

inline fun <T : Disposable?, R> T.use(block: (T) -> R) =
    try {
        block(this)
    } finally {
        try {
            // N.B. our implementation is on the nullable type `Disposable?`.
            this?.destroy()
        } catch (e: Throwable) {
            // swallow
        }
    }

// The base class for all UniFFI Object types.
//
// This class provides core operations for working with the Rust `Arc<T>` pointer to
// the live Rust struct on the other side of the FFI.
//
// There's some subtlety here, because we have to be careful not to operate on a Rust
// struct after it has been dropped, and because we must expose a public API for freeing
// the Kotlin wrapper object in lieu of reliable finalizers. The core requirements are:
//
//   * Each `FFIObject` instance holds an opaque pointer to the underlying Rust struct.
//     Method calls need to read this pointer from the object's state and pass it in to
//     the Rust FFI.
//
//   * When an `FFIObject` is no longer needed, its pointer should be passed to a
//     special destructor function provided by the Rust FFI, which will drop the
//     underlying Rust struct.
//
//   * Given an `FFIObject` instance, calling code is expected to call the special
//     `destroy` method in order to free it after use, either by calling it explicitly
//     or by using a higher-level helper like the `use` method. Failing to do so will
//     leak the underlying Rust struct.
//
//   * We can't assume that calling code will do the right thing, and must be prepared
//     to handle Kotlin method calls executing concurrently with or even after a call to
//     `destroy`, and to handle multiple (possibly concurrent!) calls to `destroy`.
//
//   * We must never allow Rust code to operate on the underlying Rust struct after
//     the destructor has been called, and must never call the destructor more than once.
//     Doing so may trigger memory unsafety.
//
// If we try to implement this with mutual exclusion on access to the pointer, there is the
// possibility of a race between a method call and a concurrent call to `destroy`:
//
//    * Thread A starts a method call, reads the value of the pointer, but is interrupted
//      before it can pass the pointer over the FFI to Rust.
//    * Thread B calls `destroy` and frees the underlying Rust struct.
//    * Thread A resumes, passing the already-read pointer value to Rust and triggering
//      a use-after-free.
//
// One possible solution would be to use a `ReadWriteLock`, with each method call taking
// a read lock (and thus allowed to run concurrently) and the special `destroy` method
// taking a write lock (and thus blocking on live method calls). However, we aim not to
// generate methods with any hidden blocking semantics, and a `destroy` method that might
// block if called incorrectly seems to meet that bar.
//
// So, we achieve our goals by giving each `FFIObject` an associated `AtomicLong` counter to track
// the number of in-flight method calls, and an `AtomicBoolean` flag to indicate whether `destroy`
// has been called. These are updated according to the following rules:
//
//    * The initial value of the counter is 1, indicating a live object with no in-flight calls.
//      The initial value for the flag is false.
//
//    * At the start of each method call, we atomically check the counter.
//      If it is 0 then the underlying Rust struct has already been destroyed and the call is aborted.
//      If it is nonzero them we atomically increment it by 1 and proceed with the method call.
//
//    * At the end of each method call, we atomically decrement and check the counter.
//      If it has reached zero then we destroy the underlying Rust struct.
//
//    * When `destroy` is called, we atomically flip the flag from false to true.
//      If the flag was already true we silently fail.
//      Otherwise we atomically decrement and check the counter.
//      If it has reached zero then we destroy the underlying Rust struct.
//
// Astute readers may observe that this all sounds very similar to the way that Rust's `Arc<T>` works,
// and indeed it is, with the addition of a flag to guard against multiple calls to `destroy`.
//
// The overall effect is that the underlying Rust struct is destroyed only when `destroy` has been
// called *and* all in-flight method calls have completed, avoiding violating any of the expectations
// of the underlying Rust code.
//
// In the future we may be able to replace some of this with automatic finalization logic, such as using
// the new "Cleaner" functionaility in Java 9. The above scheme has been designed to work even if `destroy` is
// invoked by garbage-collection machinery rather than by calling code (which by the way, it's apparently also
// possible for the JVM to finalize an object while there is an in-flight call to one of its methods [1],
// so there would still be some complexity here).
//
// Sigh...all of this for want of a robust finalization mechanism.
//
// [1] https://stackoverflow.com/questions/24376768/can-java-finalize-an-object-when-it-is-still-in-scope/24380219
//
abstract class FFIObject: Disposable, AutoCloseable {

    constructor(pointer: Pointer) {
        this.pointer = pointer
    }

    /**
     * This constructor can be used to instantiate a fake object.
     *
     * **WARNING: Any object instantiated with this constructor cannot be passed to an actual Rust-backed object.**
     * Since there isn't a backing [Pointer] the FFI lower functions will crash.
     * @param noPointer Placeholder value so we can have a constructor separate from the default empty one that may be
     *   implemented for classes extending [FFIObject].
     */
    @Suppress("UNUSED_PARAMETER")
    constructor(noPointer: NoPointer) {
        this.pointer = null
    }

    protected val pointer: Pointer?

    private val wasDestroyed = AtomicBoolean(false)
    private val callCounter = AtomicLong(1)

    open protected fun freeRustArcPtr() {
        // To be overridden in subclasses.
    }

    override fun destroy() {
        // Only allow a single call to this method.
        // TODO: maybe we should log a warning if called more than once?
        if (this.wasDestroyed.compareAndSet(false, true)) {
            // This decrement always matches the initial count of 1 given at creation time.
            if (this.callCounter.decrementAndGet() == 0L) {
                this.freeRustArcPtr()
            }
        }
    }

    @Synchronized
    override fun close() {
        this.destroy()
    }

    internal inline fun <R> callWithPointer(block: (ptr: Pointer) -> R): R {
        // Check and increment the call counter, to keep the object alive.
        // This needs a compare-and-set retry loop in case of concurrent updates.
        do {
            val c = this.callCounter.get()
            if (c == 0L) {
                throw IllegalStateException("${this.javaClass.simpleName} object has already been destroyed")
            }
            if (c == Long.MAX_VALUE) {
                throw IllegalStateException("${this.javaClass.simpleName} call counter would overflow")
            }
        } while (! this.callCounter.compareAndSet(c, c + 1L))
        // Now we can safely do the method call without the pointer being freed concurrently.
        try {
            return block(this.pointer!!)
        } finally {
            // This decrement always matches the increment we performed above.
            if (this.callCounter.decrementAndGet() == 0L) {
                this.freeRustArcPtr()
            }
        }
    }
}

/** Used to instantiate a [FFIObject] without an actual pointer, for fakes in tests, mostly. */
object NoPointer

public interface JwkInterface {
    fun `computeThumbprint`(): String
    
    companion object
}


open class Jwk : FFIObject, JwkInterface {

    constructor(pointer: Pointer): super(pointer)

    /**
     * This constructor can be used to instantiate a fake object.
     *
     * **WARNING: Any object instantiated with this constructor cannot be passed to an actual Rust-backed object.**
     * Since there isn't a backing [Pointer] the FFI lower functions will crash.
     * @param noPointer Placeholder value so we can have a constructor separate from the default empty one that may be
     *   implemented for classes extending [FFIObject].
     */
    constructor(noPointer: NoPointer): super(noPointer)
    constructor(`alg`: String, `kty`: String, `crv`: String, `d`: String?, `x`: String, `y`: String?) :
        this(
    rustCall() { _status ->
    _UniFFILib.INSTANCE.uniffi_web5_fn_constructor_jwk_new(FfiConverterString.lower(`alg`),FfiConverterString.lower(`kty`),FfiConverterString.lower(`crv`),FfiConverterOptionalString.lower(`d`),FfiConverterString.lower(`x`),FfiConverterOptionalString.lower(`y`),_status)
})

    /**
     * Disconnect the object from the underlying Rust object.
     *
     * It can be called more than once, but once called, interacting with the object
     * causes an `IllegalStateException`.
     *
     * Clients **must** call this method once done with the object, or cause a memory leak.
     */
    override protected fun freeRustArcPtr() {
        this.pointer?.let { ptr ->
            rustCall() { status ->
                _UniFFILib.INSTANCE.uniffi_web5_fn_free_jwk(ptr, status)
            }
        }
    }

    
    @Throws(JwkException::class)override fun `computeThumbprint`(): String =
        callWithPointer {
    rustCallWithError(JwkException) { _status ->
    _UniFFILib.INSTANCE.uniffi_web5_fn_method_jwk_compute_thumbprint(it,
        
        _status)
}
        }.let {
            FfiConverterString.lift(it)
        }
    
    

    
    companion object
    
}

public object FfiConverterTypeJwk: FfiConverter<Jwk, Pointer> {

    override fun lower(value: Jwk): Pointer {
        return value.callWithPointer { it }
    }

    override fun lift(value: Pointer): Jwk {
        return Jwk(value)
    }

    override fun read(buf: ByteBuffer): Jwk {
        // The Rust code always writes pointers as 8 bytes, and will
        // fail to compile if they don't fit.
        return lift(Pointer(buf.getLong()))
    }

    override fun allocationSize(value: Jwk) = 8

    override fun write(value: Jwk, buf: ByteBuffer) {
        // The Rust code always expects pointers written as 8 bytes,
        // and will fail to compile if they don't fit.
        buf.putLong(Pointer.nativeValue(lower(value)))
    }
}





sealed class CryptoException(message: String): Exception(message) {
        // Each variant is a nested class
        // Flat enums carries a string error message, so no special implementation is necessary.
        class MissingPrivateKey(message: String) : CryptoException(message)
        class DecodeException(message: String) : CryptoException(message)
        class InvalidKeyLength(message: String) : CryptoException(message)
        class InvalidSignatureLength(message: String) : CryptoException(message)
        class PublicKeyFailure(message: String) : CryptoException(message)
        class PrivateKeyFailure(message: String) : CryptoException(message)
        class VerificationFailure(message: String) : CryptoException(message)
        class SignFailure(message: String) : CryptoException(message)
        

    companion object ErrorHandler : CallStatusErrorHandler<CryptoException> {
        override fun lift(error_buf: RustBuffer.ByValue): CryptoException = FfiConverterTypeCryptoError.lift(error_buf)
    }
}

public object FfiConverterTypeCryptoError : FfiConverterRustBuffer<CryptoException> {
    override fun read(buf: ByteBuffer): CryptoException {
        
            return when(buf.getInt()) {
            1 -> CryptoException.MissingPrivateKey(FfiConverterString.read(buf))
            2 -> CryptoException.DecodeException(FfiConverterString.read(buf))
            3 -> CryptoException.InvalidKeyLength(FfiConverterString.read(buf))
            4 -> CryptoException.InvalidSignatureLength(FfiConverterString.read(buf))
            5 -> CryptoException.PublicKeyFailure(FfiConverterString.read(buf))
            6 -> CryptoException.PrivateKeyFailure(FfiConverterString.read(buf))
            7 -> CryptoException.VerificationFailure(FfiConverterString.read(buf))
            8 -> CryptoException.SignFailure(FfiConverterString.read(buf))
            else -> throw RuntimeException("invalid error enum value, something is very wrong!!")
        }
        
    }

    override fun allocationSize(value: CryptoException): Int {
        return 4
    }

    override fun write(value: CryptoException, buf: ByteBuffer) {
        when(value) {
            is CryptoException.MissingPrivateKey -> {
                buf.putInt(1)
                Unit
            }
            is CryptoException.DecodeException -> {
                buf.putInt(2)
                Unit
            }
            is CryptoException.InvalidKeyLength -> {
                buf.putInt(3)
                Unit
            }
            is CryptoException.InvalidSignatureLength -> {
                buf.putInt(4)
                Unit
            }
            is CryptoException.PublicKeyFailure -> {
                buf.putInt(5)
                Unit
            }
            is CryptoException.PrivateKeyFailure -> {
                buf.putInt(6)
                Unit
            }
            is CryptoException.VerificationFailure -> {
                buf.putInt(7)
                Unit
            }
            is CryptoException.SignFailure -> {
                buf.putInt(8)
                Unit
            }
        }.let { /* this makes the `when` an expression, which ensures it is exhaustive */ }
    }

}





sealed class JwkException(message: String): Exception(message) {
        // Each variant is a nested class
        // Flat enums carries a string error message, so no special implementation is necessary.
        class ThumbprintFailed(message: String) : JwkException(message)
        

    companion object ErrorHandler : CallStatusErrorHandler<JwkException> {
        override fun lift(error_buf: RustBuffer.ByValue): JwkException = FfiConverterTypeJwkError.lift(error_buf)
    }
}

public object FfiConverterTypeJwkError : FfiConverterRustBuffer<JwkException> {
    override fun read(buf: ByteBuffer): JwkException {
        
            return when(buf.getInt()) {
            1 -> JwkException.ThumbprintFailed(FfiConverterString.read(buf))
            else -> throw RuntimeException("invalid error enum value, something is very wrong!!")
        }
        
    }

    override fun allocationSize(value: JwkException): Int {
        return 4
    }

    override fun write(value: JwkException, buf: ByteBuffer) {
        when(value) {
            is JwkException.ThumbprintFailed -> {
                buf.putInt(1)
                Unit
            }
        }.let { /* this makes the `when` an expression, which ensures it is exhaustive */ }
    }

}




public object FfiConverterOptionalString: FfiConverterRustBuffer<String?> {
    override fun read(buf: ByteBuffer): String? {
        if (buf.get().toInt() == 0) {
            return null
        }
        return FfiConverterString.read(buf)
    }

    override fun allocationSize(value: String?): Int {
        if (value == null) {
            return 1
        } else {
            return 1 + FfiConverterString.allocationSize(value)
        }
    }

    override fun write(value: String?, buf: ByteBuffer) {
        if (value == null) {
            buf.put(0)
        } else {
            buf.put(1)
            FfiConverterString.write(value, buf)
        }
    }
}

