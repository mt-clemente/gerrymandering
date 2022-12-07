#!/bin/sh
p_option=false
while getopts e:c:p flag
do
    case "${flag}" in
        e) sample=${OPTARG};;
        c) m=${OPTARG};;
        p) p_option=true;;
    esac
done

./target/release/gerrymandering $sample $m $p_option