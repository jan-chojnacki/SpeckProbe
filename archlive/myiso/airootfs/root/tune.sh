#!/bin/bash
cpupower frequency-set -g performance
sysctl -w kernel.perf_event_paranoid=-1
sysctl -w kernel.kptr_restrict=0
sysctl -w kernel.numa_balancing=0
echo never > /sys/kernel/mm/transparent_hugepage/defrag
echo 0 > /sys/bus/workqueue/devices/writeback/cpumask
[ -f /sys/devices/system/cpu/cpufreq/boost ] && echo 1 > /sys/devices/system/cpu/cpufreq/boost