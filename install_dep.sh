#!/bin/bash

if [ ! -d "./ironfish" ];then
    git clone -b rust-v0.1.74 https://github.com/hairtail/ironfish.git --depth 1
fi