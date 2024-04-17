#!/bin/bash

set -e

mvn compile
java -cp "target/classes/:target/dependency/*" com.example.HelloWorldKt