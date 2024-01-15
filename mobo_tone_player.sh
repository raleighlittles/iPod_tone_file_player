#!/usr/bin/env bash


set -e
set +x

## Takes a given tone file and plays it using `beep`.
## Requires a motherboard speaker!

tone_file="$1"

if ! command -v beep &> /dev/null
then
    echo "Couldn't find beep command, make sure it is installed: sudo apt install beep"
    exit 1
else
    # Load the PC speaker module
    sudo modprobe --verbose pcspkr

    # Read the file
    while read tone_cmd; do

    # echo "Reading line $tone_cmd"

    # There's no official specification for Tone files but it seems like basically the first line is for metadata

    if [[ $tone_cmd =~ ^[[:digit:]] ]];
    then
        tone_cmd_arr=( $tone_cmd )

        ## Debug
        # echo "Tone command array: $tone_cmd_arr has length ${#tone_cmd_arr[@]}"
        # echo "Beep frequency (Hz) ${tone_cmd_arr[0]}"
        # echo "Beep duration (ms)  ${tone_cmd_arr[1]}"

        beep -f ${tone_cmd_arr[0]} -l ${tone_cmd_arr[1]}

    else
        echo "Skipping line $tone_cmd"
        continue
    fi


    done < "$tone_file"

fi