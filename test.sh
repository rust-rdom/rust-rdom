#!/usr/bin/env bash

sh sourcegen.sh
cargo test --package rdom
