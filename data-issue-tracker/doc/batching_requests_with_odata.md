# Batch-Verarbeitung von Anfragen mit OData

## Einleitung

Je älter eine Webanwendung wird, desto mehr Funktionen neigt sie dazu anzusammeln und desto mehr Endpunkte stellt sie bereit. Die Stilllegung oder Umstrukturierung ist selten eine Option, da Clients und deren Versionen weiterhin die Anwendung nutzen. Stellen Sie sich beispielsweise ein Frontend vor, das Dutzende von Anfragen beim ersten Seitenaufruf stellt, nur um alle verfügbaren Menüs, Benachrichtigungen, Startseiten-Elemente und mehr abzurufen. Dies dauert oft lange und führt zu Blockierungen.

## Lösungsansätze

Es gibt mehrere Möglichkeiten, solche Engpässe zu bewältigen:

- Einführung eines weiteren Endpunkts, um mehrere zusammenhängende Anfragen zu bündeln. Dies ist zwar keine schlechte Idee, erfordert jedoch Aufwand bei der Implementierung und Wartung.
- Batch-Verarbeitung mit GraphQL. Obwohl erfolgreich sein könnte, überwiegt die Wartung alter Endpunkte, und dies würde eine erhebliche Überarbeitung sowohl des Backends als auch des Frontends erfordern.
- Die dritte, weniger bekannte Option ist OData. Es bietet Batch-Verarbeitung als eine seiner Fähigkeiten.

## OData

OData (Open Data Protocol) wurde von Microsoft entwickelt. Es bietet Mechanismen zur Erweiterung der Funktionalität von REST-Endpunkten. Beispielsweise können Filterung und Auswahl bestimmter Felder zu einem Endpunkt hinzugefügt werden, der `IEnumerable<SomeViewModel>` zurückgibt, ohne dessen Inhalt zu ändern.

## Batch-Verarbeitung

Die Batch-Verarbeitung in OData funktioniert durch das Senden von Anfragen im MIME-Typ "multipart/mixed", wobei Grenzen die Anfragen im Batch trennen. Jede Batch-Komponente muss die HTTP-Methode, den Pfad und die erforderlichen Header angeben, um die Anfrage auszuführen. Die Antwort auf eine solche Anfrage ist ähnlich strukturiert und enthält die Antworten in der gleichen Reihenfolge wie die Anfragen.

## Beispiel: Batch-Anfragen

### Anfrage

```http
POST http://localhost:3000/odata/$batch
Content-Type: multipart/mixed; boundary=batch_id-12345

--batch_id-12345
Content-Type: application/http
Content-Transfer-Encoding: binary

GET /weatherforecast?$select=temperatureC HTTP/1.1
Accept: application/json

--batch_id-12345
Content-Type: application/http
Content-Transfer-Encoding: binary

GET /weatherforecast?$select=temperatureF HTTP/1.1
Accept: application/json

--batch_id-12345--
```

### Antwort

```http
HTTP/1.1 200 OK
Content-Type: multipart/mixed; boundary=batchresponse_67890

--batchresponse_67890
Content-Type: application/http
Content-Transfer-Encoding: binary

HTTP/1.1 200 OK
Content-Type: application/json

[{"TemperatureC": -15}, {"TemperatureC": -16}, {"TemperatureC": -6}]

--batchresponse_67890
Content-Type: application/http
Content-Transfer-Encoding: binary

HTTP/1.1 200 OK
Content-Type: application/json

[{"TemperatureF": 5}, {"TemperatureF": 10}, {"TemperatureF": 15}]

--batchresponse_67890--
```

## Fazit

Die Batch-Verarbeitungsfähigkeit von OData ist eine hilfreiche Option:

- Sie ermöglicht Batch-Verarbeitung mit minimalem Aufwand, wenn sie zu einer bestehenden Anwendung hinzugefügt wird, und die API kann unverändert bleiben.
- Es gibt Client-Optionen für .NET, JS und andere.
- Bei richtiger Feinabstimmung kann die Leistung verbessert werden: Middleware wie Authentifizierung wird nur einmal ausgeführt; es ist möglich, den gesamten Batch mit einer einzigen Datenbanksitzung zu erfüllen; während Anfragen in einem Batch standardmäßig immer noch nacheinander ausgeführt werden, kann die Verarbeitung durch Überschreiben von `ExecuteRequestMessagesAsync` parallelisiert werden.
- Wenn die Netzwerkgeschwindigkeit ein Problem darstellt, können einige Bytes auf der Leitung eingespart werden.

