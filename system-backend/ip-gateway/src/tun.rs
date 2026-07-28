// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Kopplung von TETRA-Paketdaten an IP-Netze.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[cfg(target_os = "linux")]
// Was: Bindet das Untermodul linux in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod linux {
    use std::ffi::CString;
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Write};
    use std::os::fd::{AsRawFd, RawFd};

    // Was: Legt den festen Wert `TUNSETIFF` für tunsetiff fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    const TUNSETIFF: libc::c_ulong = 0x4004_54ca;
    // Was: Legt den festen Wert `TUNSETPERSIST` für tunsetpersist fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    const TUNSETPERSIST: libc::c_ulong = 0x4004_54cb;
    // Was: Legt den festen Wert `TUNSETOWNER` für tunsetowner fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    const TUNSETOWNER: libc::c_ulong = 0x4004_54cc;
    // Was: Legt den festen Wert `IFF_TUN` für iff virtuelle TUN-Netzwerkschnittstelle fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    const IFF_TUN: libc::c_short = 0x0001;
    // Was: Legt den festen Wert `IFF_NO_PI` für iff no pi fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    const IFF_NO_PI: libc::c_short = 0x1000;

    #[repr(C)]
    // Was: Legt mehrere alternative Speicheransichten für if req data fest.
    // Warum: Diese Darstellung wird nur dort genutzt, wo ein festes binäres Speicherformat unterschiedliche Sichtweisen benötigt.
    union IfReqData {
        flags: libc::c_short,
        padding: [u8; 24],
    }

    #[repr(C)]
    // Was: Bündelt die zusammengehörigen Werte für if req in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    struct IfReq {
        name: [libc::c_char; libc::IFNAMSIZ],
        data: IfReqData,
    }

    // Was: Bündelt die zusammengehörigen Werte für virtuelle TUN-Netzwerkschnittstelle device in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct TunDevice {
        file: File,
        name: String,
    }

    // Was: Implementiert das zugehörige Verhalten für `TunDevice`.
    // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
    impl TunDevice {
        // Was: Diese Funktion öffnet den vorgesehenen Arbeitsschritt.
        // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
        pub fn open(name: &str, owner_user: Option<&str>, persistent: bool) -> Result<Self, String> {
            if name.is_empty() || name.len() >= libc::IFNAMSIZ {
                return Err("invalid TUN interface name".to_string());
            }
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/net/tun")
                .map_err(|error| format!("open /dev/net/tun: {error}"))?;
            let mut request = IfReq {
                name: [0; libc::IFNAMSIZ],
                data: IfReqData {
                    flags: IFF_TUN | IFF_NO_PI,
                },
            };
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for (index, byte) in name.bytes().enumerate() {
                request.name[index] = byte as libc::c_char;
            }
            let result = unsafe { libc::ioctl(file.as_raw_fd(), TUNSETIFF, &mut request) };
            if result < 0 {
                return Err(format!(
                    "TUNSETIFF {name}: {}",
                    std::io::Error::last_os_error()
                ));
            }
            if let Some(owner_user) = owner_user.filter(|value| !value.trim().is_empty()) {
                let owner = CString::new(owner_user)
                    .map_err(|_| "TUN owner_user contains a NUL byte".to_string())?;
                let password = unsafe { libc::getpwnam(owner.as_ptr()) };
                if password.is_null() {
                    return Err(format!("TUN owner user {owner_user} does not exist"));
                }
                let uid = unsafe { (*password).pw_uid };
                let result = unsafe { libc::ioctl(file.as_raw_fd(), TUNSETOWNER, uid) };
                if result < 0 {
                    return Err(format!(
                        "TUNSETOWNER {owner_user}: {}",
                        std::io::Error::last_os_error()
                    ));
                }
            }
            if persistent {
                let result = unsafe { libc::ioctl(file.as_raw_fd(), TUNSETPERSIST, 1) };
                if result < 0 {
                    return Err(format!(
                        "TUNSETPERSIST {name}: {}",
                        std::io::Error::last_os_error()
                    ));
                }
            }
            set_nonblocking(file.as_raw_fd())?;
            Ok(Self {
                file,
                name: name.to_string(),
            })
        }

        // Was: Führt den Arbeitsschritt `name` für name aus.
        // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
        pub fn name(&self) -> &str {
            &self.name
        }

        // Was: Diese Funktion liest Datenpaket.
        // Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
        pub fn read_packet(&mut self, buffer: &mut [u8]) -> Result<Option<usize>, String> {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match self.file.read(buffer) {
                Ok(0) => Ok(None),
                Ok(size) => Ok(Some(size)),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => Ok(None),
                Err(error) => Err(format!("read TUN {}: {error}", self.name)),
            }
        }

        // Was: Diese Funktion schreibt Datenpaket.
        // Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
        pub fn write_packet(&mut self, packet: &[u8]) -> Result<(), String> {
            self.file
                .write_all(packet)
                .map_err(|error| format!("write TUN {}: {error}", self.name))
        }
    }

    // Was: Diese Funktion setzt nonblocking.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    fn set_nonblocking(fd: RawFd) -> Result<(), String> {
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags < 0 {
            return Err(format!(
                "fcntl(F_GETFL): {}",
                std::io::Error::last_os_error()
            ));
        }
        let result = unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) };
        if result < 0 {
            return Err(format!(
                "fcntl(F_SETFL): {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(())
    }

    pub use TunDevice as PlatformTunDevice;
}

#[cfg(not(target_os = "linux"))]
// Was: Bindet das Untermodul other in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod other {
    // Was: Bündelt die zusammengehörigen Werte für platform virtuelle TUN-Netzwerkschnittstelle device in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct PlatformTunDevice;

    // Was: Implementiert das zugehörige Verhalten für `PlatformTunDevice`.
    // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
    impl PlatformTunDevice {
        // Was: Diese Funktion öffnet den vorgesehenen Arbeitsschritt.
        // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
        pub fn open(
            _name: &str,
            _owner_user: Option<&str>,
            _persistent: bool,
        ) -> Result<Self, String> {
            Err("TUN is supported only on Linux".to_string())
        }
        // Was: Führt den Arbeitsschritt `name` für name aus.
        // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
        pub fn name(&self) -> &str {
            "unsupported"
        }
        // Was: Diese Funktion liest Datenpaket.
        // Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
        pub fn read_packet(&mut self, _buffer: &mut [u8]) -> Result<Option<usize>, String> {
            Ok(None)
        }
        // Was: Diese Funktion schreibt Datenpaket.
        // Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
        pub fn write_packet(&mut self, _packet: &[u8]) -> Result<(), String> {
            Err("TUN is supported only on Linux".to_string())
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux::PlatformTunDevice as TunDevice;
#[cfg(not(target_os = "linux"))]
pub use other::PlatformTunDevice as TunDevice;
