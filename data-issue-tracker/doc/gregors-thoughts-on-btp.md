- Der eigene Identity Provider sollte an die BTP nur über SAP Idenity Services – SAP Identity Authentication angeschlossen werden. Das ist kein großer Aufwand hat aber den Vorteil dass man dann immer von der SAP Support bekommt.
- Beim Cloud Connector für die Produktive Nutzung mindestens 2 VMs. Eine für Dev/QA und eine für Produktion. Falls Failover benötigt wird dann dies bitte auch für Dev/QA. Also 4 VMs.
- Das Fiori Launchpad muss im On Prem System nicht aktiviert sein. Falls die Fiori App in der BTP rein mit Backend OData Services laufen soll, dann muss halt SAP Gateway aktiv sein. Das erfolgt heutzutage meist embedded. Also ohne SAP Frontend Server.
- Die App kann man in der BTP grundsätzlich auch Standalone ohne die BTP Variante vom Fiori Launchpad - SAP Build Work Zone betreiben. Nur wenn das nicht die einzige App bleibt sollte man schon überlegen Build Work Zone einzurichten.
- Wenn man Build Work Zone verwendet lässt SAP auch sicherlich gut mit sich reden wie das mit den Usern läuft die keinen User im SAP Backend haben. Das sollte man als aller erstes mit dem SAP Account Executive klären.
- Falls es eine individuelle Entwicklung für den Kunden ist sollte der Quelltext in einem Git Repository des Kunden liegen
- Für Build und Deploy in Dev würde ich SAP CI/CD Service empfehlen
- Transport in QA und Produktion dann mit dem SAP Cloud Transport Management Service