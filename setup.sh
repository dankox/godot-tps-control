#!/usr/bin/env bash

cargo build
cd godot

name="dslike"

if [ "$OS" = "Windows_NT" ]; then
	rustlib="${name}.dll"
else
	case $(uname -sm) in
	"Darwin x86_64") rustlib="lib${name}.dylib" ;;
	*) rustlib="lib${name}.so" ;;
	esac
fi

ln -s "../target/debug/${rustlib}" "${rustlib}"