# Unleash Architektur – Glossar

Dieses Glossar beschreibt die wichtigsten Architekturelemente von [Unleash](https://getunleash.io), einem Open-Source-Feature-Flag-System.

---

## Root Level
- **API Access Tokens**: Schlüssel zur Authentifizierung und Autorisierung von API-Zugriffen. Sie werden benötigt, um SDKs oder externe Tools mit Unleash zu verbinden und steuern, welche Projekte und Umgebungen abgerufen oder verändert werden dürfen.
- **Projects**: Oberste Organisationseinheit, die Feature Flags und Umgebungen gruppiert. Projekte helfen, Features logisch zu trennen (z.B. nach Microservice, Team oder Produkt).
- **Segments**: Wiederverwendbare Sammlungen von Constraints, die mehreren Strategien zugeordnet werden können. Beispiel: Ein Segment "Beta-User" mit der Bedingung E-Mail-Domain = "@firma.de" kann für verschiedene Features genutzt werden.
- **Strategy Types**: Vorgefertigte oder benutzerdefinierte Aktivierungsstrategien für Feature Flags, z.B. "Standard", "Gradual Rollout", "UserWithId". Sie bestimmen, wie und für wen ein Feature aktiviert wird.
- **Tag Types**: Kategorien zur Verschlagwortung von Feature Flags, z.B. "experimentell", "sicherheitskritisch" oder "team:frontend". Erleichtert die Suche und Verwaltung.
- **Unleash Context Fields**: Kontextinformationen wie User-ID, Session, Umgebung, Region oder beliebige Custom-Attribute. Sie werden genutzt, um Strategien und Constraints dynamisch auszuwerten (z.B. nur für bestimmte Nutzergruppen).
- **User Groups & Roles**: Verwaltung von Benutzerrechten und -gruppen (RBAC). Beispiel: Nur Admins dürfen neue Projekte anlegen, Entwickler dürfen Feature Flags ändern.

---

## Projects
- **Project**: Enthält Feature Flags und deren Konfigurationen sowie die zugehörigen Umgebungen. Jeder Unleash-Instanz muss mindestens ein Projekt zugeordnet sein. Beispiel: Ein Projekt "Webshop" mit den Flags "checkout-redesign" und "recommendation-engine".

---

## Environments
- **Environments**: Verschiedene Ausführungsumgebungen wie Entwicklung, Test, Staging, Produktion. Ein Feature Flag kann in "dev" aktiv, in "prod" aber noch deaktiviert sein. Beispiel: "Dark Mode" ist nur in der Entwicklungsumgebung sichtbar.
- **Project Environments**: Jedes Projekt kann eine Teilmenge der globalen Environments nutzen. Beispiel: Ein internes Tool nutzt nur "dev" und "prod", ein Kundenprojekt zusätzlich "staging".

---

## Features (Feature Flags)
- **Feature Flag**: Schalter, um Funktionen gezielt ein- oder auszuschalten. Sie gehören zu einem Projekt und werden in Umgebungen aktiviert. Beispiel: Das Flag "new-search" aktiviert eine neue Suchfunktion für ausgewählte Nutzer.
- **Feature Flag Type**: Typisierung eines Feature Flags, z.B. "Release" (für neue Releases), "Experiment" (für A/B-Tests), "Permission" (für Berechtigungen).

---

## Activation Strategies
- **Activation Strategy**: Definiert, für wen und unter welchen Bedingungen ein Feature Flag aktiviert wird. Beispiele: "Gradual Rollout" (nur 10% der Nutzer), "UserWithId" (nur für bestimmte User-IDs), "RemoteAddress" (nur für bestimmte IP-Bereiche).
- **Strategy Constraints**: Zusätzliche Bedingungen, die für eine Strategie erfüllt sein müssen, z.B. nur für Nutzer mit bestimmter E-Mail-Domain oder nur an bestimmten Wochentagen. Beispiel: Rollout nur für Nutzer mit E-Mail-Adresse "@firma.de" und nur montags.

---

## Segments
- **Segment**: Eine benannte, wiederverwendbare Sammlung von Constraints, die auf mehrere Strategien angewendet werden kann. Änderungen am Segment wirken sich auf alle referenzierenden Strategien aus. Beispiel: Das Segment "Early Access" kann für verschiedene Features genutzt werden, um eine bestimmte Nutzergruppe zu adressieren.

---

## Variants
- **Variant**: Alternative Ausprägung eines Features, z.B. für A/B-Tests oder Multivariantentests. Beispiel: Ein Feature "button-color" hat die Varianten "blau", "grün" und "rot". Variants können pro Umgebung unterschiedlich konfiguriert werden.
- **Variant Payload**: Beliebige Zusatzdaten, die einer Variante zugeordnet werden können, z.B. ein JSON-Objekt mit Konfigurationswerten für die jeweilige Variante.

---

## Use Case Beispiel
- **Schrittweise Einführung**: Ein neues Feature wird zunächst nur in der Entwicklungsumgebung aktiviert, später per Rollout-Strategie (z.B. 5% der Nutzer) in Produktion ausgerollt. Beispiel: "Neues Checkout-Design" wird erst für interne Tester, dann für 5%, dann für alle Nutzer aktiviert.
- **A/B-Testing**: Über Variants können verschiedene Versionen eines Features an unterschiedliche Nutzergruppen ausgespielt werden. Beispiel: 50% der Nutzer sehen einen grünen Button, 50% einen blauen Button.

---

**Weitere Informationen:**
- [Unleash Dokumentation](https://docs.getunleash.io/understanding-unleash/the-anatomy-of-unleash)
- [Feature Flags – Best Practices](https://docs.getunleash.io/topics/feature-flags/feature-flag-best-practices)
