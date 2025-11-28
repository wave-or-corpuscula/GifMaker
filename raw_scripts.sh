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

echo -e "$first_text" > /tmp/f_text.txt
echo -e "$second_text" > /tmp/s_text.txt


rm $output

# Переход между двумя цветами
ffmpeg -y -filter_complex \
    "color=c=red:d=${duration}s [red]; \
     color=c=blue:d=${duration}s [blue]; \
     [red][blue]xfade=transition=$transition\
        :duration=${duration}s" $background

# Наложение текста на виде
ffmpeg -y -i $background \
       -vf "drawtext=textfile=/tmp/f_text.txt:reload=1:line_spacing=-10:x=(w-text_w)/2:y=(h-text_h)/2:fontsize=$font_size:fontcolor=blue,\
            drawtext=textfile=/tmp/s_text.txt:x=(w-text_w)/2:y=(h-text_h)/2:fontsize=$font_size:fontcolor=red:alpha='if(gte(t,2),if(lte(t,4),(t-2)/2,1),0)'" \
        -c:a copy $output

ffplay $output


andrey@andrey-virtual-machine:/tmp$ ffmpeg
ffmpeg version 6.1.1-3ubuntu5 Copyright (c) 2000-2023 the FFmpeg developers
  built with gcc 13 (Ubuntu 13.2.0-23ubuntu3)
  configuration: --prefix=/usr --extra-version=3ubuntu5 --toolchain=hardened --libdir=/usr/lib/x86_64-linux-gnu --incdir=/usr/include/x86_64-linux-gnu --arch=amd64 --enable-gpl --disable-stripping --disable-omx --enable-gnutls --enable-libaom --enable-libass --enable-libbs2b --enable-libcaca --enable-libcdio --enable-libcodec2 --enable-libdav1d --enable-libflite --enable-libfontconfig --enable-libfreetype --enable-libfribidi --enable-libglslang --enable-libgme --enable-libgsm --enable-libharfbuzz --enable-libmp3lame --enable-libmysofa --enable-libopenjpeg --enable-libopenmpt --enable-libopus --enable-librubberband --enable-libshine --enable-libsnappy --enable-libsoxr --enable-libspeex --enable-libtheora --enable-libtwolame --enable-libvidstab --enable-libvorbis --enable-libvpx --enable-libwebp --enable-libx265 --enable-libxml2 --enable-libxvid --enable-libzimg --enable-openal --enable-opencl --enable-opengl --disable-sndio --enable-libvpl --disable-libmfx --enable-libdc1394 --enable-libdrm --enable-libiec61883 --enable-chromaprint --enable-frei0r --enable-ladspa --enable-libbluray --enable-libjack --enable-libpulse --enable-librabbitmq --enable-librist --enable-libsrt --enable-libssh --enable-libsvtav1 --enable-libx264 --enable-libzmq --enable-libzvbi --enable-lv2 --enable-sdl2 --enable-libplacebo --enable-librav1e --enable-pocketsphinx --enable-librsvg --enable-libjxl --enable-shared
  WARNING: library configuration mismatch
  avcodec     configuration: --prefix=/usr --extra-version=3ubuntu5 --toolchain=hardened --libdir=/usr/lib/x86_64-linux-gnu --incdir=/usr/include/x86_64-linux-gnu --arch=amd64 --enable-gpl --disable-stripping --disable-omx --enable-gnutls --enable-libaom --enable-libass --enable-libbs2b --enable-libcaca --enable-libcdio --enable-libcodec2 --enable-libdav1d --enable-libflite --enable-libfontconfig --enable-libfreetype --enable-libfribidi --enable-libglslang --enable-libgme --enable-libgsm --enable-libharfbuzz --enable-libmp3lame --enable-libmysofa --enable-libopenjpeg --enable-libopenmpt --enable-libopus --enable-librubberband --enable-libshine --enable-libsnappy --enable-libsoxr --enable-libspeex --enable-libtheora --enable-libtwolame --enable-libvidstab --enable-libvorbis --enable-libvpx --enable-libwebp --enable-libx265 --enable-libxml2 --enable-libxvid --enable-libzimg --enable-openal --enable-opencl --enable-opengl --disable-sndio --enable-libvpl --disable-libmfx --enable-libdc1394 --enable-libdrm --enable-libiec61883 --enable-chromaprint --enable-frei0r --enable-ladspa --enable-libbluray --enable-libjack --enable-libpulse --enable-librabbitmq --enable-librist --enable-libsrt --enable-libssh --enable-libsvtav1 --enable-libx264 --enable-libzmq --enable-libzvbi --enable-lv2 --enable-sdl2 --enable-libplacebo --enable-librav1e --enable-pocketsphinx --enable-librsvg --enable-libjxl --enable-shared --enable-version3 --disable-doc --disable-programs --disable-static --enable-libaribb24 --enable-libopencore_amrnb --enable-libopencore_amrwb --enable-libtesseract --enable-libvo_amrwbenc --enable-libsmbclient