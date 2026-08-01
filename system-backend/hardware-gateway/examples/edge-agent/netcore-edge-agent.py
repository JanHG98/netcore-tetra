#!/usr/bin/env python3
import argparse,json,os,subprocess,time,random
p=argparse.ArgumentParser(); p.add_argument('--broker',required=True); p.add_argument('--device-id',required=True); p.add_argument('--interval',type=int,default=5); p.add_argument('--name',default='NetCore Edge Node'); a=p.parse_args()
while True:
    temp=round(float(open('/sys/class/thermal/thermal_zone0/temp').read())/1000,1) if os.path.exists('/sys/class/thermal/thermal_zone0/temp') else round(25+random.random()*3,1)
    msg={'device_id':a.device_id,'name':a.name,'kind':'rack_monitor','firmware':'edge-agent-phase6','metrics':{'temperature_c':temp,'humidity_percent':45.0,'supply_voltage_v':12.2},'inputs':{'door_open':False,'water_alarm':False,'smoke_alarm':False},'outputs':{}}
    subprocess.run(['mosquitto_pub','-h',a.broker,'-t',f'netcore/v1/hardware/{a.device_id}/telemetry','-m',json.dumps(msg)],check=False)
    time.sleep(a.interval)
