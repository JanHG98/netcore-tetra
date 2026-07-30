#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Metriken, Protokolle und Betriebsüberwachung.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Optional open-lab journald forwarder for a service LXC."""
import json, os, subprocess, time, urllib.request
ENDPOINT=os.environ.get("NETCORE_OBSERVABILITY_LOG_ENDPOINT","http://127.0.0.1:8210/api/v1/logs/ingest")
SERVICE=os.environ.get("NETCORE_SERVICE_NAME","unknown-service")
NODE=os.environ.get("NETCORE_NODE_NAME",os.uname().nodename)
BATCH=max(1,int(os.environ.get("NETCORE_LOG_BATCH","25")))
proc=subprocess.Popen(["journalctl","-f","-n","0","-o","json"],stdout=subprocess.PIPE,text=True)
buffer=[]
# Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
# Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
for line in proc.stdout:
    # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
    # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
    try:
        item=json.loads(line); message=item.get("MESSAGE","")
        if not message: continue
        priority=int(item.get("PRIORITY",6)); level="error" if priority<=3 else "warn" if priority==4 else "info" if priority<=6 else "debug"
        buffer.append({"timestamp":None,"service":SERVICE,"node":NODE,"level":level,"message":str(message),"correlation_id":None,"trace_id":None,"fields":{"unit":item.get("_SYSTEMD_UNIT"),"pid":item.get("_PID")}})
        if len(buffer)<BATCH: continue
        payload=json.dumps({"records":buffer}).encode(); request=urllib.request.Request(ENDPOINT,data=payload,headers={"Content-Type":"application/json"},method="POST")
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            with urllib.request.urlopen(request,timeout=5): pass
            buffer.clear()
        except Exception as error:
            print(f"journal forward failed: {error}",flush=True); time.sleep(2)
    except Exception as error:
        print(f"journal parse failed: {error}",flush=True)
