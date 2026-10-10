# LXC-Adressen über DHCP Static Leases

**Gültigkeit:** Dokumentation gegen `main` (`c3ccdb4`, 09.10.2026) geprüft. Installer, Konfiguration und API im aktuellen Quellbaum sind maßgeblich; Phasennamen und Beispieladressen sind keine Live-Abnahme. Für Updates den bestehenden Checkout und die tatsächlich gestartete Unit verwenden.

Die Installer, die `system-backend/shared/install/lxc-network.sh` einbinden, ermitteln die aktuell aktive IPv4-Adresse und schreiben sie in die lokale Dienstkonfiguration. Das ist kein universelles Verhalten jeder Komponente: insbesondere der Provisioning-Core-Updater übernimmt diesen Helper nicht automatisch. Vor Updates das konkrete Script und die aktive Bind-Adresse prüfen.

## Ablauf

1. Dem LXC im DHCP-Server eine Static Lease zuweisen.
2. LXC starten und prüfen, ob die Lease aktiv ist:

   ```bash
   ip -4 addr show scope global
   ip -4 route
   ```

3. Den jeweiligen `install/install.sh` als `root` ausführen.

Der Installer verwendet bevorzugt die Source-Adresse der Default Route und
fällt ansonsten auf die erste aktive globale IPv4-Adresse zurück. Loopback und
Link-Local-Adressen werden nicht akzeptiert.

Dabei werden automatisch gesetzt:

- der erste `bind`-Eintrag des Dienstes auf `<LXC-IP>:<WebUI-Port>`;
- `public_base_url`, sofern der Dienst diese Option besitzt;
- `advertised_endpoint`, sofern der Dienst diese Option besitzt;
- `/etc/netcore/lxc-network.env` mit Dienstname, IP, Port und WebUI-URL.

Am Ende zeigt der Installer die konkrete Adresse an, beispielsweise:

```text
LXC-Adresse erkannt: 10.0.20.47 (DHCP/static lease)
WebUI: http://10.0.20.47:8130/
```

## Mehrere Netzwerkschnittstellen

Wählt die automatische Erkennung die falsche Schnittstelle, kann die gewünschte
Adresse für genau diesen Lauf vorgegeben werden:

```bash
NETCORE_LXC_IP=10.0.20.47 bash system-backend/media-switch/install/install.sh
```

## Geänderte Lease

Nach einer Änderung der Static Lease zuerst die neue Lease beziehen bzw. den
LXC neu starten und anschließend `install/update.sh` ausführen. Nur Update-Skripte, die den gemeinsamen Netzwerk-Helper tatsächlich aufrufen, aktualisieren dabei Bind- und WebUI-Adresse. Bei anderen Diensten diese Werte ausdrücklich in der bestehenden Konfiguration korrigieren.

## Andere LXC-Dienste

Ein LXC kann seine eigene Adresse sicher erkennen, nicht jedoch automatisch die
Adressen aller anderen Container. Abhängigkeiten wie Node Gateway, Call Control
oder Media Switch bleiben deshalb über deren tatsächliche IPs, lokale DNS-Namen
oder das Open-Lab-Inventory zu konfigurieren. Feste Beispieladressen im eigenen
`bind` oder in öffentlichen URLs sind dafür nicht mehr erforderlich.
