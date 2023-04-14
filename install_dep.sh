#!/bin/bash

if [ ! -d "./ironfish" ];then
    git clone -b oreorust-0.1.76 https://github.com/hairtail/ironfish.git --depth 1
fi