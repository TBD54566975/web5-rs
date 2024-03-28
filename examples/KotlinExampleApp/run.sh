#!/bin/bash

set -e

(cd ../../web5-kt; mvn clean install)

mvn clean compile
mvn package
mvn dependency:copy-dependencies

java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt