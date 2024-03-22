#!/bin/bash

(mvn clean compile;
  mvn package;
  cd target/classes/; 
  java -cp .:/Users/kendallw/.m2/repository/org/jetbrains/kotlin/kotlin-stdlib/1.6.10/kotlin-stdlib-1.6.10.jar com.example.HelloWorldKt)