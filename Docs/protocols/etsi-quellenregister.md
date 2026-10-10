# ETSI-Quellen und verwendete Normfassungen

**Registerstand:** 09.10.2026. Die folgende Liste wurde gegen die Titelseiten der25 bereitgestellten PDFs gelesen. Sie nennt die vorhandenen Fassungen, nicht die weltweit neueste Veröffentlichung. Ein Entwurf wird als Entwurf geführt; seine Anwesenheit ist kein Implementierungs- oder Konformitätsnachweis.

Der bestehende Air-Interface-Code verweist überwiegend auf EN300392-2 V3.8.1. Klauselnummern sind mit dieser festen Referenz zu lesen. Eine andere Fassung wird erst nach einem dokumentierten Delta zur Implementierungsgrundlage. PDU-/SAP-Inventur, Labortests und RF-/On-Air-Abnahme bleiben getrennte Nachweise.

| Bereitgestellte Datei | Titelseitenfassung | Einordnung |
| --- | --- | --- |
| `en_3003920308v010401p.pdf` | ETSI EN 300 392-3-8 V1.4.1 (2020-04) | Referenzfassung laut Titelseite |
| `en_30039209v010701p.pdf` | ETSI EN 300 392-9 V1.7.1 (2020-04) | Referenzfassung laut Titelseite |
| `ts_10081201v020205p.pdf` | ETSI TS 100 812-1 V2.2.5 (2003-10) | Referenzfassung laut Titelseite |
| `en_3003921201v010202p.pdf` | ETSI EN 300 392-12-1 V1.2.2 (2007-08) | Referenzfassung laut Titelseite |
| `en_3003920304v010301p.pdf` | ETSI EN 300 392-3-4 V1.3.1 (2010-08) | Referenzfassung laut Titelseite |
| `en_3003921117v010102p.pdf` | ETSI EN 300 392-11-17 V1.1.2 (2002-01) | Referenzfassung laut Titelseite |
| `en_3003921114v010101p.pdf` | ETSI EN 300 392-11-14 V1.1.1 (2002-07) | Referenzfassung laut Titelseite |
| `es_20081202v020401m.pdf` | ETSI ES 200 812-2 V2.4.1 (2005-08) | Entwurf laut Titelseite |
| `es_20081201v020205p.pdf` | ETSI ES 200 812-1 V2.2.5 (2003-12) | Referenzfassung laut Titelseite |
| `en_300812v020101p.pdf` | ETSI EN 300 812 V2.1.1 (2001-12) | Referenzfassung laut Titelseite |
| `en_3003921101v010201p.pdf` | ETSI EN 300 392-11-1 V1.2.1 (2004-01) | Referenzfassung laut Titelseite |
| `en_3003921006v010401p.pdf` | ETSI EN 300 392-10-6 V1.4.1 (2006-08) | Referenzfassung laut Titelseite |
| `en_3003921018v010301p.pdf` | ETSI EN 300 392-10-18 V1.3.1 (2003-10) | Referenzfassung laut Titelseite |
| `en_3003921216v010400a.pdf` | ETSI EN 300 392-12-16 V1.4.0 (2026-03) | Entwurf laut Titelseite |
| `en_30039201v010601p.pdf` | ETSI EN 300 392-1 V1.6.1 (2020-04) | Referenzfassung laut Titelseite |
| `ets_30039214e01v.pdf` | pr ETS300392-14, September1997 | Entwurf laut Titelseite |
| `en_30039207v030501p.pdf` | ETSI EN 300 392-7 V3.5.1 (2019-07) | Referenzfassung laut Titelseite |
| `en_30039401v030301p.pdf` | ETSI EN 300 394-1 V3.3.1 (2015-04) | Referenzfassung laut Titelseite |
| `en_3003920313v010201p.pdf` | ETSI EN 300 392-3-13 V1.2.1 (2020-04) | Referenzfassung laut Titelseite |
| `en_30039502v010303p.pdf` | ETSI EN 300 395-2 V1.3.3 (2025-02) | Referenzfassung laut Titelseite |
| `en_3003920303v010301p.pdf` | ETSI EN 300 392-3-3 V1.3.1 (2011-11) | Referenzfassung laut Titelseite |
| `en_30039205v020701p.pdf` | ETSI EN 300 392-5 V2.7.1 (2020-04) | Referenzfassung laut Titelseite |
| `en_3003920315v010500a.pdf` | Draft ETSI EN 300 392-3-15 V1.5.0 (2026-04) | Entwurf laut Titelseite |
| `en_30039202v030801p.pdf` | ETSI EN 300 392-2 V3.8.1 (2016-08) | Referenzfassung laut Titelseite |
| `ETSI.pdf` | ETSI EN 300 812 V2.1.1 (2001-12) | Referenzfassung laut Titelseite; gleiche Norm/Fassung wie en_300812v020101p.pdf, keine zusätzliche Norm |

## Zuordnung im Projekt

- Luftschnittstelle, MAC/LLC/MLE/MM/CMCE und SAPs: EN300392-2; [statische Protokollberichte](README.md).
- Allgemeines Netzdesign: EN300392-1; [Systemarchitektur](../wiki/architektur-und-datenwege.md).
- Sicherheit und TSIM/UICC: EN300392-7, EN300812 sowie TS/ES100/200812; [Security Core](../services/security-core/README.md) und [KMF](../services/kmf/README.md) beschreiben den tatsächlich implementierten Laborumfang.
- ISI-Sprache, Gruppenrufe, SDS und Mobility: EN300392-3-3/-4/-8/-13/-15; [Transit](../services/transit/README.md) ist derzeit NetCore-native Vermittlung und noch kein ETSI-ISI-Nachweis.
- Supplementary Services: EN300392-9 sowie die bereitgestellten Stage1/2/3-Teilnormen; konkrete Serviceabdeckung bleibt je Verfahren zu prüfen.
- Speech Codec: EN300395-2; [Media Library](../services/media-library/README.md) und [Medienpfad](../services/media-switch/README.md).
- Radio-Conformance-Tests: EN300394-1. Statische Codec-Inventuren ersetzen diese Tests nicht.
- PICS: der bereitgestellte pr-ETS300392-14-Entwurf ist ein historisches Raster für Fähigkeitslisten und kein heutiges TBS-Zertifikat.

Die PDFs werden durch dieses Register weder verändert noch in Softwarebundles kopiert. Bei exakten normativen Aussagen immer Dokumentnummer, Fassung und Klausel nennen.
