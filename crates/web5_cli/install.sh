#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

VERSION=$1
OS=$(uname | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
  "linux")
    case $ARCH in
      "x86_64") FILENAME="web5-x86_64-linux-gnu" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  "darwin")
    case $ARCH in
      "x86_64") FILENAME="web5-x86_64-apple-darwin" ;;
      "arm64") FILENAME="web5-aarch64-apple-darwin" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Download
curl -L -o /tmp/$FILENAME https://github.com/TBD54566975/web5-rs/releases/download/$VERSION/$FILENAME

# give it executable permissions
chmod +x /tmp/$FILENAME

# Move the executable to /usr/local/bin
sudo mv /tmp/$FILENAME /usr/local/bin/web5

# Cleanup
rm /tmp/$FILENAME

# Running health check
/usr/local/bin/web5 doctor
