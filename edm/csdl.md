## Association

Ein Association-Element definiert eine Beziehung zwischen zwei Entitätstypen. Eine Zuordnung muss die Entitätstypen, die in der Beziehung enthalten sind, und die mögliche Anzahl von Entitätstypen an den Enden der Beziehung angeben, die auch als Multiplizität bezeichnet wird. Die Multiplizität eines Zuordnungsendes kann über einen Wert von eins (1), null oder eins (0..1) oder n (*) verfügen. Diese Informationen werden in zwei untergeordneten End-Elementen angegeben.

Auf Entitätstypinstanzen an einem Ende einer Zuordnung kann über Navigationseigenschaften oder Fremdschlüssel zugegriffen werden, sofern sie für einen Entitätstyp verfügbar gemacht werden.

In einer Anwendung stellt eine Instanz einer Zuordnung eine bestimmte Zuordnung zwischen Instanzen von Entitätstypen dar. Zuordnungsinstanzen werden logisch in einem Zuordnungssatz gruppiert.

```xml
<Association Name="PublishedBy">
      <End Type="BooksModel.Book" Role="Book" Multiplicity="*" />
      <End Type="BooksModel.Publisher" Role="Publisher" Multiplicity="1" />
</Association>
```

## AssociationSet

Das AssociationSet-Element in der konzeptionellen Schemadefinitionssprache (CSDL) ist ein logischer Container für Zuordnungsinstanzen desselben Typs. Ein Zuordnungssatz stellt eine Definition zum Gruppieren von Zuordnungsinstanzen bereit, sodass sie einer Datenquelle zugeordnet werden können.  

```xml
<EntityContainer Name="BooksContainer" >
  <EntitySet Name="Books" EntityType="BooksModel.Book" />
  <EntitySet Name="FictionBooks" EntityType="BooksModel.Book" />
  <EntitySet Name="Publishers" EntityType="BooksModel.Publisher" />
  <EntitySet Name="Authors" EntityType="BooksModel.Author" />
  <EntitySet Name="FictionAuthors" EntityType="BooksModel.Author" />
  <AssociationSet Name="PublishedBy" Association="BooksModel.PublishedBy">
    <End Role="Book" EntitySet="Books" />
    <End Role="Publisher" EntitySet="Publishers" />
  </AssociationSet>
  <AssociationSet Name="WrittenBy" Association="BooksModel.WrittenBy">
    <End Role="Book" EntitySet="Books" />
    <End Role="Author" EntitySet="Authors" />
  </AssociationSet>
  <AssociationSet Name="FictionWrittenBy" Association="BooksModel.WrittenBy">
    <End Role="Book" EntitySet="FictionBooks" />
    <End Role="Author" EntitySet="FictionAuthors" />
  </AssociationSet>
</EntityContainer>
```
## CollectionType

Das CollectionType-Element in konzeptioneller Schemadefinitionssprache (CSDL) gibt an, dass ein Funktionsparameter oder ein Funktionsrückgabetyp eine Auflistung ist. Das CollectionType-Element kann ein untergeordnetes Element des Parameter-Elements oder des ReturnType (Function)-Elements sein.

## ComplexType

Ein ComplexType-Element definiert eine Datenstruktur, die aus EdmSimpleType-Eigenschaften oder anderen komplexen Typen besteht.  Ein komplexer Typ kann eine Eigenschaft eines Entitätstyps oder eines anderen komplexen Typs sein. Ein komplexer Typ entspricht einem Entitätstyp, in dem von einem komplexen Typ Daten definiert werden. Es gibt jedoch einige Hauptunterschiede zwischen komplexen Typen und Entitätstypen:

Komplexe Typen weisen keine Identitäten (oder Schlüssel) auf und können daher nicht unabhängig sein. Komplexe Typen können nur Eigenschaften von Entitätstypen oder anderen komplexen Typen sein.
Komplexe Typen können nicht Teil von Zuordnungen sein. Die Enden einer Zuordnung können kein komplexer Typ sein, daher können Navigationseigenschaften nicht für komplexe Typen definiert werden.
Einer komplexen Typeigenschaft kann kein NULL-Wert zugewiesen werden, obwohl jede skalare Eigenschaft eines komplexen Typs auf NULL festgelegt werden kann.

## DefineExpression

Das DefineExpression-Element in der konzeptionellen Schemadefinitionssprache (CSDL) enthält einen Entity SQL-Ausdruck, der eine Funktion im konzeptionellen Modell definiert.  

```
<Function Name="GetYearsInPrint" ReturnType="Edm.Int32" >
       <Parameter Name="book" Type="BooksModel.Book" />
       <DefiningExpression>
         Year(CurrentDateTime()) - Year(cast(book.PublishedDate as DateTime))
       </DefiningExpression>
     </Function>
```

## Dependent

Das Dependent-Element in konzeptioneller Schemadefinitionssprache (CSDL) ist ein dem ReferentialConstraint-Element untergeordnetes Element und definiert das abhängige Ende einer referenziellen Einschränkung. Ein ReferentialConstraint-Element definiert Funktionen, die einer Einschränkung der referenziellen Integrität in einer relationalen Datenbank ähnlich sind. So wie eine Spalte (oder Spalten) einer Datenbanktabelle auf den Primärschlüssel einer anderen Tabelle verweisen kann, kann eine Eigenschaft (oder Eigenschaften) eines Entitätstyps auf den Entitätsschlüssel eines anderen Entitätstyps verweisen. Der Entitätstyp, auf den verwiesen wird, wird als Prinzipalende der Einschränkung bezeichnet. Der Entitätstyp, der auf das Prinzipalende verweist, wird als abhängiges Ende der Einschränkung bezeichnet. PropertyRef-Elemente werden verwendet, um anzugeben, welche Schlüssel auf das Prinzipalende verweisen.

## Documentation

Das Documentation-Element in konzeptioneller Schemadefinitionssprache (CSDL) kann verwendet werden, um Informationen zu einem Objekt bereitzustellen, das in einem übergeordneten Element definiert ist. Ist in einer EDMX-Datei das Documentation-Element ein untergeordnetes Element eines anderen Elements, das als Objekt auf der Entwurfsoberfläche von EF Designer (z. B. als Entität, Zuordnung oder Eigenschaft) angezeigt wird, wird der Inhalt des Documentation-Elements im Properties-Fenster von Visual Studio für das Objekt angezeigt.

## End

Das End-Element kann in konzeptioneller Schemadefinitionssprache (CSDL) ein untergeordnetes Element des Association-Elements oder des AssociationSet-Elements sein. In beiden Fällen unterscheiden sich die Rolle und die anwendbaren Attribute des End-Elements.

Ein End-Element (als untergeordnetes Element des Association-Elements) identifiziert an einem Ende einer Zuordnung den Entitätstyp und die Anzahl der Entitätstypinstanzen, die an diesem Ende einer Zuordnung existieren können. Zuordnungsenden werden als Teil einer Zuordnung definiert. Eine Zuordnung muss genau zwei Zuordnungsenden aufweisen. Auf Entitätstypinstanzen an einem Ende einer Zuordnung kann über Navigationseigenschaften oder Fremdschlüssel zugegriffen werden, sofern sie für einen Entitätstyp verfügbar gemacht werden.  

Das End-Element gibt ein Ende des Zuordnungssatzes an. Das AssociationSet-Element muss zwei End-Elemente enthalten. Die in einem End-Element enthaltenen Informationen werden beim Zuordnen eines Zuordnungssatzes zu einer Datenquelle verwendet.

## EntityContainer

Das EntityContainer-Element in konzeptioneller Schemadefinitionssprache (CSDL) ist ein logischer Container für Entitätssätze, Zuordnungssätze und Funktionsimporte. Durch das EntityContainerMapping-Element wird einem Speichermodell-Entitätscontainer ein konzeptioneller Modellentitätscontainer zugeordnet. Ein Speichermodell-Entitätscontainer beschreibt die Struktur der Datenbank: Entitätssätze beschreiben Tabellen, Zuordnungssätze beschreiben Fremdschlüsseleinschränkungen, und Funktionsimporte beschreiben gespeicherte Prozeduren in einer Datenbank.

Ein EntityContainer-Element kann kein oder ein Dokumentationselement aufweisen. Wenn ein Documentation-Element vorhanden ist, muss es allen EntitySet-, AssociationSet- und FunctionImport-Elementen vorangestellt sein.

## EntitySet

Das EntitySet-Element in konzeptioneller Schemadefinitionssprache ist ein logischer Container für Instanzen eines Entitätstyps und Instanzen beliebiger von diesem Entitätstyp abgeleiteter Typen. Die Beziehung zwischen einem Entitätstyp und einem Entitätssatz ist zur Beziehung zwischen einer Zeile und einer Tabelle in einer relationalen Datenbank analog. Wie eine Zeile definiert ein Entitätstyp einen Satz verknüpfter Daten, und ebenso wie eine Tabelle enthält ein Entitätssatz Instanzen dieser Definition. Ein Entitätssatz stellt ein Konstrukt zum Gruppieren von Entitätstypinstanzen bereit, damit diese verwandten Datenstrukturen in einer Datenquelle zugeordnet werden können.  

Für einen bestimmten Entitätstyp kann mindestens ein Entitätssatz definiert werden.

## EntityType

Das EntityType-Element stellt in einem konzeptionellen Modell die Struktur eines Konzepts auf der obersten Ebene dar. Ein Entitätstyp ist eine Vorlage für Instanzen von Entitätstypen in einer Anwendung. Jede Vorlage enthält die folgenden Informationen:

Erforderlich:
- Eine eindeutige Bezeichnung.
- Ein Entitätsschlüssel, der von einem oder mehreren Eigenschaften definiert wird.

Optional:
- Eigenschaften für enthaltene Daten.
- Navigationseigenschaften, die eine Navigation von einem Ende einer Zuordnung zum anderen Ende ermöglichen.

In einer Anwendung stellt eine Instanz eines Entitätstyps ein spezielles Objekt dar, wie etwa einen bestimmten Kunden oder eine Bestellung. Jede Instanz eines Entitätstyps muss über einen eindeutigen Entitätsschlüssel innerhalb einer Entitätsmenge verfügen.

Zwei Instanzen eines Entitätstyps werden nur dann als gleich betrachtet, wenn sie vom selben Typ sind und die Werte ihrer Entitätsschlüssel übereinstimmen.

```xml
<EntityType Name="Book">
  <Key>
    <PropertyRef Name="ISBN" />
  </Key>
  <Property Type="String" Name="ISBN" Nullable="false" />
  <Property Type="String" Name="Title" Nullable="false" />
  <Property Type="Decimal" Name="Revision" Nullable="false" Precision="29" Scale="29" />
  <NavigationProperty Name="Publisher" Relationship="BooksModel.PublishedBy"
                      FromRole="Book" ToRole="Publisher" />
  <NavigationProperty Name="Authors" Relationship="BooksModel.WrittenBy"
                      FromRole="Book" ToRole="Author" />
</EntityType>
```

Mit Vererbung:

```xml
<EntityType Name="FictionBook" BaseType="BooksModel.Book" >
  <Property Type="String" Name="Genre" Nullable="false" />
</EntityType>
```

Ein EntityType kann das Attribut Abstract besitzen, dadurch wird bestimmt, dass dieser Entitätstyp keine Instanzen haben kann.

Das Attribut OpenType bestimmt, dass beliebige, nicht definierte Attribute an einer Instanz dieses Types mitgegeben werden können.



## EnumType

Das EnumType-Element stellt einen Aufzählungstyp dar.

```xml
<EnumType Name="Color" IsFlags=”false” UnderlyingTyp=”Edm.Byte”>
   <Member Name="Red" />
   <Member Name="Green" />
   <Member Name="Blue" />
 </EntityType>
```

## FunctionImport

Das FunctionImport-Element in konzeptioneller Schemadefinitionssprache (CSDL) stellt eine Funktion dar, die in der Datenquelle definiert wird, aber durch das konzeptionelle Modell für andere Objekte verfügbar ist. Ein Function-Element im Speichermodell kann z. B. verwendet werden, um eine gespeicherte Prozedur in einer Datenbank darzustellen. Ein FunctionImport-Element im konzeptionellen Modell stellt die entsprechende Funktion in einer Entity Framework-Anwendung dar und wird der Speichermodellfunktion mithilfe des FunctionImportMapping-Elements zugeordnet. Wird die Funktion in der Anwendung aufgerufen, wird die entsprechende gespeicherte Prozedur in der Datenbank ausgeführt.

Ein FunctionImport-Element kann die folgenden untergeordneten Elemente aufweisen (der vorliegenden Reihenfolge entsprechend):

Dokumentation (kein oder ein Element zugelassen)
Parameter (kein oder mehrere Elemente zugelassen)
Anmerkungselemente (kein oder mehrere Elemente zugelassen)
ReturnType (FunctionImport) (kein oder mehr zulässige Elemente)
Für jeden von der Funktion akzeptierten Parameter sollte ein Parameter-Element definiert werden.

Ein Rückgabetyp für eine Funktion muss entweder mit dem ReturnType-(FunctionImport)-Element oder dem ReturnType-Attribut (siehe unten) angegeben werden, aber nicht mit beiden. Der Rückgabetypwert muss eine Auflistung vom EdmSimpleType, EntityType oder ComplexType sein.

## Key

Das Key-Element ist ein untergeordnetes Element des EntityType-Elements und definiert einen Entitätsschlüssel (eine Eigenschaft oder eine Gruppe von Eigenschaften eines Entitätstyps, der die Identität bestimmt). Die Eigenschaften, die einen Entitätsschlüssel bilden, werden zur Entwurfszeit ausgewählt. Die Werte von Entitätsschlüsseleigenschaften müssen zur Laufzeit eine Entitätstypinstanz innerhalb einer Entitätenmenge eindeutig identifizieren. Die Eigenschaften, die einen Entitätsschlüssel bilden, sollten so ausgewählt werden, dass die Eindeutigkeit von Instanzen in einem Entitätssatz gewährleistet ist. Das Key-Element definiert einen Entitätsschlüssel, indem es auf mindestens eine der Eigenschaften eines Entitätstyps verweist.

## Member

Das Member-Element ist ein untergeordnetes Element des EnumType-Elements und definiert ein Element des Enumerationstyps.

## NavigationProperty

Ein NavigationProperty-Element definiert eine Navigationseigenschaft, die einen Verweis auf das andere Ende einer Zuordnung bereitstellt. Im Gegensatz zu mit dem Property-Element definierten Eigenschaften werden von Navigationseigenschaften Form und Eigenschaften von Daten nicht definiert. Sie bieten eine Möglichkeit, eine Zuordnung zwischen zwei Entitätstypen zu navigieren.

Beachten Sie, dass Navigationseigenschaften für beide Entitätstypen an den Enden einer Zuordnung optional sind. Wenn Sie für einen Entitätstyp am Ende einer Zuordnung eine Navigationseigenschaft definieren, muss keine Navigationseigenschaft für den Entitätstyp am anderen Ende der Zuordnung definiert werden.

Der von einer Navigationseigenschaft zurückgegebene Datentyp wird von der Multiplizität des Remotezuordnungsendes bestimmt. Angenommen, eine Navigationseigenschaft, OrdersNavProp, ist für den Entitätstyp Kunde vorhanden und navigiert in einer 1:n-Zuordnung zwischen Kunde und Bestellung. Da das Remotezuordnungsende für die Navigationseigenschaft eine Multiplizität von n (*) aufweist, ist sein Datentyp eine Auflistung (Order). Wenn eine Navigationseigenschaft CustomerNavProp für den Entitätstyp Order vorhanden ist, wäre der Datentyp Kunde, da die Multiplikation des Remote-Endes eins (1) ist.

Ein NavigationProperty setzt eine Relationship voraus.

Attribute:
- Name
- Relationship
- ToRole
- FromRole
- ContainsTarget, wodurch die Zielstruktur in die Quellstruktur eingebettet wird.


## OnDelete

Das OnDelete-Element in konzeptioneller Schemadefinitionssprache (CSDL) definiert mit einer Zuordnung verbundenes Verhalten. Wenn das Action-Attribut an einem Ende einer Zuordnung auf Cascade festgelegt wird, werden verwandte Entitätstypen am anderen Ende der Zuordnung gelöscht, sobald der Entitätstyp am ersten Ende gelöscht wird. Ist die Zuordnung zwei Entitätstypen eine Primärschlüssel-zu-Primärschlüssel-Beziehung, wird ein geladenes abhängiges Objekt gelöscht, sobald das Prinzipalobjekt am anderen Ende der Zuordnung gelöscht wird. Dies ist unabhängig von der OnDelete-Spezifikation.  

## Parameter

Das Parameter-Element in konzeptioneller Schemadefinitionssprache (CSDL) kann ein untergeordnetes Element des FunctionImport-Elements oder des Function-Elements sein.

Ein Parameter-Element (als untergeordnetes Element des FunctionImport-Elements) wird verwendet, um Eingabe- und Ausgabeparameter für Funktionsimporte zu definieren, die in CSDL deklariert werden.

Ein Parameter-Element (als untergeordnetes Element des Function-Elements) definiert Parameter für Funktionen, die definiert oder in einem konzeptionellen Modell deklariert sind.

## Principal

Das Principal-Element in konzeptioneller Schemadefinitionssprache (CSDL) ist ein dem ReferentialConstraint-Element untergeordnetes Element, welches das Prinzipalende einer referenziellen Einschränkung definiert. Ein ReferentialConstraint-Element definiert Funktionen, die einer Einschränkung der referenziellen Integrität in einer relationalen Datenbank ähnlich sind. So wie eine Spalte (oder Spalten) einer Datenbanktabelle auf den Primärschlüssel einer anderen Tabelle verweisen kann, kann eine Eigenschaft (oder Eigenschaften) eines Entitätstyps auf den Entitätsschlüssel eines anderen Entitätstyps verweisen. Der Entitätstyp, auf den verwiesen wird, wird als Prinzipalende der Einschränkung bezeichnet. Der Entitätstyp, der auf das Prinzipalende verweist, wird als abhängiges Ende der Einschränkung bezeichnet. PropertyRef-Elemente werden zur Angabe der Schlüssel verwendet, auf die vom abhängigen Ende verwiesen wird.

## Property

Das Property-Element in konzeptioneller Schemadefinitionssprache (CSDL) kann ein untergeordnetes Element des EntityType-Elements, des ComplexType-Elements oder des RowType-Elements sein.

Property-Elemente (als untergeordnete Elemente des EntityType- oder ComplexType-Elements) definieren die Form und die Eigenschaften der Daten, die eine Entitätstypinstanz oder eine komplexe Typinstanz enthält. Eigenschaften in einem konzeptionellen Modell sind analog zu den Eigenschaften, die für eine Klasse definiert werden. So wie Eigenschaften die Form einer Klasse definieren und Informationen zu Objekten enthalten definieren Eigenschaften in einem konzeptionellen Modell die Form eines Entitätstyps und enthalten Informationen zu Entitätstypinstanzen.

Property-Elemente (als untergeordnete Elemente eines RowType-Elements) definieren die Form und die Eigenschaften der Daten, die an eine modelldefinierte Funktion übergeben oder von einer modelldefinierten Funktion zurückgegeben werden können.  

## PropertyRef

Das PropertyRef-Element in konzeptioneller Schemadefinitionssprache (CSDL) verweist auf eine Eigenschaft eines Entitätstyps. So wird angegeben, dass die Eigenschaft eine der folgenden Rollen ausführt:

Ein Teil des Entitätsschlüssels (eine Eigenschaft oder ein Satz von Eigenschaften eines Entitätstyps, der die Identität bestimmt). Ein oder mehrere PropertyRef-Elemente können zum Definieren eines Entitätsschlüssels verwendet werden.
Das abhängige Ende oder Prinzipalende einer referenziellen Einschränkung.

## ReferenceType

Mit dem ReferenceType-Element in konzeptioneller Schemadefinitionssprache (CSDL) wird ein Verweis auf einen Entitätstyp angegeben. Das ReferenceType-Element kann ein untergeordnetes Element der folgenden Elemente sein:

ReturnType (Funktion)
Parameter
CollectionType
Das ReferenceType-Element wird beim Definieren eines Parameters oder eines Rückgabetyps für eine Funktion verwendet.

## ReferentialConstraint

Ein ReferentialConstraint-Element in konzeptioneller Schemadefinitionssprache (CSDL) definiert die Funktionalität, die einer Einschränkung der referenziellen Integrität in einer relationalen Datenbank ähnlich ist. So wie eine Spalte (oder Spalten) einer Datenbanktabelle auf den Primärschlüssel einer anderen Tabelle verweisen kann, kann eine Eigenschaft (oder Eigenschaften) eines Entitätstyps auf den Entitätsschlüssel eines anderen Entitätstyps verweisen. Der Entitätstyp, auf den verwiesen wird, wird als Prinzipalende der Einschränkung bezeichnet. Der Entitätstyp, der auf das Prinzipalende verweist, wird als abhängiges Ende der Einschränkung bezeichnet.

Wenn ein Fremdschlüssel, der für einen Entitätstyp verfügbar gemacht wird, auf eine Eigenschaft eines anderen Entitätstyps verweist, definiert das ReferentialConstraint-Element eine Zuordnung zwischen den zwei Entitätstypen. Da das ReferentialConstraint-Element Informationen darüber bereitstellt, wie zwei Entitätstypen zueinander in Beziehung stehen, ist kein entsprechendes AssociationSetMapping-Element in der Mapping Specification Language (MSL) erforderlich. Eine Zuordnung zwischen zwei Entitätstypen, für die keine Fremdschlüssel verfügbar ist, muss ein entsprechendes AssociationSetMapping-Element aufweisen, um der Datenquelle Zuordnungsinformationen zuzuordnen.

Wenn ein Fremdschlüssel für einen Entitätstyp nicht verfügbar ist, kann das ReferentialConstraint-Element zwischen dem Entitätstyp und einem anderen Entitätstyp nur eine Primärschlüssel-zu-Primärschlüssel-Einschränkung definieren.
