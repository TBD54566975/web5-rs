#/bin/bash

set -e

# TODO find a way to not have to do this... should be contained within the project
sudo cp src/main/resources/natives/libweb5.dylib ~/Library/Java/JavaVirtualMachines/jdk-17.0.8_7.jdk/Contents/Home/bin

mvn clean install