#!/bin/bash

set -e

# (cd ../../bindings/kt; ./install.sh)

mvn clean compile
mvn package
mvn dependency:copy-dependencies

java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt