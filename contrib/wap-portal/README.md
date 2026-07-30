# NetCore-Tetra WAP-Portal

Dieses Verzeichnis enthaelt ein statisches Referenzportal mit 19 Seiten in zwei Formaten:

- `xhtml/`: XHTML Basic 1.1
- `wml/`: WML 1.1

Alle Seiten sind innerhalb ihres Formats vollstaendig navigierbar. Die Startseiten sind:

- `xhtml/index.xhtml`
- `wml/index.wml`

Die Basisstation verwendet fuer den echten WSP/Openwave-Pfad zusaetzlich extrem kurze Routen wie `/x/st` und `/w/st`. Die lesbaren Aliase `/status.xhtml` und `/status.wml` bleiben ebenfalls gueltig.

## MIME-Typen

```text
.xhtml  application/vnd.wap.xhtml+xml; charset=UTF-8
.wml    text/vnd.wap.wml; charset=UTF-8
```
