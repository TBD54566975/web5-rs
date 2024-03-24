#!/bin/bash

sudo cp ../../target/release/libjwk.dylib /Users/kendallw/Library/Java/JavaVirtualMachines/jdk-17.0.8_7.jdk/Contents/Home/bin

mvn clean compile
mvn package
mvn dependency:copy-dependencies

# Ensure this runs from the project root directory
java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt
