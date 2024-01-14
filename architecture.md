# Architektur des Distributed Merged File Systems

## Offene Fragen

### Wie sehen Attribute konkret aus

* wie lässt sich eine Verzeichnis-Beziehung ausdrücken?
* wie lassen sich ganzzahlige Werte ausdrücken?
* wie lassen sich Zeichenketten ausdrücken?

Beispiele:

Struktur:

Actors / Peter Capaldi / Local Hero / HD.mp4

Attribute:

HD.mp4: 
    Actors:
        - Name: Peter Capaldi
          Role: Oldsen
        - Name: Peter Riggert
          Role: Mac
        - Name: Burt Lancaster
          Role: Felix Happer

## Protokoll

Das System kommuniziert über ein einheitliches Protokoll miteinander.



## Controller

### Pflichten

Der Controller liefert Directories und deren Inhalt aus. Unabhängig davon, ob die Inhalte physisch oder nur virtuell zusammengefasst werden in Verzeichnissen.

Der Controller kennt alle Dateien beim Namen und deren Zugehörigkeit zu einzelnen Verzeichnissen.

Der Controller kennt Regeln, die eine Datei einem virtuellen Verzeichnis zuordnen.

### Erwartungen

1. Agenten melden sich an.
2. Agenten liefern Informationen über alle Dateien, die im lokalen Suchraum sind. Die Informationen müssen in Form von einer Menge von einheitlich definierten Attributen übergeben werden.
3. Agenten liefern automtisch aktualisierte Informationen, wenn sie sich ergeben.
4. Es existiert für jeden Suchbreich des Agenten auch ein File-Service, der die Zugriffe auf die Dateien ermöglicht. Diese Zugriffe muss aber nicht der Agent selber sicherstellen. Es kann auch z.B. über eine ssh oder http Schnittstelle auf die Dateien zugegeriffen werden. So kann auch eine Web-Site etc. eingebunden werden. Es braucht dafür dann nur einen spezialisierten Agenten.

### Generelle Spielregeln

Es gibt Verzeichnisse, die wiederum Dateien oder weitere Verzeichnisse enthalten.

Die Zugehörigkeit zu einem Verzeichnis basiert auf Regeln.

Wenn eine Datei oder ein Verzeichnis einem Verzeichnis zugeordnet werden soll, muss das System automatisch die passende Regel realisieren, d.h. ein Attribut muss dann entsprechend angepasst werden, so dass die Regel für die Selektion für das Verzeichnis zukünftig greift.


## Agent

### Pflichten

### Erwartungen


## Client

Ein Client kann sich entweder mit dem Controller oder einem Agent verbinden um dort Aktionen
auszuführen.

### Pflichten

### Erwartungen
