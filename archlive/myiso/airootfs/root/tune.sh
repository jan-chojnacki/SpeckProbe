#!/bin/bash
cpupower frequency-set -g performance
sysctl -w kernel.perf_event_paranoid=-1