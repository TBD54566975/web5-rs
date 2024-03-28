#!/bin/bash

mvn clean compile
mvn package
mvn dependency:copy-dependencies

# Ensure this runs from the project root directory
java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt