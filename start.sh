#!/bin/sh

# This script sets up a sane environment for abstp to run in. You can also build your own environment, if you like.
# This script doesn't automatically know everything. Please configure these lines to the best of your knowledge.
# Please ensure that evtest and xdotool are installed.

# PASS YOUR SCREEN AND TP RESOLUTION
SCREEN_RES="[1920,1200]"
TP_RES="[4655,2730]"
# PASS REGEXES THAT MATCH YOUR KEYBOARD AND TP NAMES IN /proc/bus/input/devices HERE
KBD_NAME="[^\"]*[Kk]eyboard[^\"]*"
TP_NAME="[^\"]*[Tt]ouchpad[&\"]*"

trap 'jobs -p | xargs kill' INT

set -xv
DIR=/tmp/abstp-$(whoami)-$$
# prepare FIFOs
mkdir -p $DIR
mkfifo $DIR/kb
mkfifo $DIR/tp
mkfifo $DIR/combine
mkfifo $DIR/xdo
chmod -R 700 $DIR
chown -R $(whoami) $DIR
# determine which devices to use
# Don't edit these regexes, please.
KBD_RE="/\nN: Name=\"$KBD_NAME\"[\s\S]*?\nH: Handlers=.*?(event\d*)/m"
TP_RE="/\nN: Name=\"$TP_NAME\"[\s\S]*?\nH: Handlers=.*?(event\d*)/m"
set +xv
DEVICES="$(</proc/bus/input/devices)"
set -xv
KBD_EVENT=$(cat /proc/bus/input/devices | perl -0777 -lane "print m$KBD_RE")
TP_EVENT=$(cat /proc/bus/input/devices | perl -0777 -lane "print m$TP_RE")
# Start companion daemons
evtest /dev/input/$TP_EVENT >$DIR/tp &
evtest /dev/input/$KBD_EVENT >$DIR/kb &
cat $DIR/kb >$DIR/combine &
cat $DIR/tp >$DIR/combine &
cat $DIR/xdo | xdotool - &
# Start abstp
if [[ -d "target/release" ]]; then
	cargo build --release
	export PATH=./target/release:$PATH
elif [[ -d "target/debug" ]]; then
	cargo build
	export PATH=./target/debug:$PATH
fi
cat $DIR/combine | abstp -r $SCREEN_RES -R $TP_RES >$DIR/xdo
jobs -p | xargs kill
wait
