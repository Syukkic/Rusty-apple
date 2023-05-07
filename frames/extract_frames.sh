#!/bin/bash

input_file="BadApple.mp4"
output_prefix="frame_"

ffmpeg -i $input_file -vf "fps=20" $output_prefix%04d.png
