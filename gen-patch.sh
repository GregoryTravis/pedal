#!/bin/bash

name=$1

export PEDAL_PATCH_NAME=$name

tmp=tmp.tmp

cp -r patch-template $name

envsubst < $name/patch/Cargo.toml > $tmp
mv $tmp $name/patch/Cargo.toml

envsubst < $name/Makefile > $tmp
mv $tmp $name/Makefile
