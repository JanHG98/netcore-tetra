#!/usr/bin/env python3
import json, math, time
phase=time.time()/20.0
forward=5.0+0.4*math.sin(phase)
reflected=0.12+0.03*math.sin(phase*1.7)
print(json.dumps({
  "metrics": {
    "forward_power_w": round(forward,3),
    "reflected_power_w": round(max(reflected,0),3),
    "pa_voltage_v": 13.72,
    "pa_current_a": round(2.1+0.1*math.sin(phase),3),
    "pa_temperature_c": round(48+2*math.sin(phase/2),2),
    "cabinet_temperature_c": 29.4,
    "fan_rpm": 2450
  },
  "inputs": {
    "antenna_fault": False,
    "pa_fault": False,
    "fan_fault": False,
    "high_swr_trip": False,
    "pll_lock": True
  },
  "metadata": {"probe":"mock-rf-probe","calibrated":False}
}))
