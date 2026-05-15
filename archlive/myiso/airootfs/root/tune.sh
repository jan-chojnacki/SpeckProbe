#!/bin/bash
cpupower frequency-set -g performance
sysctl -w kernel.perf_event_paranoid=-1
[ -f /sys/devices/system/cpu/cpufreq/boost ] && echo 1 > /sys/devices/system/cpu/cpufreq/boost