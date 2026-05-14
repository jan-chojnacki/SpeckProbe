#!/bin/bash

cpupower frequency-set -g performance

sysctl -w kernel.sched_migration_cost_ns=5000000
sysctl -w kernel.sched_autogroup_enabled=0
sysctl -w kernel.nmi_watchdog=0
sysctl -w kernel.perf_event_paranoid=-1

for irq_dir in /proc/irq/[0-9]*/; do
    echo 1 > "${irq_dir}smp_affinity" 2>/dev/null
done

modprobe msr

echo "tuning applied"