package web5

// #include <web5.h>
import "C"

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"math"
	"runtime"
	"sync/atomic"
	"time"
	"unsafe"
)

type RustBuffer = C.RustBuffer

type RustBufferI interface {
	AsReader() *bytes.Reader
	Free()
	ToGoBytes() []byte
	Data() unsafe.Pointer
	Len() int
	Capacity() int
}

func RustBufferFromExternal(b RustBufferI) RustBuffer {
	return RustBuffer{
		capacity: C.int(b.Capacity()),
		len:      C.int(b.Len()),
		data:     (*C.uchar)(b.Data()),
	}
}

func (cb RustBuffer) Capacity() int {
	return int(cb.capacity)
}

func (cb RustBuffer) Len() int {
	return int(cb.len)
}

func (cb RustBuffer) Data() unsafe.Pointer {
	return unsafe.Pointer(cb.data)
}

func (cb RustBuffer) AsReader() *bytes.Reader {
	b := unsafe.Slice((*byte)(cb.data), C.int(cb.len))
	return bytes.NewReader(b)
}

func (cb RustBuffer) Free() {
	rustCall(func(status *C.RustCallStatus) bool {
		C.ffi_web5_uniffi_rustbuffer_free(cb, status)
		return false
	})
}

func (cb RustBuffer) ToGoBytes() []byte {
	return C.GoBytes(unsafe.Pointer(cb.data), C.int(cb.len))
}

func stringToRustBuffer(str string) RustBuffer {
	return bytesToRustBuffer([]byte(str))
}

func bytesToRustBuffer(b []byte) RustBuffer {
	if len(b) == 0 {
		return RustBuffer{}
	}
	// We can pass the pointer along here, as it is pinned
	// for the duration of this call
	foreign := C.ForeignBytes{
		len:  C.int(len(b)),
		data: (*C.uchar)(unsafe.Pointer(&b[0])),
	}

	return rustCall(func(status *C.RustCallStatus) RustBuffer {
		return C.ffi_web5_uniffi_rustbuffer_from_bytes(foreign, status)
	})
}

type BufLifter[GoType any] interface {
	Lift(value RustBufferI) GoType
}

type BufLowerer[GoType any] interface {
	Lower(value GoType) RustBuffer
}

type FfiConverter[GoType any, FfiType any] interface {
	Lift(value FfiType) GoType
	Lower(value GoType) FfiType
}

type BufReader[GoType any] interface {
	Read(reader io.Reader) GoType
}

type BufWriter[GoType any] interface {
	Write(writer io.Writer, value GoType)
}

type FfiRustBufConverter[GoType any, FfiType any] interface {
	FfiConverter[GoType, FfiType]
	BufReader[GoType]
}

func LowerIntoRustBuffer[GoType any](bufWriter BufWriter[GoType], value GoType) RustBuffer {
	// This might be not the most efficient way but it does not require knowing allocation size
	// beforehand
	var buffer bytes.Buffer
	bufWriter.Write(&buffer, value)

	bytes, err := io.ReadAll(&buffer)
	if err != nil {
		panic(fmt.Errorf("reading written data: %w", err))
	}
	return bytesToRustBuffer(bytes)
}

func LiftFromRustBuffer[GoType any](bufReader BufReader[GoType], rbuf RustBufferI) GoType {
	defer rbuf.Free()
	reader := rbuf.AsReader()
	item := bufReader.Read(reader)
	if reader.Len() > 0 {
		// TODO: Remove this
		leftover, _ := io.ReadAll(reader)
		panic(fmt.Errorf("Junk remaining in buffer after lifting: %s", string(leftover)))
	}
	return item
}

func rustCallWithError[U any](converter BufLifter[error], callback func(*C.RustCallStatus) U) (U, error) {
	var status C.RustCallStatus
	returnValue := callback(&status)
	err := checkCallStatus(converter, status)

	return returnValue, err
}

func checkCallStatus(converter BufLifter[error], status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		return converter.Lift(status.errorBuf)
	case 2:
		// when the rust code sees a panic, it tries to construct a rustbuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(status.errorBuf)))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func checkCallStatusUnknown(status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		panic(fmt.Errorf("function not returning an error returned an error"))
	case 2:
		// when the rust code sees a panic, it tries to construct a rustbuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(status.errorBuf)))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func rustCall[U any](callback func(*C.RustCallStatus) U) U {
	returnValue, err := rustCallWithError(nil, callback)
	if err != nil {
		panic(err)
	}
	return returnValue
}

func writeInt8(writer io.Writer, value int8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint8(writer io.Writer, value uint8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt16(writer io.Writer, value int16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint16(writer io.Writer, value uint16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt32(writer io.Writer, value int32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint32(writer io.Writer, value uint32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt64(writer io.Writer, value int64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint64(writer io.Writer, value uint64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat32(writer io.Writer, value float32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat64(writer io.Writer, value float64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func readInt8(reader io.Reader) int8 {
	var result int8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint8(reader io.Reader) uint8 {
	var result uint8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt16(reader io.Reader) int16 {
	var result int16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint16(reader io.Reader) uint16 {
	var result uint16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt32(reader io.Reader) int32 {
	var result int32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint32(reader io.Reader) uint32 {
	var result uint32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt64(reader io.Reader) int64 {
	var result int64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint64(reader io.Reader) uint64 {
	var result uint64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat32(reader io.Reader) float32 {
	var result float32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat64(reader io.Reader) float64 {
	var result float64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func init() {

	uniffiCheckChecksums()
}

func uniffiCheckChecksums() {
	// Get the bindings contract version from our ComponentInterface
	bindingsContractVersion := 24
	// Get the scaffolding contract version by calling the into the dylib
	scaffoldingContractVersion := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint32_t {
		return C.ffi_web5_uniffi_uniffi_contract_version(uniffiStatus)
	})
	if bindingsContractVersion != int(scaffoldingContractVersion) {
		// If this happens try cleaning and rebuilding your project
		panic("web5: UniFFI contract version mismatch")
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_func_did_dht_resolve(uniffiStatus)
		})
		if checksum != 58593 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_func_did_dht_resolve: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_func_did_jwk_resolve(uniffiStatus)
		})
		if checksum != 47278 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_func_did_jwk_resolve: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_func_ed25519_generator_generate(uniffiStatus)
		})
		if checksum != 25550 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_func_ed25519_generator_generate: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_bearerdid_get_data(uniffiStatus)
		})
		if checksum != 16186 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_bearerdid_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_bearerdid_get_signer(uniffiStatus)
		})
		if checksum != 18848 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_bearerdid_get_signer: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_did_get_data(uniffiStatus)
		})
		if checksum != 29837 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_did_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_diddht_deactivate(uniffiStatus)
		})
		if checksum != 33164 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_diddht_deactivate: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_diddht_get_data(uniffiStatus)
		})
		if checksum != 65347 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_diddht_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_diddht_publish(uniffiStatus)
		})
		if checksum != 36939 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_diddht_publish: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_didjwk_get_data(uniffiStatus)
		})
		if checksum != 17888 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_didjwk_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_document_find_public_key_jwk(uniffiStatus)
		})
		if checksum != 57320 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_document_find_public_key_jwk: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_document_get_data(uniffiStatus)
		})
		if checksum != 32403 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_document_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_ed25519signer_sign(uniffiStatus)
		})
		if checksum != 53043 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_ed25519signer_sign: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_ed25519verifier_verify(uniffiStatus)
		})
		if checksum != 6864 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_ed25519verifier_verify: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_inmemorykeymanager_get_as_key_manager(uniffiStatus)
		})
		if checksum != 27709 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_inmemorykeymanager_get_as_key_manager: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_inmemorykeymanager_get_signer(uniffiStatus)
		})
		if checksum != 36296 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_inmemorykeymanager_get_signer: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_inmemorykeymanager_import_private_jwk(uniffiStatus)
		})
		if checksum != 24698 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_inmemorykeymanager_import_private_jwk: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_keymanager_get_signer(uniffiStatus)
		})
		if checksum != 17146 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_keymanager_get_signer: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_portabledid_get_data(uniffiStatus)
		})
		if checksum != 24767 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_portabledid_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_presentationdefinition_get_json_serialized_presentation_definition(uniffiStatus)
		})
		if checksum != 20236 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_presentationdefinition_get_json_serialized_presentation_definition: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_presentationdefinition_select_credentials(uniffiStatus)
		})
		if checksum != 27150 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_presentationdefinition_select_credentials: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_resolutionresult_get_data(uniffiStatus)
		})
		if checksum != 14089 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_resolutionresult_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_signer_sign(uniffiStatus)
		})
		if checksum != 43601 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_signer_sign: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_verifiablecredential_get_data(uniffiStatus)
		})
		if checksum != 8758 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_verifiablecredential_get_data: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_method_verifier_verify(uniffiStatus)
		})
		if checksum != 14435 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_method_verifier_verify: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_bearerdid_from_portable_did(uniffiStatus)
		})
		if checksum != 51603 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_bearerdid_from_portable_did: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_bearerdid_new(uniffiStatus)
		})
		if checksum != 24529 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_bearerdid_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_did_new(uniffiStatus)
		})
		if checksum != 46671 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_did_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_diddht_from_identity_key(uniffiStatus)
		})
		if checksum != 23885 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_diddht_from_identity_key: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_diddht_from_uri(uniffiStatus)
		})
		if checksum != 18864 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_diddht_from_uri: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_didjwk_from_public_jwk(uniffiStatus)
		})
		if checksum != 50631 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_didjwk_from_public_jwk: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_didjwk_from_uri(uniffiStatus)
		})
		if checksum != 37114 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_didjwk_from_uri: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_document_new(uniffiStatus)
		})
		if checksum != 40417 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_document_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_ed25519signer_new(uniffiStatus)
		})
		if checksum != 17079 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_ed25519signer_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_ed25519verifier_new(uniffiStatus)
		})
		if checksum != 64370 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_ed25519verifier_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_inmemorykeymanager_new(uniffiStatus)
		})
		if checksum != 11548 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_inmemorykeymanager_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_portabledid_new(uniffiStatus)
		})
		if checksum != 53323 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_portabledid_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_presentationdefinition_new(uniffiStatus)
		})
		if checksum != 8071 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_presentationdefinition_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_resolutionresult_new(uniffiStatus)
		})
		if checksum != 45616 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_resolutionresult_new: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_web5_uniffi_checksum_constructor_verifiablecredential_create(uniffiStatus)
		})
		if checksum != 36616 {
			// If this happens try cleaning and rebuilding your project
			panic("web5: uniffi_web5_uniffi_checksum_constructor_verifiablecredential_create: UniFFI API checksum mismatch")
		}
	}
}

type FfiConverterBool struct{}

var FfiConverterBoolINSTANCE = FfiConverterBool{}

func (FfiConverterBool) Lower(value bool) C.int8_t {
	if value {
		return C.int8_t(1)
	}
	return C.int8_t(0)
}

func (FfiConverterBool) Write(writer io.Writer, value bool) {
	if value {
		writeInt8(writer, 1)
	} else {
		writeInt8(writer, 0)
	}
}

func (FfiConverterBool) Lift(value C.int8_t) bool {
	return value != 0
}

func (FfiConverterBool) Read(reader io.Reader) bool {
	return readInt8(reader) != 0
}

type FfiDestroyerBool struct{}

func (FfiDestroyerBool) Destroy(_ bool) {}

type FfiConverterString struct{}

var FfiConverterStringINSTANCE = FfiConverterString{}

func (FfiConverterString) Lift(rb RustBufferI) string {
	defer rb.Free()
	reader := rb.AsReader()
	b, err := io.ReadAll(reader)
	if err != nil {
		panic(fmt.Errorf("reading reader: %w", err))
	}
	return string(b)
}

func (FfiConverterString) Read(reader io.Reader) string {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading string, expected %d, read %d", length, read_length))
	}
	return string(buffer)
}

func (FfiConverterString) Lower(value string) RustBuffer {
	return stringToRustBuffer(value)
}

func (FfiConverterString) Write(writer io.Writer, value string) {
	if len(value) > math.MaxInt32 {
		panic("String is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := io.WriteString(writer, value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing string, expected %d, written %d", len(value), write_length))
	}
}

type FfiDestroyerString struct{}

func (FfiDestroyerString) Destroy(_ string) {}

type FfiConverterBytes struct{}

var FfiConverterBytesINSTANCE = FfiConverterBytes{}

func (c FfiConverterBytes) Lower(value []byte) RustBuffer {
	return LowerIntoRustBuffer[[]byte](c, value)
}

func (c FfiConverterBytes) Write(writer io.Writer, value []byte) {
	if len(value) > math.MaxInt32 {
		panic("[]byte is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := writer.Write(value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing []byte, expected %d, written %d", len(value), write_length))
	}
}

func (c FfiConverterBytes) Lift(rb RustBufferI) []byte {
	return LiftFromRustBuffer[[]byte](c, rb)
}

func (c FfiConverterBytes) Read(reader io.Reader) []byte {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading []byte, expected %d, read %d", length, read_length))
	}
	return buffer
}

type FfiDestroyerBytes struct{}

func (FfiDestroyerBytes) Destroy(_ []byte) {}

type FfiConverterTimestamp struct{}

var FfiConverterTimestampINSTANCE = FfiConverterTimestamp{}

func (c FfiConverterTimestamp) Lift(rb RustBufferI) time.Time {
	return LiftFromRustBuffer[time.Time](c, rb)
}

func (c FfiConverterTimestamp) Read(reader io.Reader) time.Time {
	sec := readInt64(reader)
	nsec := readUint32(reader)

	var sign int64 = 1
	if sec < 0 {
		sign = -1
	}

	return time.Unix(sec, int64(nsec)*sign)
}

func (c FfiConverterTimestamp) Lower(value time.Time) RustBuffer {
	return LowerIntoRustBuffer[time.Time](c, value)
}

func (c FfiConverterTimestamp) Write(writer io.Writer, value time.Time) {
	sec := value.Unix()
	nsec := uint32(value.Nanosecond())
	if value.Unix() < 0 {
		nsec = 1_000_000_000 - nsec
		sec += 1
	}

	writeInt64(writer, sec)
	writeUint32(writer, nsec)
}

type FfiDestroyerTimestamp struct{}

func (FfiDestroyerTimestamp) Destroy(_ time.Time) {}

// Below is an implementation of synchronization requirements outlined in the link.
// https://github.com/mozilla/uniffi-rs/blob/0dc031132d9493ca812c3af6e7dd60ad2ea95bf0/uniffi_bindgen/src/bindings/kotlin/templates/ObjectRuntime.kt#L31

type FfiObject struct {
	pointer      unsafe.Pointer
	callCounter  atomic.Int64
	freeFunction func(unsafe.Pointer, *C.RustCallStatus)
	destroyed    atomic.Bool
}

func newFfiObject(pointer unsafe.Pointer, freeFunction func(unsafe.Pointer, *C.RustCallStatus)) FfiObject {
	return FfiObject{
		pointer:      pointer,
		freeFunction: freeFunction,
	}
}

func (ffiObject *FfiObject) incrementPointer(debugName string) unsafe.Pointer {
	for {
		counter := ffiObject.callCounter.Load()
		if counter <= -1 {
			panic(fmt.Errorf("%v object has already been destroyed", debugName))
		}
		if counter == math.MaxInt64 {
			panic(fmt.Errorf("%v object call counter would overflow", debugName))
		}
		if ffiObject.callCounter.CompareAndSwap(counter, counter+1) {
			break
		}
	}

	return ffiObject.pointer
}

func (ffiObject *FfiObject) decrementPointer() {
	if ffiObject.callCounter.Add(-1) == -1 {
		ffiObject.freeRustArcPtr()
	}
}

func (ffiObject *FfiObject) destroy() {
	if ffiObject.destroyed.CompareAndSwap(false, true) {
		if ffiObject.callCounter.Add(-1) == -1 {
			ffiObject.freeRustArcPtr()
		}
	}
}

func (ffiObject *FfiObject) freeRustArcPtr() {
	rustCall(func(status *C.RustCallStatus) int32 {
		ffiObject.freeFunction(ffiObject.pointer, status)
		return 0
	})
}

type BearerDid struct {
	ffiObject FfiObject
}

func NewBearerDid(uri string, keyManager *KeyManager) (*BearerDid, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_bearerdid_new(FfiConverterStringINSTANCE.Lower(uri), FfiConverterKeyManagerINSTANCE.Lower(keyManager), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *BearerDid
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBearerDidINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func BearerDidFromPortableDid(portableDid *PortableDid) (*BearerDid, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_bearerdid_from_portable_did(FfiConverterPortableDidINSTANCE.Lower(portableDid), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *BearerDid
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBearerDidINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *BearerDid) GetData() BearerDidData {
	_pointer := _self.ffiObject.incrementPointer("*BearerDid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeBearerDidDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_bearerdid_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (_self *BearerDid) GetSigner(keyId string) (*Signer, error) {
	_pointer := _self.ffiObject.incrementPointer("*BearerDid")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_method_bearerdid_get_signer(
			_pointer, FfiConverterStringINSTANCE.Lower(keyId), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *Signer
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSignerINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *BearerDid) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterBearerDid struct{}

var FfiConverterBearerDidINSTANCE = FfiConverterBearerDid{}

func (c FfiConverterBearerDid) Lift(pointer unsafe.Pointer) *BearerDid {
	result := &BearerDid{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_bearerdid(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*BearerDid).Destroy)
	return result
}

func (c FfiConverterBearerDid) Read(reader io.Reader) *BearerDid {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterBearerDid) Lower(value *BearerDid) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*BearerDid")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterBearerDid) Write(writer io.Writer, value *BearerDid) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerBearerDid struct{}

func (_ FfiDestroyerBearerDid) Destroy(value *BearerDid) {
	value.Destroy()
}

type Did struct {
	ffiObject FfiObject
}

func NewDid(uri string) (*Did, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_did_new(FfiConverterStringINSTANCE.Lower(uri), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *Did
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterDidINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *Did) GetData() DidData {
	_pointer := _self.ffiObject.incrementPointer("*Did")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeDidDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_did_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (object *Did) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterDid struct{}

var FfiConverterDidINSTANCE = FfiConverterDid{}

func (c FfiConverterDid) Lift(pointer unsafe.Pointer) *Did {
	result := &Did{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_did(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*Did).Destroy)
	return result
}

func (c FfiConverterDid) Read(reader io.Reader) *Did {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterDid) Lower(value *Did) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Did")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterDid) Write(writer io.Writer, value *Did) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerDid struct{}

func (_ FfiDestroyerDid) Destroy(value *Did) {
	value.Destroy()
}

type DidDht struct {
	ffiObject FfiObject
}

func DidDhtFromIdentityKey(identityKey JwkData) (*DidDht, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_diddht_from_identity_key(FfiConverterTypeJwkDataINSTANCE.Lower(identityKey), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *DidDht
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterDidDhtINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func DidDhtFromUri(uri string) (*DidDht, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_diddht_from_uri(FfiConverterStringINSTANCE.Lower(uri), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *DidDht
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterDidDhtINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *DidDht) Deactivate(signer *Signer) error {
	_pointer := _self.ffiObject.incrementPointer("*DidDht")
	defer _self.ffiObject.decrementPointer()
	_, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_web5_uniffi_fn_method_diddht_deactivate(
			_pointer, FfiConverterSignerINSTANCE.Lower(signer), _uniffiStatus)
		return false
	})
	return _uniffiErr
}

func (_self *DidDht) GetData() DidDhtData {
	_pointer := _self.ffiObject.incrementPointer("*DidDht")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeDidDhtDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_diddht_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (_self *DidDht) Publish(signer *Signer) error {
	_pointer := _self.ffiObject.incrementPointer("*DidDht")
	defer _self.ffiObject.decrementPointer()
	_, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_web5_uniffi_fn_method_diddht_publish(
			_pointer, FfiConverterSignerINSTANCE.Lower(signer), _uniffiStatus)
		return false
	})
	return _uniffiErr
}

func (object *DidDht) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterDidDht struct{}

var FfiConverterDidDhtINSTANCE = FfiConverterDidDht{}

func (c FfiConverterDidDht) Lift(pointer unsafe.Pointer) *DidDht {
	result := &DidDht{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_diddht(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*DidDht).Destroy)
	return result
}

func (c FfiConverterDidDht) Read(reader io.Reader) *DidDht {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterDidDht) Lower(value *DidDht) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*DidDht")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterDidDht) Write(writer io.Writer, value *DidDht) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerDidDht struct{}

func (_ FfiDestroyerDidDht) Destroy(value *DidDht) {
	value.Destroy()
}

type DidJwk struct {
	ffiObject FfiObject
}

func DidJwkFromPublicJwk(publicJwk JwkData) (*DidJwk, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_didjwk_from_public_jwk(FfiConverterTypeJwkDataINSTANCE.Lower(publicJwk), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *DidJwk
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterDidJwkINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func DidJwkFromUri(uri string) (*DidJwk, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_didjwk_from_uri(FfiConverterStringINSTANCE.Lower(uri), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *DidJwk
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterDidJwkINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *DidJwk) GetData() DidJwkData {
	_pointer := _self.ffiObject.incrementPointer("*DidJwk")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeDidJwkDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_didjwk_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (object *DidJwk) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterDidJwk struct{}

var FfiConverterDidJwkINSTANCE = FfiConverterDidJwk{}

func (c FfiConverterDidJwk) Lift(pointer unsafe.Pointer) *DidJwk {
	result := &DidJwk{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_didjwk(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*DidJwk).Destroy)
	return result
}

func (c FfiConverterDidJwk) Read(reader io.Reader) *DidJwk {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterDidJwk) Lower(value *DidJwk) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*DidJwk")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterDidJwk) Write(writer io.Writer, value *DidJwk) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerDidJwk struct{}

func (_ FfiDestroyerDidJwk) Destroy(value *DidJwk) {
	value.Destroy()
}

type Document struct {
	ffiObject FfiObject
}

func NewDocument(data DocumentData) *Document {
	return FfiConverterDocumentINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_document_new(FfiConverterTypeDocumentDataINSTANCE.Lower(data), _uniffiStatus)
	}))
}

func (_self *Document) FindPublicKeyJwk(keyId string) (JwkData, error) {
	_pointer := _self.ffiObject.incrementPointer("*Document")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_document_find_public_key_jwk(
			_pointer, FfiConverterStringINSTANCE.Lower(keyId), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue JwkData
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterTypeJwkDataINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *Document) GetData() DocumentData {
	_pointer := _self.ffiObject.incrementPointer("*Document")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeDocumentDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_document_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (object *Document) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterDocument struct{}

var FfiConverterDocumentINSTANCE = FfiConverterDocument{}

func (c FfiConverterDocument) Lift(pointer unsafe.Pointer) *Document {
	result := &Document{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_document(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*Document).Destroy)
	return result
}

func (c FfiConverterDocument) Read(reader io.Reader) *Document {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterDocument) Lower(value *Document) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Document")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterDocument) Write(writer io.Writer, value *Document) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerDocument struct{}

func (_ FfiDestroyerDocument) Destroy(value *Document) {
	value.Destroy()
}

type Ed25519Signer struct {
	ffiObject FfiObject
}

func NewEd25519Signer(privateKey JwkData) *Ed25519Signer {
	return FfiConverterEd25519SignerINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_ed25519signer_new(FfiConverterTypeJwkDataINSTANCE.Lower(privateKey), _uniffiStatus)
	}))
}

func (_self *Ed25519Signer) Sign(payload []byte) ([]byte, error) {
	_pointer := _self.ffiObject.incrementPointer("*Ed25519Signer")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_ed25519signer_sign(
			_pointer, FfiConverterBytesINSTANCE.Lower(payload), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []byte
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBytesINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *Ed25519Signer) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterEd25519Signer struct{}

var FfiConverterEd25519SignerINSTANCE = FfiConverterEd25519Signer{}

func (c FfiConverterEd25519Signer) Lift(pointer unsafe.Pointer) *Ed25519Signer {
	result := &Ed25519Signer{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_ed25519signer(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*Ed25519Signer).Destroy)
	return result
}

func (c FfiConverterEd25519Signer) Read(reader io.Reader) *Ed25519Signer {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterEd25519Signer) Lower(value *Ed25519Signer) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Ed25519Signer")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterEd25519Signer) Write(writer io.Writer, value *Ed25519Signer) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerEd25519Signer struct{}

func (_ FfiDestroyerEd25519Signer) Destroy(value *Ed25519Signer) {
	value.Destroy()
}

type Ed25519Verifier struct {
	ffiObject FfiObject
}

func NewEd25519Verifier(publicJwk JwkData) *Ed25519Verifier {
	return FfiConverterEd25519VerifierINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_ed25519verifier_new(FfiConverterTypeJwkDataINSTANCE.Lower(publicJwk), _uniffiStatus)
	}))
}

func (_self *Ed25519Verifier) Verify(message []byte, signature []byte) (bool, error) {
	_pointer := _self.ffiObject.incrementPointer("*Ed25519Verifier")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_web5_uniffi_fn_method_ed25519verifier_verify(
			_pointer, FfiConverterBytesINSTANCE.Lower(message), FfiConverterBytesINSTANCE.Lower(signature), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue bool
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBoolINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *Ed25519Verifier) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterEd25519Verifier struct{}

var FfiConverterEd25519VerifierINSTANCE = FfiConverterEd25519Verifier{}

func (c FfiConverterEd25519Verifier) Lift(pointer unsafe.Pointer) *Ed25519Verifier {
	result := &Ed25519Verifier{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_ed25519verifier(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*Ed25519Verifier).Destroy)
	return result
}

func (c FfiConverterEd25519Verifier) Read(reader io.Reader) *Ed25519Verifier {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterEd25519Verifier) Lower(value *Ed25519Verifier) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Ed25519Verifier")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterEd25519Verifier) Write(writer io.Writer, value *Ed25519Verifier) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerEd25519Verifier struct{}

func (_ FfiDestroyerEd25519Verifier) Destroy(value *Ed25519Verifier) {
	value.Destroy()
}

type InMemoryKeyManager struct {
	ffiObject FfiObject
}

func NewInMemoryKeyManager() *InMemoryKeyManager {
	return FfiConverterInMemoryKeyManagerINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_inmemorykeymanager_new(_uniffiStatus)
	}))
}

func (_self *InMemoryKeyManager) GetAsKeyManager() *KeyManager {
	_pointer := _self.ffiObject.incrementPointer("*InMemoryKeyManager")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterKeyManagerINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_method_inmemorykeymanager_get_as_key_manager(
			_pointer, _uniffiStatus)
	}))
}

func (_self *InMemoryKeyManager) GetSigner(publicJwk JwkData) (*Signer, error) {
	_pointer := _self.ffiObject.incrementPointer("*InMemoryKeyManager")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_method_inmemorykeymanager_get_signer(
			_pointer, FfiConverterTypeJwkDataINSTANCE.Lower(publicJwk), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *Signer
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSignerINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *InMemoryKeyManager) ImportPrivateJwk(privateKey JwkData) (JwkData, error) {
	_pointer := _self.ffiObject.incrementPointer("*InMemoryKeyManager")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_inmemorykeymanager_import_private_jwk(
			_pointer, FfiConverterTypeJwkDataINSTANCE.Lower(privateKey), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue JwkData
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterTypeJwkDataINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *InMemoryKeyManager) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterInMemoryKeyManager struct{}

var FfiConverterInMemoryKeyManagerINSTANCE = FfiConverterInMemoryKeyManager{}

func (c FfiConverterInMemoryKeyManager) Lift(pointer unsafe.Pointer) *InMemoryKeyManager {
	result := &InMemoryKeyManager{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_inmemorykeymanager(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*InMemoryKeyManager).Destroy)
	return result
}

func (c FfiConverterInMemoryKeyManager) Read(reader io.Reader) *InMemoryKeyManager {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterInMemoryKeyManager) Lower(value *InMemoryKeyManager) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*InMemoryKeyManager")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterInMemoryKeyManager) Write(writer io.Writer, value *InMemoryKeyManager) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerInMemoryKeyManager struct{}

func (_ FfiDestroyerInMemoryKeyManager) Destroy(value *InMemoryKeyManager) {
	value.Destroy()
}

type KeyManager struct {
	ffiObject FfiObject
}

func (_self *KeyManager) GetSigner(publicJwk JwkData) (*Signer, error) {
	_pointer := _self.ffiObject.incrementPointer("*KeyManager")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_method_keymanager_get_signer(
			_pointer, FfiConverterTypeJwkDataINSTANCE.Lower(publicJwk), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *Signer
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSignerINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *KeyManager) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterKeyManager struct{}

var FfiConverterKeyManagerINSTANCE = FfiConverterKeyManager{}

func (c FfiConverterKeyManager) Lift(pointer unsafe.Pointer) *KeyManager {
	result := &KeyManager{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_keymanager(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*KeyManager).Destroy)
	return result
}

func (c FfiConverterKeyManager) Read(reader io.Reader) *KeyManager {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterKeyManager) Lower(value *KeyManager) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*KeyManager")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterKeyManager) Write(writer io.Writer, value *KeyManager) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerKeyManager struct{}

func (_ FfiDestroyerKeyManager) Destroy(value *KeyManager) {
	value.Destroy()
}

type PortableDid struct {
	ffiObject FfiObject
}

func NewPortableDid(json string) (*PortableDid, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_portabledid_new(FfiConverterStringINSTANCE.Lower(json), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *PortableDid
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPortableDidINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *PortableDid) GetData() PortableDidData {
	_pointer := _self.ffiObject.incrementPointer("*PortableDid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypePortableDidDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_portabledid_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (object *PortableDid) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterPortableDid struct{}

var FfiConverterPortableDidINSTANCE = FfiConverterPortableDid{}

func (c FfiConverterPortableDid) Lift(pointer unsafe.Pointer) *PortableDid {
	result := &PortableDid{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_portabledid(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*PortableDid).Destroy)
	return result
}

func (c FfiConverterPortableDid) Read(reader io.Reader) *PortableDid {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterPortableDid) Lower(value *PortableDid) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*PortableDid")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterPortableDid) Write(writer io.Writer, value *PortableDid) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerPortableDid struct{}

func (_ FfiDestroyerPortableDid) Destroy(value *PortableDid) {
	value.Destroy()
}

type PresentationDefinition struct {
	ffiObject FfiObject
}

func NewPresentationDefinition(jsonSerializedPresentationDefinition string) (*PresentationDefinition, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_presentationdefinition_new(FfiConverterStringINSTANCE.Lower(jsonSerializedPresentationDefinition), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *PresentationDefinition
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterPresentationDefinitionINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *PresentationDefinition) GetJsonSerializedPresentationDefinition() (string, error) {
	_pointer := _self.ffiObject.incrementPointer("*PresentationDefinition")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_presentationdefinition_get_json_serialized_presentation_definition(
			_pointer, _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterStringINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *PresentationDefinition) SelectCredentials(vcJwts []string) ([]string, error) {
	_pointer := _self.ffiObject.incrementPointer("*PresentationDefinition")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_presentationdefinition_select_credentials(
			_pointer, FfiConverterSequenceStringINSTANCE.Lower(vcJwts), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []string
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceStringINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *PresentationDefinition) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterPresentationDefinition struct{}

var FfiConverterPresentationDefinitionINSTANCE = FfiConverterPresentationDefinition{}

func (c FfiConverterPresentationDefinition) Lift(pointer unsafe.Pointer) *PresentationDefinition {
	result := &PresentationDefinition{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_presentationdefinition(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*PresentationDefinition).Destroy)
	return result
}

func (c FfiConverterPresentationDefinition) Read(reader io.Reader) *PresentationDefinition {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterPresentationDefinition) Lower(value *PresentationDefinition) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*PresentationDefinition")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterPresentationDefinition) Write(writer io.Writer, value *PresentationDefinition) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerPresentationDefinition struct{}

func (_ FfiDestroyerPresentationDefinition) Destroy(value *PresentationDefinition) {
	value.Destroy()
}

type ResolutionResult struct {
	ffiObject FfiObject
}

func NewResolutionResult(uri string) *ResolutionResult {
	return FfiConverterResolutionResultINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_resolutionresult_new(FfiConverterStringINSTANCE.Lower(uri), _uniffiStatus)
	}))
}

func (_self *ResolutionResult) GetData() ResolutionResultData {
	_pointer := _self.ffiObject.incrementPointer("*ResolutionResult")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeResolutionResultDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_resolutionresult_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (object *ResolutionResult) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterResolutionResult struct{}

var FfiConverterResolutionResultINSTANCE = FfiConverterResolutionResult{}

func (c FfiConverterResolutionResult) Lift(pointer unsafe.Pointer) *ResolutionResult {
	result := &ResolutionResult{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_resolutionresult(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*ResolutionResult).Destroy)
	return result
}

func (c FfiConverterResolutionResult) Read(reader io.Reader) *ResolutionResult {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterResolutionResult) Lower(value *ResolutionResult) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*ResolutionResult")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterResolutionResult) Write(writer io.Writer, value *ResolutionResult) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerResolutionResult struct{}

func (_ FfiDestroyerResolutionResult) Destroy(value *ResolutionResult) {
	value.Destroy()
}

type Signer struct {
	ffiObject FfiObject
}

func (_self *Signer) Sign(payload []byte) ([]byte, error) {
	_pointer := _self.ffiObject.incrementPointer("*Signer")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_signer_sign(
			_pointer, FfiConverterBytesINSTANCE.Lower(payload), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []byte
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBytesINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *Signer) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterSigner struct{}

var FfiConverterSignerINSTANCE = FfiConverterSigner{}

func (c FfiConverterSigner) Lift(pointer unsafe.Pointer) *Signer {
	result := &Signer{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_signer(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*Signer).Destroy)
	return result
}

func (c FfiConverterSigner) Read(reader io.Reader) *Signer {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterSigner) Lower(value *Signer) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Signer")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterSigner) Write(writer io.Writer, value *Signer) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerSigner struct{}

func (_ FfiDestroyerSigner) Destroy(value *Signer) {
	value.Destroy()
}

type VerifiableCredential struct {
	ffiObject FfiObject
}

func VerifiableCredentialCreate(jsonSerializedIssuer string, jsonSerializedCredentialSubject string, options *VerifiableCredentialCreateOptionsData) (*VerifiableCredential, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_constructor_verifiablecredential_create(FfiConverterStringINSTANCE.Lower(jsonSerializedIssuer), FfiConverterStringINSTANCE.Lower(jsonSerializedCredentialSubject), FfiConverterOptionalTypeVerifiableCredentialCreateOptionsDataINSTANCE.Lower(options), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *VerifiableCredential
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterVerifiableCredentialINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (_self *VerifiableCredential) GetData() VerifiableCredentialData {
	_pointer := _self.ffiObject.incrementPointer("*VerifiableCredential")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterTypeVerifiableCredentialDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_method_verifiablecredential_get_data(
			_pointer, _uniffiStatus)
	}))
}

func (object *VerifiableCredential) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterVerifiableCredential struct{}

var FfiConverterVerifiableCredentialINSTANCE = FfiConverterVerifiableCredential{}

func (c FfiConverterVerifiableCredential) Lift(pointer unsafe.Pointer) *VerifiableCredential {
	result := &VerifiableCredential{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_verifiablecredential(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*VerifiableCredential).Destroy)
	return result
}

func (c FfiConverterVerifiableCredential) Read(reader io.Reader) *VerifiableCredential {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterVerifiableCredential) Lower(value *VerifiableCredential) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*VerifiableCredential")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterVerifiableCredential) Write(writer io.Writer, value *VerifiableCredential) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerVerifiableCredential struct{}

func (_ FfiDestroyerVerifiableCredential) Destroy(value *VerifiableCredential) {
	value.Destroy()
}

type Verifier struct {
	ffiObject FfiObject
}

func (_self *Verifier) Verify(message []byte, signature []byte) (bool, error) {
	_pointer := _self.ffiObject.incrementPointer("*Verifier")
	defer _self.ffiObject.decrementPointer()
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_web5_uniffi_fn_method_verifier_verify(
			_pointer, FfiConverterBytesINSTANCE.Lower(message), FfiConverterBytesINSTANCE.Lower(signature), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue bool
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterBoolINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func (object *Verifier) Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterVerifier struct{}

var FfiConverterVerifierINSTANCE = FfiConverterVerifier{}

func (c FfiConverterVerifier) Lift(pointer unsafe.Pointer) *Verifier {
	result := &Verifier{
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_web5_uniffi_fn_free_verifier(pointer, status)
			}),
	}
	runtime.SetFinalizer(result, (*Verifier).Destroy)
	return result
}

func (c FfiConverterVerifier) Read(reader io.Reader) *Verifier {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterVerifier) Lower(value *Verifier) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Verifier")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterVerifier) Write(writer io.Writer, value *Verifier) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerVerifier struct{}

func (_ FfiDestroyerVerifier) Destroy(value *Verifier) {
	value.Destroy()
}

type BearerDidData struct {
	Did        DidData
	Document   DocumentData
	KeyManager *KeyManager
}

func (r *BearerDidData) Destroy() {
	FfiDestroyerTypeDidData{}.Destroy(r.Did)
	FfiDestroyerTypeDocumentData{}.Destroy(r.Document)
	FfiDestroyerKeyManager{}.Destroy(r.KeyManager)
}

type FfiConverterTypeBearerDidData struct{}

var FfiConverterTypeBearerDidDataINSTANCE = FfiConverterTypeBearerDidData{}

func (c FfiConverterTypeBearerDidData) Lift(rb RustBufferI) BearerDidData {
	return LiftFromRustBuffer[BearerDidData](c, rb)
}

func (c FfiConverterTypeBearerDidData) Read(reader io.Reader) BearerDidData {
	return BearerDidData{
		FfiConverterTypeDidDataINSTANCE.Read(reader),
		FfiConverterTypeDocumentDataINSTANCE.Read(reader),
		FfiConverterKeyManagerINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeBearerDidData) Lower(value BearerDidData) RustBuffer {
	return LowerIntoRustBuffer[BearerDidData](c, value)
}

func (c FfiConverterTypeBearerDidData) Write(writer io.Writer, value BearerDidData) {
	FfiConverterTypeDidDataINSTANCE.Write(writer, value.Did)
	FfiConverterTypeDocumentDataINSTANCE.Write(writer, value.Document)
	FfiConverterKeyManagerINSTANCE.Write(writer, value.KeyManager)
}

type FfiDestroyerTypeBearerDidData struct{}

func (_ FfiDestroyerTypeBearerDidData) Destroy(value BearerDidData) {
	value.Destroy()
}

type DidData struct {
	Uri      string
	Url      string
	Method   string
	Id       string
	Params   *map[string]string
	Path     *string
	Query    *string
	Fragment *string
}

func (r *DidData) Destroy() {
	FfiDestroyerString{}.Destroy(r.Uri)
	FfiDestroyerString{}.Destroy(r.Url)
	FfiDestroyerString{}.Destroy(r.Method)
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerOptionalMapStringString{}.Destroy(r.Params)
	FfiDestroyerOptionalString{}.Destroy(r.Path)
	FfiDestroyerOptionalString{}.Destroy(r.Query)
	FfiDestroyerOptionalString{}.Destroy(r.Fragment)
}

type FfiConverterTypeDidData struct{}

var FfiConverterTypeDidDataINSTANCE = FfiConverterTypeDidData{}

func (c FfiConverterTypeDidData) Lift(rb RustBufferI) DidData {
	return LiftFromRustBuffer[DidData](c, rb)
}

func (c FfiConverterTypeDidData) Read(reader io.Reader) DidData {
	return DidData{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalMapStringStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeDidData) Lower(value DidData) RustBuffer {
	return LowerIntoRustBuffer[DidData](c, value)
}

func (c FfiConverterTypeDidData) Write(writer io.Writer, value DidData) {
	FfiConverterStringINSTANCE.Write(writer, value.Uri)
	FfiConverterStringINSTANCE.Write(writer, value.Url)
	FfiConverterStringINSTANCE.Write(writer, value.Method)
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterOptionalMapStringStringINSTANCE.Write(writer, value.Params)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Path)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Query)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Fragment)
}

type FfiDestroyerTypeDidData struct{}

func (_ FfiDestroyerTypeDidData) Destroy(value DidData) {
	value.Destroy()
}

type DidDhtData struct {
	Did      DidData
	Document DocumentData
}

func (r *DidDhtData) Destroy() {
	FfiDestroyerTypeDidData{}.Destroy(r.Did)
	FfiDestroyerTypeDocumentData{}.Destroy(r.Document)
}

type FfiConverterTypeDidDhtData struct{}

var FfiConverterTypeDidDhtDataINSTANCE = FfiConverterTypeDidDhtData{}

func (c FfiConverterTypeDidDhtData) Lift(rb RustBufferI) DidDhtData {
	return LiftFromRustBuffer[DidDhtData](c, rb)
}

func (c FfiConverterTypeDidDhtData) Read(reader io.Reader) DidDhtData {
	return DidDhtData{
		FfiConverterTypeDidDataINSTANCE.Read(reader),
		FfiConverterTypeDocumentDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeDidDhtData) Lower(value DidDhtData) RustBuffer {
	return LowerIntoRustBuffer[DidDhtData](c, value)
}

func (c FfiConverterTypeDidDhtData) Write(writer io.Writer, value DidDhtData) {
	FfiConverterTypeDidDataINSTANCE.Write(writer, value.Did)
	FfiConverterTypeDocumentDataINSTANCE.Write(writer, value.Document)
}

type FfiDestroyerTypeDidDhtData struct{}

func (_ FfiDestroyerTypeDidDhtData) Destroy(value DidDhtData) {
	value.Destroy()
}

type DidJwkData struct {
	Did      DidData
	Document DocumentData
}

func (r *DidJwkData) Destroy() {
	FfiDestroyerTypeDidData{}.Destroy(r.Did)
	FfiDestroyerTypeDocumentData{}.Destroy(r.Document)
}

type FfiConverterTypeDidJwkData struct{}

var FfiConverterTypeDidJwkDataINSTANCE = FfiConverterTypeDidJwkData{}

func (c FfiConverterTypeDidJwkData) Lift(rb RustBufferI) DidJwkData {
	return LiftFromRustBuffer[DidJwkData](c, rb)
}

func (c FfiConverterTypeDidJwkData) Read(reader io.Reader) DidJwkData {
	return DidJwkData{
		FfiConverterTypeDidDataINSTANCE.Read(reader),
		FfiConverterTypeDocumentDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeDidJwkData) Lower(value DidJwkData) RustBuffer {
	return LowerIntoRustBuffer[DidJwkData](c, value)
}

func (c FfiConverterTypeDidJwkData) Write(writer io.Writer, value DidJwkData) {
	FfiConverterTypeDidDataINSTANCE.Write(writer, value.Did)
	FfiConverterTypeDocumentDataINSTANCE.Write(writer, value.Document)
}

type FfiDestroyerTypeDidJwkData struct{}

func (_ FfiDestroyerTypeDidJwkData) Destroy(value DidJwkData) {
	value.Destroy()
}

type DidWebData struct {
	Did      DidData
	Document DocumentData
}

func (r *DidWebData) Destroy() {
	FfiDestroyerTypeDidData{}.Destroy(r.Did)
	FfiDestroyerTypeDocumentData{}.Destroy(r.Document)
}

type FfiConverterTypeDidWebData struct{}

var FfiConverterTypeDidWebDataINSTANCE = FfiConverterTypeDidWebData{}

func (c FfiConverterTypeDidWebData) Lift(rb RustBufferI) DidWebData {
	return LiftFromRustBuffer[DidWebData](c, rb)
}

func (c FfiConverterTypeDidWebData) Read(reader io.Reader) DidWebData {
	return DidWebData{
		FfiConverterTypeDidDataINSTANCE.Read(reader),
		FfiConverterTypeDocumentDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeDidWebData) Lower(value DidWebData) RustBuffer {
	return LowerIntoRustBuffer[DidWebData](c, value)
}

func (c FfiConverterTypeDidWebData) Write(writer io.Writer, value DidWebData) {
	FfiConverterTypeDidDataINSTANCE.Write(writer, value.Did)
	FfiConverterTypeDocumentDataINSTANCE.Write(writer, value.Document)
}

type FfiDestroyerTypeDidWebData struct{}

func (_ FfiDestroyerTypeDidWebData) Destroy(value DidWebData) {
	value.Destroy()
}

type DocumentData struct {
	Id                   string
	Context              *[]string
	Controller           *[]string
	AlsoKnownAs          *[]string
	VerificationMethod   []VerificationMethodData
	Authentication       *[]string
	AssertionMethod      *[]string
	KeyAgreement         *[]string
	CapabilityInvocation *[]string
	CapabilityDelegation *[]string
	Service              *[]ServiceData
}

func (r *DocumentData) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.Context)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.Controller)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.AlsoKnownAs)
	FfiDestroyerSequenceTypeVerificationMethodData{}.Destroy(r.VerificationMethod)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.Authentication)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.AssertionMethod)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.KeyAgreement)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.CapabilityInvocation)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.CapabilityDelegation)
	FfiDestroyerOptionalSequenceTypeServiceData{}.Destroy(r.Service)
}

type FfiConverterTypeDocumentData struct{}

var FfiConverterTypeDocumentDataINSTANCE = FfiConverterTypeDocumentData{}

func (c FfiConverterTypeDocumentData) Lift(rb RustBufferI) DocumentData {
	return LiftFromRustBuffer[DocumentData](c, rb)
}

func (c FfiConverterTypeDocumentData) Read(reader io.Reader) DocumentData {
	return DocumentData{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterSequenceTypeVerificationMethodDataINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceTypeServiceDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeDocumentData) Lower(value DocumentData) RustBuffer {
	return LowerIntoRustBuffer[DocumentData](c, value)
}

func (c FfiConverterTypeDocumentData) Write(writer io.Writer, value DocumentData) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.Context)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.Controller)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.AlsoKnownAs)
	FfiConverterSequenceTypeVerificationMethodDataINSTANCE.Write(writer, value.VerificationMethod)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.Authentication)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.AssertionMethod)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.KeyAgreement)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.CapabilityInvocation)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.CapabilityDelegation)
	FfiConverterOptionalSequenceTypeServiceDataINSTANCE.Write(writer, value.Service)
}

type FfiDestroyerTypeDocumentData struct{}

func (_ FfiDestroyerTypeDocumentData) Destroy(value DocumentData) {
	value.Destroy()
}

type DocumentMetadataData struct {
	Created       *string
	Updated       *string
	Deactivated   *bool
	NextUpdate    *string
	VersionId     *string
	NextVersionId *string
	EquivalentId  *[]string
	CanonicalId   *string
}

func (r *DocumentMetadataData) Destroy() {
	FfiDestroyerOptionalString{}.Destroy(r.Created)
	FfiDestroyerOptionalString{}.Destroy(r.Updated)
	FfiDestroyerOptionalBool{}.Destroy(r.Deactivated)
	FfiDestroyerOptionalString{}.Destroy(r.NextUpdate)
	FfiDestroyerOptionalString{}.Destroy(r.VersionId)
	FfiDestroyerOptionalString{}.Destroy(r.NextVersionId)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.EquivalentId)
	FfiDestroyerOptionalString{}.Destroy(r.CanonicalId)
}

type FfiConverterTypeDocumentMetadataData struct{}

var FfiConverterTypeDocumentMetadataDataINSTANCE = FfiConverterTypeDocumentMetadataData{}

func (c FfiConverterTypeDocumentMetadataData) Lift(rb RustBufferI) DocumentMetadataData {
	return LiftFromRustBuffer[DocumentMetadataData](c, rb)
}

func (c FfiConverterTypeDocumentMetadataData) Read(reader io.Reader) DocumentMetadataData {
	return DocumentMetadataData{
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalBoolINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeDocumentMetadataData) Lower(value DocumentMetadataData) RustBuffer {
	return LowerIntoRustBuffer[DocumentMetadataData](c, value)
}

func (c FfiConverterTypeDocumentMetadataData) Write(writer io.Writer, value DocumentMetadataData) {
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Created)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Updated)
	FfiConverterOptionalBoolINSTANCE.Write(writer, value.Deactivated)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextUpdate)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.VersionId)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.NextVersionId)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.EquivalentId)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.CanonicalId)
}

type FfiDestroyerTypeDocumentMetadataData struct{}

func (_ FfiDestroyerTypeDocumentMetadataData) Destroy(value DocumentMetadataData) {
	value.Destroy()
}

type JwkData struct {
	Alg *string
	Kty string
	Crv string
	D   *string
	X   string
	Y   *string
}

func (r *JwkData) Destroy() {
	FfiDestroyerOptionalString{}.Destroy(r.Alg)
	FfiDestroyerString{}.Destroy(r.Kty)
	FfiDestroyerString{}.Destroy(r.Crv)
	FfiDestroyerOptionalString{}.Destroy(r.D)
	FfiDestroyerString{}.Destroy(r.X)
	FfiDestroyerOptionalString{}.Destroy(r.Y)
}

type FfiConverterTypeJwkData struct{}

var FfiConverterTypeJwkDataINSTANCE = FfiConverterTypeJwkData{}

func (c FfiConverterTypeJwkData) Lift(rb RustBufferI) JwkData {
	return LiftFromRustBuffer[JwkData](c, rb)
}

func (c FfiConverterTypeJwkData) Read(reader io.Reader) JwkData {
	return JwkData{
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterOptionalStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeJwkData) Lower(value JwkData) RustBuffer {
	return LowerIntoRustBuffer[JwkData](c, value)
}

func (c FfiConverterTypeJwkData) Write(writer io.Writer, value JwkData) {
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Alg)
	FfiConverterStringINSTANCE.Write(writer, value.Kty)
	FfiConverterStringINSTANCE.Write(writer, value.Crv)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.D)
	FfiConverterStringINSTANCE.Write(writer, value.X)
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Y)
}

type FfiDestroyerTypeJwkData struct{}

func (_ FfiDestroyerTypeJwkData) Destroy(value JwkData) {
	value.Destroy()
}

type PortableDidData struct {
	DidUri      string
	Document    DocumentData
	PrivateJwks []JwkData
}

func (r *PortableDidData) Destroy() {
	FfiDestroyerString{}.Destroy(r.DidUri)
	FfiDestroyerTypeDocumentData{}.Destroy(r.Document)
	FfiDestroyerSequenceTypeJwkData{}.Destroy(r.PrivateJwks)
}

type FfiConverterTypePortableDidData struct{}

var FfiConverterTypePortableDidDataINSTANCE = FfiConverterTypePortableDidData{}

func (c FfiConverterTypePortableDidData) Lift(rb RustBufferI) PortableDidData {
	return LiftFromRustBuffer[PortableDidData](c, rb)
}

func (c FfiConverterTypePortableDidData) Read(reader io.Reader) PortableDidData {
	return PortableDidData{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeDocumentDataINSTANCE.Read(reader),
		FfiConverterSequenceTypeJwkDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypePortableDidData) Lower(value PortableDidData) RustBuffer {
	return LowerIntoRustBuffer[PortableDidData](c, value)
}

func (c FfiConverterTypePortableDidData) Write(writer io.Writer, value PortableDidData) {
	FfiConverterStringINSTANCE.Write(writer, value.DidUri)
	FfiConverterTypeDocumentDataINSTANCE.Write(writer, value.Document)
	FfiConverterSequenceTypeJwkDataINSTANCE.Write(writer, value.PrivateJwks)
}

type FfiDestroyerTypePortableDidData struct{}

func (_ FfiDestroyerTypePortableDidData) Destroy(value PortableDidData) {
	value.Destroy()
}

type ResolutionMetadataData struct {
	Error *ResolutionMetadataError
}

func (r *ResolutionMetadataData) Destroy() {
	FfiDestroyerOptionalTypeResolutionMetadataError{}.Destroy(r.Error)
}

type FfiConverterTypeResolutionMetadataData struct{}

var FfiConverterTypeResolutionMetadataDataINSTANCE = FfiConverterTypeResolutionMetadataData{}

func (c FfiConverterTypeResolutionMetadataData) Lift(rb RustBufferI) ResolutionMetadataData {
	return LiftFromRustBuffer[ResolutionMetadataData](c, rb)
}

func (c FfiConverterTypeResolutionMetadataData) Read(reader io.Reader) ResolutionMetadataData {
	return ResolutionMetadataData{
		FfiConverterOptionalTypeResolutionMetadataErrorINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeResolutionMetadataData) Lower(value ResolutionMetadataData) RustBuffer {
	return LowerIntoRustBuffer[ResolutionMetadataData](c, value)
}

func (c FfiConverterTypeResolutionMetadataData) Write(writer io.Writer, value ResolutionMetadataData) {
	FfiConverterOptionalTypeResolutionMetadataErrorINSTANCE.Write(writer, value.Error)
}

type FfiDestroyerTypeResolutionMetadataData struct{}

func (_ FfiDestroyerTypeResolutionMetadataData) Destroy(value ResolutionMetadataData) {
	value.Destroy()
}

type ResolutionResultData struct {
	Document           *DocumentData
	DocumentMetadata   *DocumentMetadataData
	ResolutionMetadata ResolutionMetadataData
}

func (r *ResolutionResultData) Destroy() {
	FfiDestroyerOptionalTypeDocumentData{}.Destroy(r.Document)
	FfiDestroyerOptionalTypeDocumentMetadataData{}.Destroy(r.DocumentMetadata)
	FfiDestroyerTypeResolutionMetadataData{}.Destroy(r.ResolutionMetadata)
}

type FfiConverterTypeResolutionResultData struct{}

var FfiConverterTypeResolutionResultDataINSTANCE = FfiConverterTypeResolutionResultData{}

func (c FfiConverterTypeResolutionResultData) Lift(rb RustBufferI) ResolutionResultData {
	return LiftFromRustBuffer[ResolutionResultData](c, rb)
}

func (c FfiConverterTypeResolutionResultData) Read(reader io.Reader) ResolutionResultData {
	return ResolutionResultData{
		FfiConverterOptionalTypeDocumentDataINSTANCE.Read(reader),
		FfiConverterOptionalTypeDocumentMetadataDataINSTANCE.Read(reader),
		FfiConverterTypeResolutionMetadataDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeResolutionResultData) Lower(value ResolutionResultData) RustBuffer {
	return LowerIntoRustBuffer[ResolutionResultData](c, value)
}

func (c FfiConverterTypeResolutionResultData) Write(writer io.Writer, value ResolutionResultData) {
	FfiConverterOptionalTypeDocumentDataINSTANCE.Write(writer, value.Document)
	FfiConverterOptionalTypeDocumentMetadataDataINSTANCE.Write(writer, value.DocumentMetadata)
	FfiConverterTypeResolutionMetadataDataINSTANCE.Write(writer, value.ResolutionMetadata)
}

type FfiDestroyerTypeResolutionResultData struct{}

func (_ FfiDestroyerTypeResolutionResultData) Destroy(value ResolutionResultData) {
	value.Destroy()
}

type ServiceData struct {
	Id              string
	Type            string
	ServiceEndpoint []string
}

func (r *ServiceData) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerString{}.Destroy(r.Type)
	FfiDestroyerSequenceString{}.Destroy(r.ServiceEndpoint)
}

type FfiConverterTypeServiceData struct{}

var FfiConverterTypeServiceDataINSTANCE = FfiConverterTypeServiceData{}

func (c FfiConverterTypeServiceData) Lift(rb RustBufferI) ServiceData {
	return LiftFromRustBuffer[ServiceData](c, rb)
}

func (c FfiConverterTypeServiceData) Read(reader io.Reader) ServiceData {
	return ServiceData{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeServiceData) Lower(value ServiceData) RustBuffer {
	return LowerIntoRustBuffer[ServiceData](c, value)
}

func (c FfiConverterTypeServiceData) Write(writer io.Writer, value ServiceData) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterStringINSTANCE.Write(writer, value.Type)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.ServiceEndpoint)
}

type FfiDestroyerTypeServiceData struct{}

func (_ FfiDestroyerTypeServiceData) Destroy(value ServiceData) {
	value.Destroy()
}

type VerifiableCredentialCreateOptionsData struct {
	Id             *string
	Context        *[]string
	Type           *[]string
	IssuanceDate   *time.Time
	ExpirationDate *time.Time
}

func (r *VerifiableCredentialCreateOptionsData) Destroy() {
	FfiDestroyerOptionalString{}.Destroy(r.Id)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.Context)
	FfiDestroyerOptionalSequenceString{}.Destroy(r.Type)
	FfiDestroyerOptionalTimestamp{}.Destroy(r.IssuanceDate)
	FfiDestroyerOptionalTimestamp{}.Destroy(r.ExpirationDate)
}

type FfiConverterTypeVerifiableCredentialCreateOptionsData struct{}

var FfiConverterTypeVerifiableCredentialCreateOptionsDataINSTANCE = FfiConverterTypeVerifiableCredentialCreateOptionsData{}

func (c FfiConverterTypeVerifiableCredentialCreateOptionsData) Lift(rb RustBufferI) VerifiableCredentialCreateOptionsData {
	return LiftFromRustBuffer[VerifiableCredentialCreateOptionsData](c, rb)
}

func (c FfiConverterTypeVerifiableCredentialCreateOptionsData) Read(reader io.Reader) VerifiableCredentialCreateOptionsData {
	return VerifiableCredentialCreateOptionsData{
		FfiConverterOptionalStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalSequenceStringINSTANCE.Read(reader),
		FfiConverterOptionalTimestampINSTANCE.Read(reader),
		FfiConverterOptionalTimestampINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeVerifiableCredentialCreateOptionsData) Lower(value VerifiableCredentialCreateOptionsData) RustBuffer {
	return LowerIntoRustBuffer[VerifiableCredentialCreateOptionsData](c, value)
}

func (c FfiConverterTypeVerifiableCredentialCreateOptionsData) Write(writer io.Writer, value VerifiableCredentialCreateOptionsData) {
	FfiConverterOptionalStringINSTANCE.Write(writer, value.Id)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.Context)
	FfiConverterOptionalSequenceStringINSTANCE.Write(writer, value.Type)
	FfiConverterOptionalTimestampINSTANCE.Write(writer, value.IssuanceDate)
	FfiConverterOptionalTimestampINSTANCE.Write(writer, value.ExpirationDate)
}

type FfiDestroyerTypeVerifiableCredentialCreateOptionsData struct{}

func (_ FfiDestroyerTypeVerifiableCredentialCreateOptionsData) Destroy(value VerifiableCredentialCreateOptionsData) {
	value.Destroy()
}

type VerifiableCredentialData struct {
	Context                         []string
	Type                            []string
	Id                              string
	JsonSerializedIssuer            string
	JsonSerializedCredentialSubject string
	IssuanceDate                    time.Time
	ExpirationDate                  *time.Time
}

func (r *VerifiableCredentialData) Destroy() {
	FfiDestroyerSequenceString{}.Destroy(r.Context)
	FfiDestroyerSequenceString{}.Destroy(r.Type)
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerString{}.Destroy(r.JsonSerializedIssuer)
	FfiDestroyerString{}.Destroy(r.JsonSerializedCredentialSubject)
	FfiDestroyerTimestamp{}.Destroy(r.IssuanceDate)
	FfiDestroyerOptionalTimestamp{}.Destroy(r.ExpirationDate)
}

type FfiConverterTypeVerifiableCredentialData struct{}

var FfiConverterTypeVerifiableCredentialDataINSTANCE = FfiConverterTypeVerifiableCredentialData{}

func (c FfiConverterTypeVerifiableCredentialData) Lift(rb RustBufferI) VerifiableCredentialData {
	return LiftFromRustBuffer[VerifiableCredentialData](c, rb)
}

func (c FfiConverterTypeVerifiableCredentialData) Read(reader io.Reader) VerifiableCredentialData {
	return VerifiableCredentialData{
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterSequenceStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterOptionalTimestampINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeVerifiableCredentialData) Lower(value VerifiableCredentialData) RustBuffer {
	return LowerIntoRustBuffer[VerifiableCredentialData](c, value)
}

func (c FfiConverterTypeVerifiableCredentialData) Write(writer io.Writer, value VerifiableCredentialData) {
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Context)
	FfiConverterSequenceStringINSTANCE.Write(writer, value.Type)
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterStringINSTANCE.Write(writer, value.JsonSerializedIssuer)
	FfiConverterStringINSTANCE.Write(writer, value.JsonSerializedCredentialSubject)
	FfiConverterTimestampINSTANCE.Write(writer, value.IssuanceDate)
	FfiConverterOptionalTimestampINSTANCE.Write(writer, value.ExpirationDate)
}

type FfiDestroyerTypeVerifiableCredentialData struct{}

func (_ FfiDestroyerTypeVerifiableCredentialData) Destroy(value VerifiableCredentialData) {
	value.Destroy()
}

type VerificationMethodData struct {
	Id           string
	Type         string
	Controller   string
	PublicKeyJwk JwkData
}

func (r *VerificationMethodData) Destroy() {
	FfiDestroyerString{}.Destroy(r.Id)
	FfiDestroyerString{}.Destroy(r.Type)
	FfiDestroyerString{}.Destroy(r.Controller)
	FfiDestroyerTypeJwkData{}.Destroy(r.PublicKeyJwk)
}

type FfiConverterTypeVerificationMethodData struct{}

var FfiConverterTypeVerificationMethodDataINSTANCE = FfiConverterTypeVerificationMethodData{}

func (c FfiConverterTypeVerificationMethodData) Lift(rb RustBufferI) VerificationMethodData {
	return LiftFromRustBuffer[VerificationMethodData](c, rb)
}

func (c FfiConverterTypeVerificationMethodData) Read(reader io.Reader) VerificationMethodData {
	return VerificationMethodData{
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterStringINSTANCE.Read(reader),
		FfiConverterTypeJwkDataINSTANCE.Read(reader),
	}
}

func (c FfiConverterTypeVerificationMethodData) Lower(value VerificationMethodData) RustBuffer {
	return LowerIntoRustBuffer[VerificationMethodData](c, value)
}

func (c FfiConverterTypeVerificationMethodData) Write(writer io.Writer, value VerificationMethodData) {
	FfiConverterStringINSTANCE.Write(writer, value.Id)
	FfiConverterStringINSTANCE.Write(writer, value.Type)
	FfiConverterStringINSTANCE.Write(writer, value.Controller)
	FfiConverterTypeJwkDataINSTANCE.Write(writer, value.PublicKeyJwk)
}

type FfiDestroyerTypeVerificationMethodData struct{}

func (_ FfiDestroyerTypeVerificationMethodData) Destroy(value VerificationMethodData) {
	value.Destroy()
}

type Dsa uint

const (
	DsaEd25519 Dsa = 1
)

type FfiConverterTypeDsa struct{}

var FfiConverterTypeDsaINSTANCE = FfiConverterTypeDsa{}

func (c FfiConverterTypeDsa) Lift(rb RustBufferI) Dsa {
	return LiftFromRustBuffer[Dsa](c, rb)
}

func (c FfiConverterTypeDsa) Lower(value Dsa) RustBuffer {
	return LowerIntoRustBuffer[Dsa](c, value)
}
func (FfiConverterTypeDsa) Read(reader io.Reader) Dsa {
	id := readInt32(reader)
	return Dsa(id)
}

func (FfiConverterTypeDsa) Write(writer io.Writer, value Dsa) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerTypeDsa struct{}

func (_ FfiDestroyerTypeDsa) Destroy(value Dsa) {
}

type ResolutionMetadataError uint

const (
	ResolutionMetadataErrorInvalidDid                 ResolutionMetadataError = 1
	ResolutionMetadataErrorNotFound                   ResolutionMetadataError = 2
	ResolutionMetadataErrorRepresentationNotSupported ResolutionMetadataError = 3
	ResolutionMetadataErrorMethodNotSupported         ResolutionMetadataError = 4
	ResolutionMetadataErrorInvalidDidDocument         ResolutionMetadataError = 5
	ResolutionMetadataErrorInvalidPublicKey           ResolutionMetadataError = 6
	ResolutionMetadataErrorInvalidDidDocumentLength   ResolutionMetadataError = 7
	ResolutionMetadataErrorInternalError              ResolutionMetadataError = 8
)

type FfiConverterTypeResolutionMetadataError struct{}

var FfiConverterTypeResolutionMetadataErrorINSTANCE = FfiConverterTypeResolutionMetadataError{}

func (c FfiConverterTypeResolutionMetadataError) Lift(rb RustBufferI) ResolutionMetadataError {
	return LiftFromRustBuffer[ResolutionMetadataError](c, rb)
}

func (c FfiConverterTypeResolutionMetadataError) Lower(value ResolutionMetadataError) RustBuffer {
	return LowerIntoRustBuffer[ResolutionMetadataError](c, value)
}
func (FfiConverterTypeResolutionMetadataError) Read(reader io.Reader) ResolutionMetadataError {
	id := readInt32(reader)
	return ResolutionMetadataError(id)
}

func (FfiConverterTypeResolutionMetadataError) Write(writer io.Writer, value ResolutionMetadataError) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerTypeResolutionMetadataError struct{}

func (_ FfiDestroyerTypeResolutionMetadataError) Destroy(value ResolutionMetadataError) {
}

type Web5Error struct {
	err error
}

func (err Web5Error) Error() string {
	return fmt.Sprintf("Web5Error: %s", err.err.Error())
}

func (err Web5Error) Unwrap() error {
	return err.err
}

// Err* are used for checking error type with `errors.Is`
var ErrWeb5ErrorError = fmt.Errorf("Web5ErrorError")

// Variant structs
type Web5ErrorError struct {
	Type    string
	Variant string
	Msg     string
}

func NewWeb5ErrorError(
	varType string,
	variant string,
	msg string,
) *Web5Error {
	return &Web5Error{
		err: &Web5ErrorError{
			Type:    varType,
			Variant: variant,
			Msg:     msg,
		},
	}
}

func (err Web5ErrorError) Error() string {
	return fmt.Sprint("Error",
		": ",

		"Type=",
		err.Type,
		", ",
		"Variant=",
		err.Variant,
		", ",
		"Msg=",
		err.Msg,
	)
}

func (self Web5ErrorError) Is(target error) bool {
	return target == ErrWeb5ErrorError
}

type FfiConverterTypeWeb5Error struct{}

var FfiConverterTypeWeb5ErrorINSTANCE = FfiConverterTypeWeb5Error{}

func (c FfiConverterTypeWeb5Error) Lift(eb RustBufferI) error {
	return LiftFromRustBuffer[error](c, eb)
}

func (c FfiConverterTypeWeb5Error) Lower(value *Web5Error) RustBuffer {
	return LowerIntoRustBuffer[*Web5Error](c, value)
}

func (c FfiConverterTypeWeb5Error) Read(reader io.Reader) error {
	errorID := readUint32(reader)

	switch errorID {
	case 1:
		return &Web5Error{&Web5ErrorError{
			Type:    FfiConverterStringINSTANCE.Read(reader),
			Variant: FfiConverterStringINSTANCE.Read(reader),
			Msg:     FfiConverterStringINSTANCE.Read(reader),
		}}
	default:
		panic(fmt.Sprintf("Unknown error code %d in FfiConverterTypeWeb5Error.Read()", errorID))
	}
}

func (c FfiConverterTypeWeb5Error) Write(writer io.Writer, value *Web5Error) {
	switch variantValue := value.err.(type) {
	case *Web5ErrorError:
		writeInt32(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Type)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Variant)
		FfiConverterStringINSTANCE.Write(writer, variantValue.Msg)
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiConverterTypeWeb5Error.Write", value))
	}
}

type FfiConverterOptionalBool struct{}

var FfiConverterOptionalBoolINSTANCE = FfiConverterOptionalBool{}

func (c FfiConverterOptionalBool) Lift(rb RustBufferI) *bool {
	return LiftFromRustBuffer[*bool](c, rb)
}

func (_ FfiConverterOptionalBool) Read(reader io.Reader) *bool {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterBoolINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalBool) Lower(value *bool) RustBuffer {
	return LowerIntoRustBuffer[*bool](c, value)
}

func (_ FfiConverterOptionalBool) Write(writer io.Writer, value *bool) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterBoolINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalBool struct{}

func (_ FfiDestroyerOptionalBool) Destroy(value *bool) {
	if value != nil {
		FfiDestroyerBool{}.Destroy(*value)
	}
}

type FfiConverterOptionalString struct{}

var FfiConverterOptionalStringINSTANCE = FfiConverterOptionalString{}

func (c FfiConverterOptionalString) Lift(rb RustBufferI) *string {
	return LiftFromRustBuffer[*string](c, rb)
}

func (_ FfiConverterOptionalString) Read(reader io.Reader) *string {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterStringINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalString) Lower(value *string) RustBuffer {
	return LowerIntoRustBuffer[*string](c, value)
}

func (_ FfiConverterOptionalString) Write(writer io.Writer, value *string) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalString struct{}

func (_ FfiDestroyerOptionalString) Destroy(value *string) {
	if value != nil {
		FfiDestroyerString{}.Destroy(*value)
	}
}

type FfiConverterOptionalTimestamp struct{}

var FfiConverterOptionalTimestampINSTANCE = FfiConverterOptionalTimestamp{}

func (c FfiConverterOptionalTimestamp) Lift(rb RustBufferI) *time.Time {
	return LiftFromRustBuffer[*time.Time](c, rb)
}

func (_ FfiConverterOptionalTimestamp) Read(reader io.Reader) *time.Time {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTimestampINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTimestamp) Lower(value *time.Time) RustBuffer {
	return LowerIntoRustBuffer[*time.Time](c, value)
}

func (_ FfiConverterOptionalTimestamp) Write(writer io.Writer, value *time.Time) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTimestampINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTimestamp struct{}

func (_ FfiDestroyerOptionalTimestamp) Destroy(value *time.Time) {
	if value != nil {
		FfiDestroyerTimestamp{}.Destroy(*value)
	}
}

type FfiConverterOptionalTypeDocumentData struct{}

var FfiConverterOptionalTypeDocumentDataINSTANCE = FfiConverterOptionalTypeDocumentData{}

func (c FfiConverterOptionalTypeDocumentData) Lift(rb RustBufferI) *DocumentData {
	return LiftFromRustBuffer[*DocumentData](c, rb)
}

func (_ FfiConverterOptionalTypeDocumentData) Read(reader io.Reader) *DocumentData {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeDocumentDataINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeDocumentData) Lower(value *DocumentData) RustBuffer {
	return LowerIntoRustBuffer[*DocumentData](c, value)
}

func (_ FfiConverterOptionalTypeDocumentData) Write(writer io.Writer, value *DocumentData) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeDocumentDataINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeDocumentData struct{}

func (_ FfiDestroyerOptionalTypeDocumentData) Destroy(value *DocumentData) {
	if value != nil {
		FfiDestroyerTypeDocumentData{}.Destroy(*value)
	}
}

type FfiConverterOptionalTypeDocumentMetadataData struct{}

var FfiConverterOptionalTypeDocumentMetadataDataINSTANCE = FfiConverterOptionalTypeDocumentMetadataData{}

func (c FfiConverterOptionalTypeDocumentMetadataData) Lift(rb RustBufferI) *DocumentMetadataData {
	return LiftFromRustBuffer[*DocumentMetadataData](c, rb)
}

func (_ FfiConverterOptionalTypeDocumentMetadataData) Read(reader io.Reader) *DocumentMetadataData {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeDocumentMetadataDataINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeDocumentMetadataData) Lower(value *DocumentMetadataData) RustBuffer {
	return LowerIntoRustBuffer[*DocumentMetadataData](c, value)
}

func (_ FfiConverterOptionalTypeDocumentMetadataData) Write(writer io.Writer, value *DocumentMetadataData) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeDocumentMetadataDataINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeDocumentMetadataData struct{}

func (_ FfiDestroyerOptionalTypeDocumentMetadataData) Destroy(value *DocumentMetadataData) {
	if value != nil {
		FfiDestroyerTypeDocumentMetadataData{}.Destroy(*value)
	}
}

type FfiConverterOptionalTypeVerifiableCredentialCreateOptionsData struct{}

var FfiConverterOptionalTypeVerifiableCredentialCreateOptionsDataINSTANCE = FfiConverterOptionalTypeVerifiableCredentialCreateOptionsData{}

func (c FfiConverterOptionalTypeVerifiableCredentialCreateOptionsData) Lift(rb RustBufferI) *VerifiableCredentialCreateOptionsData {
	return LiftFromRustBuffer[*VerifiableCredentialCreateOptionsData](c, rb)
}

func (_ FfiConverterOptionalTypeVerifiableCredentialCreateOptionsData) Read(reader io.Reader) *VerifiableCredentialCreateOptionsData {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeVerifiableCredentialCreateOptionsDataINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeVerifiableCredentialCreateOptionsData) Lower(value *VerifiableCredentialCreateOptionsData) RustBuffer {
	return LowerIntoRustBuffer[*VerifiableCredentialCreateOptionsData](c, value)
}

func (_ FfiConverterOptionalTypeVerifiableCredentialCreateOptionsData) Write(writer io.Writer, value *VerifiableCredentialCreateOptionsData) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeVerifiableCredentialCreateOptionsDataINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeVerifiableCredentialCreateOptionsData struct{}

func (_ FfiDestroyerOptionalTypeVerifiableCredentialCreateOptionsData) Destroy(value *VerifiableCredentialCreateOptionsData) {
	if value != nil {
		FfiDestroyerTypeVerifiableCredentialCreateOptionsData{}.Destroy(*value)
	}
}

type FfiConverterOptionalTypeResolutionMetadataError struct{}

var FfiConverterOptionalTypeResolutionMetadataErrorINSTANCE = FfiConverterOptionalTypeResolutionMetadataError{}

func (c FfiConverterOptionalTypeResolutionMetadataError) Lift(rb RustBufferI) *ResolutionMetadataError {
	return LiftFromRustBuffer[*ResolutionMetadataError](c, rb)
}

func (_ FfiConverterOptionalTypeResolutionMetadataError) Read(reader io.Reader) *ResolutionMetadataError {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeResolutionMetadataErrorINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeResolutionMetadataError) Lower(value *ResolutionMetadataError) RustBuffer {
	return LowerIntoRustBuffer[*ResolutionMetadataError](c, value)
}

func (_ FfiConverterOptionalTypeResolutionMetadataError) Write(writer io.Writer, value *ResolutionMetadataError) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeResolutionMetadataErrorINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeResolutionMetadataError struct{}

func (_ FfiDestroyerOptionalTypeResolutionMetadataError) Destroy(value *ResolutionMetadataError) {
	if value != nil {
		FfiDestroyerTypeResolutionMetadataError{}.Destroy(*value)
	}
}

type FfiConverterOptionalSequenceString struct{}

var FfiConverterOptionalSequenceStringINSTANCE = FfiConverterOptionalSequenceString{}

func (c FfiConverterOptionalSequenceString) Lift(rb RustBufferI) *[]string {
	return LiftFromRustBuffer[*[]string](c, rb)
}

func (_ FfiConverterOptionalSequenceString) Read(reader io.Reader) *[]string {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterSequenceStringINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalSequenceString) Lower(value *[]string) RustBuffer {
	return LowerIntoRustBuffer[*[]string](c, value)
}

func (_ FfiConverterOptionalSequenceString) Write(writer io.Writer, value *[]string) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterSequenceStringINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalSequenceString struct{}

func (_ FfiDestroyerOptionalSequenceString) Destroy(value *[]string) {
	if value != nil {
		FfiDestroyerSequenceString{}.Destroy(*value)
	}
}

type FfiConverterOptionalSequenceTypeServiceData struct{}

var FfiConverterOptionalSequenceTypeServiceDataINSTANCE = FfiConverterOptionalSequenceTypeServiceData{}

func (c FfiConverterOptionalSequenceTypeServiceData) Lift(rb RustBufferI) *[]ServiceData {
	return LiftFromRustBuffer[*[]ServiceData](c, rb)
}

func (_ FfiConverterOptionalSequenceTypeServiceData) Read(reader io.Reader) *[]ServiceData {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterSequenceTypeServiceDataINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalSequenceTypeServiceData) Lower(value *[]ServiceData) RustBuffer {
	return LowerIntoRustBuffer[*[]ServiceData](c, value)
}

func (_ FfiConverterOptionalSequenceTypeServiceData) Write(writer io.Writer, value *[]ServiceData) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterSequenceTypeServiceDataINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalSequenceTypeServiceData struct{}

func (_ FfiDestroyerOptionalSequenceTypeServiceData) Destroy(value *[]ServiceData) {
	if value != nil {
		FfiDestroyerSequenceTypeServiceData{}.Destroy(*value)
	}
}

type FfiConverterOptionalMapStringString struct{}

var FfiConverterOptionalMapStringStringINSTANCE = FfiConverterOptionalMapStringString{}

func (c FfiConverterOptionalMapStringString) Lift(rb RustBufferI) *map[string]string {
	return LiftFromRustBuffer[*map[string]string](c, rb)
}

func (_ FfiConverterOptionalMapStringString) Read(reader io.Reader) *map[string]string {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterMapStringStringINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalMapStringString) Lower(value *map[string]string) RustBuffer {
	return LowerIntoRustBuffer[*map[string]string](c, value)
}

func (_ FfiConverterOptionalMapStringString) Write(writer io.Writer, value *map[string]string) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterMapStringStringINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalMapStringString struct{}

func (_ FfiDestroyerOptionalMapStringString) Destroy(value *map[string]string) {
	if value != nil {
		FfiDestroyerMapStringString{}.Destroy(*value)
	}
}

type FfiConverterSequenceString struct{}

var FfiConverterSequenceStringINSTANCE = FfiConverterSequenceString{}

func (c FfiConverterSequenceString) Lift(rb RustBufferI) []string {
	return LiftFromRustBuffer[[]string](c, rb)
}

func (c FfiConverterSequenceString) Read(reader io.Reader) []string {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]string, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterStringINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceString) Lower(value []string) RustBuffer {
	return LowerIntoRustBuffer[[]string](c, value)
}

func (c FfiConverterSequenceString) Write(writer io.Writer, value []string) {
	if len(value) > math.MaxInt32 {
		panic("[]string is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterStringINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceString struct{}

func (FfiDestroyerSequenceString) Destroy(sequence []string) {
	for _, value := range sequence {
		FfiDestroyerString{}.Destroy(value)
	}
}

type FfiConverterSequenceTypeJwkData struct{}

var FfiConverterSequenceTypeJwkDataINSTANCE = FfiConverterSequenceTypeJwkData{}

func (c FfiConverterSequenceTypeJwkData) Lift(rb RustBufferI) []JwkData {
	return LiftFromRustBuffer[[]JwkData](c, rb)
}

func (c FfiConverterSequenceTypeJwkData) Read(reader io.Reader) []JwkData {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]JwkData, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTypeJwkDataINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTypeJwkData) Lower(value []JwkData) RustBuffer {
	return LowerIntoRustBuffer[[]JwkData](c, value)
}

func (c FfiConverterSequenceTypeJwkData) Write(writer io.Writer, value []JwkData) {
	if len(value) > math.MaxInt32 {
		panic("[]JwkData is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTypeJwkDataINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTypeJwkData struct{}

func (FfiDestroyerSequenceTypeJwkData) Destroy(sequence []JwkData) {
	for _, value := range sequence {
		FfiDestroyerTypeJwkData{}.Destroy(value)
	}
}

type FfiConverterSequenceTypeServiceData struct{}

var FfiConverterSequenceTypeServiceDataINSTANCE = FfiConverterSequenceTypeServiceData{}

func (c FfiConverterSequenceTypeServiceData) Lift(rb RustBufferI) []ServiceData {
	return LiftFromRustBuffer[[]ServiceData](c, rb)
}

func (c FfiConverterSequenceTypeServiceData) Read(reader io.Reader) []ServiceData {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]ServiceData, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTypeServiceDataINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTypeServiceData) Lower(value []ServiceData) RustBuffer {
	return LowerIntoRustBuffer[[]ServiceData](c, value)
}

func (c FfiConverterSequenceTypeServiceData) Write(writer io.Writer, value []ServiceData) {
	if len(value) > math.MaxInt32 {
		panic("[]ServiceData is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTypeServiceDataINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTypeServiceData struct{}

func (FfiDestroyerSequenceTypeServiceData) Destroy(sequence []ServiceData) {
	for _, value := range sequence {
		FfiDestroyerTypeServiceData{}.Destroy(value)
	}
}

type FfiConverterSequenceTypeVerificationMethodData struct{}

var FfiConverterSequenceTypeVerificationMethodDataINSTANCE = FfiConverterSequenceTypeVerificationMethodData{}

func (c FfiConverterSequenceTypeVerificationMethodData) Lift(rb RustBufferI) []VerificationMethodData {
	return LiftFromRustBuffer[[]VerificationMethodData](c, rb)
}

func (c FfiConverterSequenceTypeVerificationMethodData) Read(reader io.Reader) []VerificationMethodData {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]VerificationMethodData, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterTypeVerificationMethodDataINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceTypeVerificationMethodData) Lower(value []VerificationMethodData) RustBuffer {
	return LowerIntoRustBuffer[[]VerificationMethodData](c, value)
}

func (c FfiConverterSequenceTypeVerificationMethodData) Write(writer io.Writer, value []VerificationMethodData) {
	if len(value) > math.MaxInt32 {
		panic("[]VerificationMethodData is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterTypeVerificationMethodDataINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceTypeVerificationMethodData struct{}

func (FfiDestroyerSequenceTypeVerificationMethodData) Destroy(sequence []VerificationMethodData) {
	for _, value := range sequence {
		FfiDestroyerTypeVerificationMethodData{}.Destroy(value)
	}
}

type FfiConverterMapStringString struct{}

var FfiConverterMapStringStringINSTANCE = FfiConverterMapStringString{}

func (c FfiConverterMapStringString) Lift(rb RustBufferI) map[string]string {
	return LiftFromRustBuffer[map[string]string](c, rb)
}

func (_ FfiConverterMapStringString) Read(reader io.Reader) map[string]string {
	result := make(map[string]string)
	length := readInt32(reader)
	for i := int32(0); i < length; i++ {
		key := FfiConverterStringINSTANCE.Read(reader)
		value := FfiConverterStringINSTANCE.Read(reader)
		result[key] = value
	}
	return result
}

func (c FfiConverterMapStringString) Lower(value map[string]string) RustBuffer {
	return LowerIntoRustBuffer[map[string]string](c, value)
}

func (_ FfiConverterMapStringString) Write(writer io.Writer, mapValue map[string]string) {
	if len(mapValue) > math.MaxInt32 {
		panic("map[string]string is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(mapValue)))
	for key, value := range mapValue {
		FfiConverterStringINSTANCE.Write(writer, key)
		FfiConverterStringINSTANCE.Write(writer, value)
	}
}

type FfiDestroyerMapStringString struct{}

func (_ FfiDestroyerMapStringString) Destroy(mapValue map[string]string) {
	for key, value := range mapValue {
		FfiDestroyerString{}.Destroy(key)
		FfiDestroyerString{}.Destroy(value)
	}
}

func DidDhtResolve(uri string) (*ResolutionResult, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeWeb5Error{}, func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_func_did_dht_resolve(FfiConverterStringINSTANCE.Lower(uri), _uniffiStatus)
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue *ResolutionResult
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterResolutionResultINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func DidJwkResolve(uri string) *ResolutionResult {
	return FfiConverterResolutionResultINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_web5_uniffi_fn_func_did_jwk_resolve(FfiConverterStringINSTANCE.Lower(uri), _uniffiStatus)
	}))
}

func Ed25519GeneratorGenerate() JwkData {
	return FfiConverterTypeJwkDataINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_web5_uniffi_fn_func_ed25519_generator_generate(_uniffiStatus)
	}))
}
