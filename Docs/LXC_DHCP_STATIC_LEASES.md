# LXC-Adressen über DHCP Static Leases

Die Backend-Installer enthalten keine feste Management-IP mehr. Jeder Installer
ermittelt beim Installieren oder Aktualisieren die aktuell im LXC aktive IPv4-
Adresse und schreibt sie in die lokale Dienstkonfiguration.

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
LXC neu starten und anschließend `install/update.sh` ausführen. Auch die
Update-Skripte aktualisieren die lokale Bind- und WebUI-Adresse.

## Andere LXC-Dienste

Ein LXC kann seine eigene Adresse sicher erkennen, nicht jedoch automatisch die
Adressen aller anderen Container. Abhängigkeiten wie Node Gateway, Call Control
oder Media Switch bleiben deshalb über deren tatsächliche IPs, lokale DNS-Namen
oder das Open-Lab-Inventory zu konfigurieren. Feste Beispieladressen im eigenen
`bind` oder in öffentlichen URLs sind dafür nicht mehr erforderlich.
