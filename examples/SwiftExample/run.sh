#!/bin/bash

set -e

swift package clean
swift build
swift run