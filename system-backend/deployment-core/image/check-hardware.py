#!/usr/bin/env python3
"""Probe the real SXceiver before launching the radio service, independently of core/VPN."""
import subprocess
import sys

result = subprocess.run(['SoapySDRUtil', '--probe=driver=sx'], stdout=subprocess.PIPE,
                        stderr=subprocess.STDOUT, text=True, timeout=30)
print(result.stdout, end='', flush=True)
if result.returncode or 'No devices found' in result.stdout or 'Probe device' not in result.stdout or '[ERROR]' in result.stdout:
    print('SXceiver-Prüfung fehlgeschlagen; HAT/EEPROM, I²S/SPI und Geräteberechtigungen prüfen.', flush=True)
    sys.exit(1)
