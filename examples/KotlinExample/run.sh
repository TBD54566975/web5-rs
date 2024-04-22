#!/bin/bash

set -e

# TODO the `sudo cp` command below is not what we want
(cd ../../bindings/kt; \
  sudo cp src/main/resources/natives/libweb5.dylib ~/Library/Java/JavaVirtualMachines/jdk-17.0.8_7.jdk/Contents/Home/bin; \
  mvn clean install)

mvn clean compile
mvn package
mvn dependency:copy-dependencies

java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt