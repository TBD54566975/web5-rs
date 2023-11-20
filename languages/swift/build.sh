THIS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $THIS_DIR/../..

NAME="web5_ffi"
BINDINGS_DIR="languages/swift/bindings"
FRAMEWORK_DIR="languages/swift/framework"
TEMP_HEADER_DIR="${BINDINGS_DIR}/include"
TARGET_ARCHES=(
    aarch64-apple-ios
    aarch64-apple-ios-sim
)

# Cleanup dirs
rm -rf "${BINDINGS_DIR}"
rm -rf "${FRAMEWORK_DIR}"

# Build library for each target architecture
for arch in "${TARGET_ARCHES[@]}"; do
    rustup target add "$arch"
    cargo build -p web5-ffi --target "$arch" --release
done

# Generate Swift bindings
cargo run -p uniffi-bindgen generate \
    target/${TARGET_ARCHES[0]}/release/lib${NAME}.dylib \
    --library \
    --language swift \
    --out-dir "$BINDINGS_DIR"

# Move headers and module map into a temp directory, 
# with proper naming conventions for xcframework
mkdir -p "${TEMP_HEADER_DIR}"

# Move each header file into the temp directory, with the proper name
for header_file in "${BINDINGS_DIR}"/*.h; do
    mv "$header_file" "${TEMP_HEADER_DIR}"
done

destination_module_map="${TEMP_HEADER_DIR}/module.modulemap"
for module_map_file in "${BINDINGS_DIR}"/*.modulemap; do
    cat "$module_map_file" >> "$destination_module_map"
    echo "\n\n" >> "$destination_module_map"
done

# Build a new xcframework, with libraries and headers for each target architecture
XCODE_BUILD_CMD="xcodebuild -create-xcframework"
for arch in "${TARGET_ARCHES[@]}"; do
    XCODE_BUILD_CMD+=" -library \"target/${arch}/release/lib${NAME}.a\""
    XCODE_BUILD_CMD+=" -headers \"${TEMP_HEADER_DIR}\""
done
XCODE_BUILD_CMD+=" -output \"${FRAMEWORK_DIR}/${NAME}_framework.xcframework\""
echo "{$XCODE_BUILD_CMD}"
eval "$XCODE_BUILD_CMD"

# Remove the temp header directory
rm -rf "${TEMP_HEADER_DIR}"
