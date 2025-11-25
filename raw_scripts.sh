#!/bin/bash


# custom
# fade
# wipeleft
# wiperight
# wipeup
# wipedown
# slideleft
# slideright
# slideup
# slidedown
# circlecrop
# rectcrop
# distance
# fadeblack
# fadewhite
# radial
# smoothleft
# smoothright
# smoothup
# smoothdown
# circleopen
# circleclose
# vertopen
# vertclose
# horzopen
# horzclose
# dissolve
# pixelize
# diagtl
# diagtr
# diagbl
# diagbr
# hlslice
# hrslice
# vuslice
# vdslice
# hblur
# fadegrays
# wipetl
# wipetr
# wipebl
# wipebr
# squeezeh
# squeezev
# zoomin
# fadefast
# fadeslow
# hlwind
# hrwind
# vuwind
# vdwind
# coverleft
# coverright
# coverup
# coverdown
# revealleft
# revealright
# revealup
# revealdown

background=test/background.mp4
output=test/output.mp4
duration=6
transition=vertclose
font_size=20

max_length=15

first_text="wwwwwwwwwwwwwww\nwwwwww-wwwwwwww\nwwwwwwwwwwwwwww\nwwwwww-wwwwwwww"
second_text="Нет, я так не считаю"

echo -e "$first_text" > /tmp/first_text.txt
echo -e "$second_text" > /tmp/second_text.txt


rm $output

# Переход между двумя цветами
ffmpeg -y -filter_complex \
    "color=c=red:d=${duration}s [red]; \
     color=c=blue:d=${duration}s [blue]; \
     [red][blue]xfade=transition=$transition\
        :duration=${duration}s" $background

# Наложение текста на виде
ffmpeg -y -i $background \
       -vf "drawtext=textfile=/tmp/first_text.txt:reload=1:line_spacing=-10:x=(w-text_w)/2:y=(h-text_h)/2:fontsize=$font_size:fontcolor=blue,\
            drawtext=textfile=/tmp/second_text.txt:x=(w-text_w)/2:y=(h-text_h)/2:fontsize=$font_size:fontcolor=red:alpha='if(gte(t,2),if(lte(t,4),(t-2)/2,1),0)'" \
        -c:a copy $output

ffplay $output


