#!/bin/bash
set -u

for g in /sys/devices/system/cpu/cpufreq/policy*/scaling_governor; do
    echo performance > "$g" 2>/dev/null || true
done
for e in /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference; do
    echo performance > "$e" 2>/dev/null || true
done

sysctl -q -w kernel.numa_balancing=0 2>/dev/null || true

echo never > /sys/kernel/mm/transparent_hugepage/defrag 2>/dev/null || true

echo 1 > /sys/bus/workqueue/devices/writeback/cpumask 2>/dev/null || true

for b in /sys/devices/system/cpu/cpufreq/boost \
         /sys/devices/system/cpu/amd_pstate/cpb_boost \
         /sys/devices/system/cpu/cpufreq/policy*/boost; do
    [ -w "$b" ] && echo 1 > "$b" 2>/dev/null || true
done

exit 0