#!/bin/bash

# TODO find a way to not have to do this... should be contained within the project
sudo cp src/main/resources/natives/libjwk.dylib ~/Library/Java/JavaVirtualMachines/jdk-17.0.8_7.jdk/Contents/Home/bin

mvn clean compile
mvn package
mvn dependency:copy-dependencies

# Ensure this runs from the project root directory
java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt
