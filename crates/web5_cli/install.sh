#!/bin/bash

# Clean up files in case of error
clean_up_temp () {
    [[ -f "/tmp/$FILENAME" ]] && rm -f "/tmp/$FILENAME"
}

trap clean_up_temp EXIT

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

VERSION=$1
OS=$(uname | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
  "linux")
    echo "Operating System - Architecture: Linux - $ARCH"
    case $ARCH in
      "x86_64") 
      if [[ "$VERSION" == "v0.0.2" || "$VERSION" == "v0.0.3" || "$VERSION" == "v0.0.4" ]]; then
        FILENAME="web5-x86_64-linux-gnu.zip" 
      else
        FILENAME="web5_cli-x86_64-unknown-linux-gnu"
      fi;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  "darwin")
    echo "Operating System - Architecture: macOS - $ARCH"
    case $ARCH in
      "x86_64") 
      if [[ "$VERSION" == "v0.0.2" || "$VERSION" == "v0.0.3" || "$VERSION" == "v0.0.4" ]]; then
        FILENAME="web5-x86_64-apple-darwin.zip"
      else
        FILENAME="web5_cli-x86_64-apple-darwin"
      fi;;
      "arm64") 
      if [[ "$VERSION" == "v0.0.2" || "$VERSION" == "v0.0.3" || "$VERSION" == "v0.0.4" ]]; then
        FILENAME="web5-aarch64-apple-darwin.zip"
      else
        FILENAME="web5_cli-aarch64-apple-darwin"
      fi;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Download
echo "Downloading $FILENAME"
curl -L -f -o /tmp/$FILENAME https://github.com/TBD54566975/web5-rs/releases/download/$VERSION/$FILENAME

# Check download errors
if [ $? -ne 0 ] ; then
  echo "Error while downloading $FILENAME"
  echo "Exiting..."
  exit 1
fi

# give it executable permissions
chmod +x /tmp/$FILENAME

# Move the executable to /usr/local/bin
if [ -d "$DIRECTORY" ]; then
  echo "Creating $DIRECTORY."
  mkdir $DIRECTORY
fi
sudo mv /tmp/$FILENAME /usr/local/bin/web5

# Cleanup
rm /tmp/$FILENAME