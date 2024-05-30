#!/bin/sh
# DAEMON post-initialization script.

adb start-server
adb devices -l

/bin/daemon
