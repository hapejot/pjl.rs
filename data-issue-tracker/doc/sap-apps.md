Ja, du kannst Fiori-Apps von einem **SAP On-Premises-System** in die **SAP Business Technology Platform (BTP)** bringen – das ist sogar ein häufiger Anwendungsfall im Rahmen der **Cloud-Strategie von SAP**. Dabei gibt es verschiedene Möglichkeiten, je nachdem, was genau du erreichen willst (z. B. Reuse, Migration, Modernisierung). Im Folgenden eine Übersicht:

---

### ✅ Möglichkeiten, Fiori-Apps in die BTP zu bringen

#### 1. **Fiori-App als HTML5-App in die BTP deployen**

Du kannst deine Fiori-App (meist UI5-basierend) als eigenständige HTML5-App in die **SAP BTP, Cloud Foundry** oder **SAP BTP, ABAP Environment (Steampunk)** hochladen.

* **Typisch für Eigenentwicklungen.**
* Du kannst Backend-Daten über OData-Services (z. B. über SAP Gateway) konsumieren.
* Authentifizierung via **SAP Identity Authentication Service (IAS)**.
* Backend-Anbindung z. B. über **SAP Cloud Connector** (bei On-Prem).

#### 2. **Fiori Launchpad (FLP) auf der BTP nutzen**

Anstatt das On-Premises-Fiori-Launchpad zu nutzen, kannst du:

* Die App im **Launchpad-Service der BTP** einbinden.
* Dabei können sowohl cloud-native Apps als auch On-Prem-Apps (via Remote-Content) integriert werden.

#### 3. **Remote-Zugriff (Zentrale FLP mit Remote-Apps)**

Wenn du die App *nicht* migrieren, sondern *nur integrieren* willst:

* Du kannst mit dem **SAP Launchpad Service auf der BTP** eine zentrale Benutzeroberfläche schaffen.
* Dann wird die App **remote vom On-Prem-System aufgerufen** (ähnlich wie eine Verlinkung).
* Vorteil: Keine Migration nötig, aber trotzdem zentrale Oberfläche in der Cloud.

---

### 🔧 Voraussetzungen / Tools

| Komponente                          | Beschreibung                                      |
| ----------------------------------- | ------------------------------------------------- |
| **SAP Cloud Connector**             | Sichere Tunnelverbindung zur On-Prem-Landschaft   |
| **Destination Service (BTP)**       | Definition der Zielsysteme, z. B. On-Prem-Gateway |
| **SAP Identity Authentication**     | Zentrale Authentifizierung                        |
| **SAP Business Application Studio** | Für Anpassungen/Entwicklung der App               |
| **Fiori Tools / UI5 Tooling**       | Für Entwicklung und Deployment in BTP             |

---

### 🧭 Beispiel: Migration einer UI5-Fiori-App

1. **Quell-App identifizieren**
   → SAPUI5-App auf ABAP-Stack oder im WebIDE.

2. **Source-Code extrahieren**
   → z. B. aus `/UI5/UI5_REPOSITORY_LOAD`.

3. **App in Business Application Studio öffnen**
   → ggf. modernisieren (z. B. TypeScript, neues UI5-Thema).

4. **Deployment in die BTP**
   → über `cf push` oder via `ui5 deploy` in ein HTML5-Repo.

5. **App in Launchpad-Service registrieren**

---

### 💡 Wann macht eine Migration Sinn?

| Migration                                                           | Remote-Zugriff                                   |
| ------------------------------------------------------------------- | ------------------------------------------------ |
| Wenn App modernisiert werden soll oder Teil eines Cloud-Portals ist | Wenn die App unverändert bleiben kann            |
| Wenn du Cloud-Authentifizierung brauchst (IAS, SSO)                 | Wenn du SSO durch bestehende Infrastruktur nutzt |
| Wenn du Performance optimieren willst                               | Wenn minimale Anpassung nötig ist                |

---

Möchtest du mir sagen, **welche Art Fiori-App du hast** (Standard oder Eigenentwicklung) und **ob du sie migrieren oder nur integrieren willst**? Dann kann ich dir einen präziseren Fahrplan geben.


Gute Frage – und die kurze Antwort lautet:

> **Nein, eine SAP Standard-Fiori-App kannst du in der Regel *nicht komplett* in der BTP ausführen, wenn das SAP-Backend (z. B. S/4HANA) weiterhin On-Premises steht.**
> Du kannst sie aber **remote in das Launchpad auf der BTP integrieren**.

---

### ⚙️ Warum geht das nicht „einfach so“?

SAP Standard-Fiori-Apps sind meist **komplexer aufgebaut**:

| Bestandteil                         | Ort (Standard)                |
| ----------------------------------- | ----------------------------- |
| UI-Komponente (SAPUI5 App)          | SAP Frontend Server (On-Prem) |
| OData-Services                      | ABAP Backend (On-Prem)        |
| Rollen / Catalogs / Target Mappings | SAP Fiori Launchpad (On-Prem) |

Diese Apps sind **eng mit dem Backend verdrahtet** – viele erfordern:

* SAP-Geschäftslogik im ABAP-Backend
* Lokale OData-Services
* PFCG-Rollen und Business Catalogs

---

### ✅ Was ist **möglich**?

#### 🔹 **Remote-Inbetriebnahme in der BTP (Zentrales Fiori Launchpad)**

Du kannst die Standard-Fiori-Apps **remote im Launchpad-Service auf der BTP konsumieren**, über sogenannte:

> **"Content Federation" oder "Federated Content"**

So läuft die App technisch immer noch **im lokalen SAP-System**, aber sie erscheint im Cloud-Launchpad – mit Single Sign-On und zentraler Benutzerführung.

**Dazu brauchst du:**

* **SAP Cloud Connector** (On-Prem → BTP)
* **Content Federation** im Launchpad-Service (BTP)
* **Entsprechende Rollen/Catalogs im Backend**

#### 🔹 **Alternative: Rebuild oder Erweiterung**

Wenn du mehr Kontrolle willst oder z. B. eine App modernisieren oder erweitern willst, kannst du:

* Die App-Logik **neu in der BTP entwickeln**
* Die Backend-Daten über **OData oder REST-Services aus dem On-Prem-System** konsumieren

Das wäre aber dann keine **Standard-SAP-App mehr**, sondern eine **eigene UI5-App in der BTP**.

---

### 📝 Fazit

| Option                                 | Läuft in BTP?   | SAP Backend nötig? | Bemerkung                           |
| -------------------------------------- | --------------- | ------------------ | ----------------------------------- |
| Standard-Fiori-App (SAP) direkt in BTP | ❌ Nein          | Ja (lokal)         | Nicht möglich, da tief integriert   |
| Remote-Integration ins FLP (BTP)       | ✅ Ja (Frontend) | Ja (lokal)         | Empfohlen – via Content Federation  |
| Neuentwicklung in der BTP              | ✅ Ja            | Ja (via OData)     | Nur sinnvoll bei Individualisierung |

---

Wenn du mir sagst, **welche App oder welchen Geschäftsbereich (z. B. MM, SD, FI)** du im Kopf hast, kann ich dir sagen, ob die App „federated“ eingebunden werden kann oder ob sich eine Eigenentwicklung lohnt.

Für die genannten Standard-Apps **F1639A**, **F1643A** und **F1995** handelt es sich um SAP-MM‑Fiori-Apps (z. B. „Requisition Manager“), die typischerweise im On-Prem-System (S/4HANA) als Standard-Content ausgeliefert werden. Diese können **nicht vollständig in der BTP ausgeführt** werden – das Frontend wird zwar cloudfähig, das Backend bleibt aber lokal notwendig.

---

## ✅ Lösung: Remote-Ausführung via Content Federation

Du kannst die Apps **als Remotes in dein Fiori Launchpad in der BTP einbinden** – das nennt sich auch „Content Federation“ oder „Federated Content“. Dabei läuft:

* **Frontend** im Launchpad-Service der BTP (z. B. Build Work Zone)
* **Backend** inklusive OData-Services, Logik und PFCG-Rollen weiterhin im On‑Premises‑System

### 🔧 Technischer Ablauf – Schritt-für-Schritt

1. **SAP Cloud Connector einrichten**
   Sichere Verbindung On-Prem → BTP
2. **Destinations in der BTP konfigurieren**

   * Design‑Time: zur Abfrage der Kataloge/Catalog Content
   * Runtime: für die Ausführung der Apps
3. **Expose der PFCG-Rollen/Kataloge im ABAP-System**
   Transaktionen wie `/UI2/CDM3_EXP_SCOPE`, UCONCOCKPIT, PFCG
4. **Content Provider im Build Work Zone anlegen**
   Mit den obigen Destinations
5. **Synchronisation durchführen** – dann erscheinen die Apps im Launchpad
6. **Zuweisung der Role Collections in der BTP** – z. B. über IAS oder Identity Provisioning (automatisch möglich) ([community.sap.com][1], [joyk.com][2])

---

## 📝 Fazit speziell für F1639A, F1643A, F1995

Du kannst diese Apps **inklusive UI5‑Frontend in dein BTP‑Launchpad integrieren** – das On-Prem Backend bleibt zwar für Logik und Daten notwendig, aber für die Nutzer wirkt es wie eine Cloud-App.

Kein komplettes Rebuild nötig, sondern eine **Integration mit Single Sign‑On**, zentraler Benutzeroberfläche und rollenbasierter Zuweisung über BTP.

---

## 💡 Nächste Schritte für dich

* Klären, ob ihr **Build Work Zone** oder **Launchpad-Service** nutzt.
* Cloud Connector & Destinations einrichten.
* Im On-Prem-System Katalog/Rollen expose’n (F1639A etc.).
* Content Provider in BTP anlegen und syncen.
* App-Rollen via IAS oder Identity Provisioning den Nutzern zuordnen .

---

Möchtest du, dass ich dich bei einem dieser Schritte genauer unterstütze oder konkrete Befehle/Transaktionscodes (z. B. `/UI2/CDM3_EXP_SCOPE`) beisteure?

[1]: https://community.sap.com/t5/technology-blog-posts-by-sap/how-to-federate-your-abap-pfcg-role-containing-fiori-apps-to-sap-build-work/ba-p/13577722?utm_source=chatgpt.com "How to federate your ABAP PFCG Role containing Fio... - SAP Community"
[2]: https://www.joyk.com/dig/detail/1680158901370767?utm_source=chatgpt.com "Access Fiori apps via SAP Build Work Zone Site with Federated SAP S/4HANA Conten... - JOYK Joy of Geek, Geek News, Link all geek"
Hier sind die wichtigsten **Transaktionen**, die du im **SAP-Backend (On-Premises-System)** brauchst, um **SAP-Standard-Fiori-Apps (wie F1639A, F1643A, F1995)** in der **SAP BTP via Content Federation** bereitzustellen:

---

## 🔧 Relevante Transaktionen zur Content Federation

### 1. **/UI2/CDM3\_EXP\_SCOPE**

**Zweck:**
Definiert, welche **Fiori-Kataloge** (Business Catalogs) für die **Content Federation** zur Verfügung gestellt werden sollen.

**Typischer Ablauf:**

* Scope-Definition anlegen
* Katalog(e) (z. B. `SAP_MM_BC_PR_PROCESS_MC`) auswählen
* Expose-Flag setzen

---

### 2. **/UI2/CDM3\_EXP\_SYNC**

**Zweck:**
Synchronisiert die zuvor festgelegten Kataloge für die Federated Content-Verwendung.

**Wird benötigt, wenn:**

* Du neue Kataloge bereitstellst
* Änderungen im Scope vorgenommen wurden

---

### 3. **PFCG**

**Zweck:**
Zuweisung und Pflege von Rollen im SAP-System.

**Wichtig für:**

* Zuordnung der Fiori-Kataloge zu Benutzerrollen
* Kontrolle, welche Apps Benutzer im Launchpad sehen

Tipp: Die PFCG-Rolle sollte die **gleichen Kataloge enthalten**, die du per `/UI2/CDM3_EXP_SCOPE` freigibst.

---

### 4. **/IWFND/MAINT\_SERVICE**

**Zweck:**
Aktivierung und Prüfung von **OData-Services**, die von den Apps benötigt werden.

**Wichtig für:**

* Sicherstellen, dass Services wie `MM_PUR_REQ_MANAGE` aktiv sind
* Test der Service-URLs

---

### 5. **UCONCOCKPIT** (Optional, aber empfohlen)

**Zweck:**
Einrichten der **Whitelist für Remote-Aufrufe** aus der BTP (Unified Connectivity Cockpit).

**Wichtig für:**

* Autorisierung von Zugriffen aus der Cloud über den SAP Cloud Connector
* Sicherheit & Governance

---

## ✅ Optional: Test & Debugging

| Transaktion      | Zweck                                        |
| ---------------- | -------------------------------------------- |
| **/UI2/FLP**     | Lokales Launchpad zur Vorschau               |
| **/UI2/INTEROP** | Anzeigen der Benutzer-Apps & Target Mappings |
| **ST01**         | Systemtrace, z. B. bei fehlenden Rechten     |

---

Wenn du möchtest, kann ich dir eine Beispielkonfiguration geben – z. B. wie man den Katalog für App **F1639A („Requisition Manager“) freigibt** und synchronisiert. Sag einfach Bescheid!
Hier ist eine **Beispielkonfiguration für die Federated Content-Nutzung** der App **F1639A** („Requisition Manager“) in der **SAP BTP** – konkret, wie du sie aus deinem **SAP On-Premises-System** bereitstellst, sodass sie im **Launchpad-Service (BTP)** oder der **Build Work Zone, Standard Edition** erscheint.

---

## 🔧 Ziel: App F1639A in der BTP anzeigen (via Content Federation)

---

### 🔹 Schritt 1: Katalog identifizieren

Die App F1639A gehört zu folgendem **Business Catalog**:

* **Katalog:** `SAP_MM_BC_PR_PROCESS_MC`
* **Business Role Template:** `SAP_BR_PURCHASER`

👉 Du brauchst diesen Katalog für die Content-Federation.

---

### 🔹 Schritt 2: Scope anlegen in `/UI2/CDM3_EXP_SCOPE`

**Transaktion:** `/UI2/CDM3_EXP_SCOPE`

1. **Scope-ID anlegen**: z. B. `Z_SCOPE_BTP_FLP`
2. **Bezeichnung vergeben** (frei wählbar)
3. **Business Catalogs hinzufügen**:

   * `SAP_MM_BC_PR_PROCESS_MC` (für F1639A)
   * Falls du F1643A, F1995 ebenfalls brauchst:

     * `SAP_MM_BC_REQ_MANAGE_PC` (F1643A)
     * `SAP_MM_BC_REQ_LIST_PC` (F1995)
4. Häkchen bei „**Expose for Launchpad Content Aggregation**“ setzen
5. Speichern

---

### 🔹 Schritt 3: Synchronisation ausführen in `/UI2/CDM3_EXP_SYNC`

**Transaktion:** `/UI2/CDM3_EXP_SYNC`

1. Scope `Z_SCOPE_BTP_FLP` auswählen
2. „**Synchronize**“ starten
3. Meldung prüfen → „Export Successful“

Ergebnis: Die Inhalte (Apps, Target Mappings, etc.) dieses Scopes stehen jetzt über die BTP zur Verfügung.

---

### 🔹 Schritt 4: OData-Services aktivieren (falls nicht schon aktiv)

**Transaktion:** `/IWFND/MAINT_SERVICE`

Aktiviere ggf. folgende OData-Services:

| App    | OData-Service            |
| ------ | ------------------------ |
| F1639A | `MM_PUR_REQ_MANAGE`      |
| F1643A | `MM_PUR_REQ_SSP_APPROVE` |
| F1995  | `MM_PUR_REQ_LIST`        |

Falls diese Services fehlen oder nicht aktiv sind, bekommst du später beim App-Start Fehler („Service not found“).

---

### 🔹 Schritt 5: Rollen prüfen (PFCG)

**Transaktion:** `PFCG`

1. Öffne die Rolle `SAP_BR_PURCHASER` (oder deine Z-Rolle)
2. Sicherstellen, dass sie den Katalog `SAP_MM_BC_PR_PROCESS_MC` enthält
3. Diese Rolle muss dem Benutzer zugewiesen sein, der sich später in der BTP anmeldet (über SSO)

---

### 🔹 Schritt 6: BTP-Konfiguration (über Launchpad-Service)

In der **SAP BTP Cockpit** oder **Build Work Zone**:

1. **SAP Cloud Connector**: Verbindung zum Backend eingerichtet
2. **Destinations**:

   * Eine für **Design-Time** (`/sap/opu/odata/UI2/CDM3`) zur Synchronisation
   * Eine für **Runtime** (Zugriff auf OData-Services)
3. **Content Provider anlegen** (in Launchpad oder Work Zone)

   * Verweise auf die Scope-ID `Z_SCOPE_BTP_FLP`
4. **Synchronisieren**

Die App erscheint im **Federated Content** unter den „Available Apps“. Du kannst sie jetzt zu einem Space/Page hinzufügen und Rollen zuweisen.

---

## ✅ Ergebnis

* Die App F1639A läuft weiter im On-Prem-System
* Das UI wird im Cloud-Fiori-Launchpad bereitgestellt
* Single Sign-On über IAS (meist SAML)
* Keine Migration, kein Code-Transfer nötig

---

Wenn du möchtest, kann ich dir auch ein Beispiel für die **BTP-Destination-Konfiguration (Design-Time & Runtime)** geben – oder ein Beispiel für das JSON aus `/sap/bc/ui2/cdm3/context`.

Sag einfach, worauf du dich gerade konzentrierst.



Der **Aufwand**, um SAP-Standard-Fiori-Apps wie **F1639A, F1643A, F1995** in die **SAP Build Work Zone, Standard Edition** (BTP) zu integrieren, ist **überschaubar**, aber hängt von ein paar Rahmenbedingungen ab.

---

## 🕒 Aufwandsschätzung – Überblick

| Phase                                         | Dauer (geschätzt) | Beteiligte Rollen           |
| --------------------------------------------- | ----------------- | --------------------------- |
| 1. Cloud Connector einrichten                 | 0,5 – 1 Tag       | Basis / Netzwerk            |
| 2. OData & Services prüfen                    | 0,5 Tag           | SAP-Entwickler / Functional |
| 3. CDM-Scopes definieren                      | 1 – 2 Stunden     | SAP-Fiori-Admin / Dev       |
| 4. BTP-Destinations einrichten                | 0,5 Tag           | BTP-Admin                   |
| 5. Content Federation einrichten in Work Zone | 1 Tag             | BTP-Admin                   |
| 6. Rollen/Spaces/Pages zuordnen               | 0,5 – 1 Tag       | Fiori-Admin / Key User      |
| **Gesamtschätzung (Pilot)**                   | **2 – 3 Tage**    |                             |

> ⚠️ Der Aufwand gilt für ein initiales Setup mit ein paar Apps. Wenn deine BTP-Landschaft schon steht (Cloud Connector, IAS, Work Zone), kann es auch **innerhalb eines Tages** erledigt sein.

---

## 🎯 Voraussetzungen & Einflussfaktoren auf den Aufwand

### Technisch:

* **Ist der Cloud Connector schon aktiv?**

  * ⏱️ Wenn nein → 1 Tag Aufwand für Installation + Konfiguration
* **Gibt es bereits eine IAS/IDP-Verknüpfung (z. B. SSO)?**

  * ⏱️ Wenn nein → Aufwand für Benutzerzuordnung & Test
* **Wurde die Work Zone bereits eingerichtet?**

  * ⏱️ Wenn ja → nur Content Federation
  * ⏱️ Wenn nein → ca. 1 Tag für Setup, Branding, Navigation

### Funktional:

* **Welche Kataloge und Rollen müssen freigegeben werden?**

  * Standardrollen wie `SAP_BR_PURCHASER` sind schnell exportiert
  * Komplexität steigt mit Z-Rollen, Custom Catalogs

---

## ✅ Was du auf jeden Fall brauchst

### 1. **In SAP On-Prem:**

* `/UI2/CDM3_EXP_SCOPE` und `/UI2/CDM3_EXP_SYNC` einrichten
* OData-Services prüfen/aktivieren
* Rollen mit passenden Catalogs erstellen/prüfen

### 2. **In der BTP:**

* Cloud Connector-Instanz (verweist aufs SAP-System)
* Destinations (Design-Time & Runtime)
* Build Work Zone: Content Provider anlegen + Sync
* Role Collections für Benutzerverwaltung

---

## 💡 Empfehlungen zur Reduktion des Aufwands

* Beginne mit **einem Katalog** (z. B. `SAP_MM_BC_PR_PROCESS_MC`) – dann Schritt für Schritt erweitern.
* Verwende **Standardrollen von SAP**, um Customizing zu vermeiden.
* Automatisiere das Provisioning mit **SAP Identity Provisioning Service** (falls mehrere Nutzer involviert sind).

---

Möchtest du eine **Checkliste als PDF oder Excel** für dein Projektteam? Oder ein **technisches How-to-Dokument mit Screenshots** für die Konfiguration? Ich kann dir beides in wenigen Minuten erstellen.
