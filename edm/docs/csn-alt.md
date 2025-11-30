# Core Schema Notation (CSN)

CSN (pronounced as \"*Season*\") is a notation for compact
representations of CDS models --- tailored to serve as an optimized
format to share and interpret models with minimal footprint and
dependencies.

It\'s similar to [JSON Schema](https://json-schema.org) but goes beyond
JSON\'s abilities, in order to capture full-blown *Entity-Relationship
Models* and [Extensions](#aspects). This makes CSN models a perfect
source to generate target models, such as
[OData/EDM](./../advanced/odata) or [OpenAPI](https://www.openapis.org)
interfaces, as well as persistence models for SQL or NoSQL databases.

## Anatomy

A CSN model in **JSON**:

``` json
{
  "requires": [ "@sap/cds/common", "./db/schema" ],
  "definitions": {
    "some.type": { "type": "cds.String", "length": 11 },
    "another.type": { "type": "some.type" },
    "structured.type": { "elements": {
      "foo": { "type": "cds.Integer" },
      "bar": { "type": "cds.String" }
    }}
  },
  "extensions": [
    { "extend":"Foo", "elements":{
      "bar": { "type": "cds.String" }
    }}
  ]
}
```

The same model in **YAML**:

``` yaml
requires:
  - @sap/cds/common
  - ./db/schema
definitions:
  some.type: {type: cds.String, length: 11}
  another.type: {type: some.type }
  structured.type:
    elements:
      foo: {type: cds.Integer}
      bar: {type: cds.String}
extensions: [
  - extend: Foo
    elements:
      bar: {type: cds.String}
]
```

The same model as a **plain JavaScript** object:

``` js
({
  requires:[ '@sap/cds/common', './db/schema' ],
  definitions: {
    'some.type': { type:"cds.String", length:11 },
    'another.type': { type:"some.type" },
    'structured.type': { elements: {
      'foo': { type:"cds.Integer" },
      'bar': { type:"cds.String" }
    }}
  },
  extensions: [
    { extend:'Foo', elements:{
      'bar': { type:"cds.String" }
    }
  ],
})
```

For the remainder of this spec, you see examples in yaml representation
with the following **conventions**:

    property:...   // a CSN-specified property name
    'name':...     // a definition's declared name
    value          // a string value, including referred names
    11, true       // number and boolean literal values

#### Properties

-   `requires` -- an array listing [imported models](#imports)
-   `definitions` -- a dictionary of named [definitions](#definitions)
-   `extensions` -- an array of unnamed [aspects](#aspects)
-   `i18n` -- a dictionary of dictionaries of [text translations](#i18n)

All properties are optional

For example, one model could contain a few definitions, while another
one only contains some extensions.

References are case-sensitive

All references in properties like `type` or `target` use exactly the
same notation regarding casing as their targets\' names. To avoid
problems when translating models to case-insensitive environments like
SQL databases, avoid case-significant names and references. For example,
avoid two different definitions in the same scope whose names only
differ in casing, such as `foo` and `Foo`.

## Literals

There are several places where literals can show up in models, such as
in SQL expressions, calculated fields, or annotations.

Standard literals are represented as in JSON:

|Kind           |Example                      
|-------------- |--------------------------   
|Globals        |`true`, `false`, `null`    
|Numbers^1^     |`11` or `2.4`    
|Strings        |`"foo"`    
|Dates^2^       |`"2016-11-24"`   
|Times^2^       |`"16:11Z"`   
|DateTimes^2^   |`"2016-11-24T16:11Z"`    
|Records        |`{"foo":<literal>, ...}`   
|Arrays         |`[<literal>, ...]`   

In addition, CSN specifies these special forms for references,
expressions, and `enum` symbols:

|Kind                   |Example
|---------------------- |-----------------------
|Unparsed Expressions   |`{"=":"foo.bar < 9"}`
|Enum symbols^3^        |`{"#":"asc"}`

#### Remarks

> ^1^ This is as in JSON and shares the same issues when decimals are
> mapped to doubles with potential rounding errors. The same applies to
> Integer64. Use strings to avoid that, if applicable.
>
> ^2^ Also, as in JSON, dates, and times are represented just as strings
> as specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
> consumers are assumed to know the types and handle the values
> correctly.
>
> ^3^ As enum symbols are equal to their values, it frequently suffices
> to just provide them as strings. Similar to time and dates in CSN and
> JSON, the consumers are assumed to know the types and handle the
> values correctly. The `{"#":...}` syntax option is to serve cases
> where you have to distinguish the kind only based on the provided
> value, for example, in untyped annotations.

## Definitions

Each entry in the `definitions` dictionary is essentially a type
definition. The name is the absolute, fully qualified name of the
definition, and the value is a record with the definition details.

#### Example

``` yaml
definitions:
  'Name':     {type:"cds.String"}
  'Currency': {type:"cds.String", length:3}
  'USD':      {type:"Currency"}
  'Amount':
    elements:
      'value':    {type:"cds.Decimal", precision:11, scale:3}
      'currency': {type:"Currency"}
  'SortOrder': {enum: { 'asc': {}, 'desc': {} } }
```

The **name** of a definition is its key in the enclosing dictionary,
like in `definitions` for top-level entries or in `elements` for
structured types and entities.

Names **must**:

-   Be nonempty strings.
-   Neither start, nor end with `.` or `::`.
-   Not contain substrings `..` or \`
-   Not contain the substring `::` more than once.

#### Properties

-   `kind` -- one of `context`, `service`, `entity`, `type`, `action`,
    `function`, or `annotation`
-   `type` -- an optional base type that this definition is derived from
-   `elements` -- optional dictionary of *elements* in case of
    structured types

Property `kind` is always omitted for elements and can be omitted for
top-level type definitions. (PJL: what is an element, what is a
top-level type definition what are the other kinds? why isn't this
introduced earlier in this document?)

These examples are semantically equivalent:

``` js
Foo1 = { type:"cds.String" }
Foo2 = { type:"cds.String", kind:"type" }
```

## Type Definitions

Custom-defined types are entries in [`definitions`](#definitions) with
an optional property `kind`=`"type"` and the following properties.

  Property     Used for
  ------------ ---------------------------------------------------------------------------------------------------------
  `type`       [Scalar Types](#scalar-types), [Structured Types](#structured-types), and [Associations](#associations)
  `elements`   [Structured Types](#structured-types)
  `items`      [Arrayed Types](#arrayed-types)
  `enum`       [Enumeration Types](#enumeration-types)

#### Example

``` yaml
definitions:
  scalar.type:
    type: cds.String
    length: 3
  struct.type:
    elements:
      foo:
        type: cds.Integer
  arrayed.type:
    items:
      type: cds.Integer
  enum.type:
    enum:
      asc: {}
      desc: {}
```

#### Properties

-   `kind` -- omitted or *`"type"`*
-   `type` -- the base type, this definition is derived from
-   [`elements`](#structured-types) -- optional element definitions for
    [*structured types*](#structured-types).
-   [`items`](#arrayed-types) -- optional definition of item types for
    [*arrayed types*](#arrayed-types).
-   [`enum`](#enumeration-types) -- an optional dictionary of enum
    members for [*enumeration types*](#enumeration-types).
-   `value` -- a constant [literal value](#literals) or calculation
    expression
-   `default` -- a default [value or expression](#literals)
-   `localized` *= true* if this type was declared like *foo : localized
    String*
-   `...` -- other type-specific properties, for example, a String\'s
    `length`

### Scalar Type

Scalar types always have property `type` specified, plus optional
type-specific parameter properties.

``` yaml
definitions:
  scalar.type:
    type: cds.String
    length: 3
```

Types always have to be referred to fully qualified. (why?)

    definitions:
      Foo:
        type: cds.Integer
      Bar:
        type: cds.Decimal
        precision: 11
        scale: 3

### Structured Types

Structured types are signified by the presence of an *elements*
property. The value of *elements* is a dictionary of elements. The name
is the local name of the element and the values in turn are Type
Definitions.

The optional property *includes* contains a list of fully qualified
entity-, aspect-, or type-names. Elements, actions, and annotations from
those definitions are then copied into the structured type.

``` yaml
definitions:
  structured.type:
    elements:
      foo:
        type: cds.Integer
      bar:
        type: cds.String
```

### Arrayed Types

Arrayed types are signified by the presence of a property *items*. The
value of which is in turn a [type definition](#type-definitions) that
specifies the arrayed items\' type.

    definitions:
      arrayed.type:
        items:
          type: cds.Integer

### Enumeration Types

The *enum* property is a dictionary of enum member elements with the
name being the enum symbol and the value being a CQN literal value
expression. The literal expression optionally specifies a constant `val`
as a [literal](#literals) plus optional annotations. An enumeration type
can specify an explicit `type` (for example, *Decimal*) but can also
omit it and refer from given enumeration values, or *String* as default.

    definitions:
      Gender:
        enum:
          male: {}
          female: {}
          non_binary:
            val: non-binary
      Status:
        enum:
          submitted:
            val: 1
          fulfilled:
            val: 2
      Rating:
        type: cds.Decimal
        enum:
          low:
            val: 0
          medium:
            val: 50
          high:
            val: 100

**Literal Values**

(PJL Semantics?)

Literal values are represented as {val:...} with property val holding
the actual literal value as specified in JSON.

Examples:

    cds.parse.expr(`'a string'`)  == {val:'a string'}
    cds.parse.expr(`11`)  == {val:11}
    cds.parse.expr(`true`)  == {val:true}
    cds.parse.expr(`null`)  == {val:null}
    cds.parse.expr(`date'2023-04-15'`)  == {val: '2023-04-15', literal: 'date'}
    cds.parse.expr(`time'13:05:23Z'`)  == {val: '13:05:23Z', literal: 'time'}
    cds.parse.expr(`timestamp'2023-04-15T13:05:23Z'`)  == {val: '2023-04-15T13:05:23Z', literal: 'timestamp'}

**References**

(PJL Semantics?)

A reference is represented as { ref: ... } with property ref. This
property holds an array of reference segments as plain identifier
strings. Only in case of infix filters and/or arguments, the property
holds an object { id: 'identifier', ... } and all properties except id
are optional, as shown in the following snippet:

    ref = {ref:[..._segment]}
    _segment = string | { id: string, args: _named, where: _xpr,
                          groupBy: [ ...expr ], having: _xpr,
                          orderBy: [ ...ordering_term ], limit: { rows: expr, offset: expr } }
    _named = { ... <name>:expr }

Examples:

    let cqn4 = cds.parse.expr
    cqn4(`![keyword]`) == {ref:['keyword']}
    cqn4(`foo.bar`) == {ref:['foo','bar']}
    cqn4(`foo[9].bar`) == {ref:[{ id:'foo', where:[{val:9}] }, 'bar' ]}
    cqn4(`foo(p:x).bar`) == {ref:[{ id:'foo', args:{p:{ref:['x']}} }, 'bar' ]}
    cqn4(`foo[where a=1 group by b having b>2 order by c limit 7].bar`)
      == {ref:[{ id:'foo', where:[{ref:['a']}, '=', {val:9}],
                           groupBy: [{ref: ['b']}], having: [{ref: ['b']}, '>',  {val:2}],
                           orderBy: [{ref: ['c']}], limit: {rows: {val: 7}} },
               'bar' ]}

## Entity Definitions

Entities are [structured types](#structured-types) with *kind* =
'entity'. In addition, one or more elements usually have property *key*
set to true, to flag the entity\'s primary key.

#### Example

    definitions:
      Products:
        kind: entity
        elements:
          ID:
            type: cds.Integer
            key: true
          title:
            type: cds.String
            notNull: true
          price:
            type: Amount
            virtual: true

#### Properties

-   *kind* -- is always 'entity'
-   *elements* -- as in [Structured Types](#structured-types),
    optionally equipped with one or more of these boolean properties:
    -   *key* -- signifies that the element is (part of) the primary key
    -   *virtual* -- has this element ignored in generic persistence
        mapping
    -   *notNull* -- the *not null* constraint as in SQL
-   *includes* -- as in [Structured Types](#structured-types)

### View Definitions

Views are entities defined as projections on underlying entities. In
CSN, views are signified by the presence of property *query*, which
captures the projection as a CQN expression.

#### Example

    definitions:
      Foo:
        kind: entity
        query:
          SELECT:
            from:
              ref:
                - Bar
            columns:
              - ref:
                  - title
              - ref:
                  - price

#### Properties

-   *kind* -- mandatory; always 'entity'
-   *query* -- the parsed query in CQN format
-   *elements* -- optional elements signature, omitted and inferred
-   *params* -- optional parameters

### Views with Declared Signatures

Views with declared signatures have the additional property *elements*
filled in as in [entities](./cdl#entities):

    definitions:
      with.declared.signature:
        kind: entity
        elements:
          title:
            type: cds.String
          price:
            type: Amount
        query:
          SELECT: ...

### Viewswith Parameters [​](#views-with-parameters)

Views with parameters have an additional property `params` -- an
optional dictionary of parameter [type definitions](#type-definitions):

    definitions:
      with.params:
        kind: entity
        params:
          ID:
            type: cds.Integer
        query:
          SELECT:
            expr: ...

### Projetions [​](#projections)

Use the `projection` property for views if you don\'t need the full
power of SQL. See `as projection on` in [CDL](./cdl#as-projection-on)
for restrictions.

    definitions:
      Foo:
        kind: entity
        projection:
          from:
            ref:
              - Bar
          columns:
            - '*'

#### Properties [](#properties-4)

-   `kind` -- mandatory; always *`"entity"`*
-   `projection` -- the parsed query; equivalent to `query.SELECT`, see
    [CQN](./cqn)
-   `elements` -- optional [elements
    signature](#views-with-declared-signatures), omitted and inferred

## Associations [](#associations)

Associations are like [scalar type definitions](#scalar-types) with
`type` being `cds.Association` or `cds.Composition` plus additional
properties specifying the association\'s `target` and optional
information like `on` conditions or foreign `keys`.

### Basicto-one Associations [​](#basic-to-one-associations)

The basic form of associations are *to-one* associations to a designated
target:

    definitions:
      Books:
        kind: entity
        elements:
          author:
            type: cds.Association
            target: Authors
      Currency:
        type: cds.Association
        target: Currencies

### With pecified `cardinality` [​](#assoc-card)

Add property `cardinality` to explicitly specify a *to-one* or *to-many*
relationship:

    definitions:
      Authors:
        kind: entity
        elements:
          books:
            type: cds.Association
            target: Books
            cardinality:
              max: '*'

Property `cardinality` is an object `{src?,min?,max}` with\...

-   `src` set to `1` give a hint to database optimizers, that a source
    entity always exists
-   `min` specifying the target\'s minimum cardinality -- default: `0`
-   `max` specifying the target\'s maximum cardinality -- default: `1`

In summary, the default cardinality is *\[0..1\]*, which means *to-one*.

### With pecified `on` Condition [​](#assoc-on)

So-called *unmanaged* associations have an explicitly specified `on`
condition:

    definitions:
      Authors:
        kind: entity
        elements:
          books:
            type: cds.Association
            target: Books
            cardinality:
              max: '*'
            'on':
              - ref:
                  - books
                  - author
              - '='
              - ref:
                  - $self

### With Specified `keys` [](#assoc-keys)

Managed to-one associations automatically use the target\'s designated
primary `key` elements. You can overrule this by explicitly specifying
alternative target properties to be used in the `keys` property:

    definitions:
      Books:
        kind: entity
        elements:
          genre:
            type: cds.Association
            target: Genres
            keys:
              - ref:
                  - category
                as: cat
              - ref:
                  - name

Property `keys` has the format and mechanisms of [CQN
projections](./cqn#select).

## Annotations [](#annotations)

Annotations are represented as properties, prefixed with `@`. This
format applies to type/entity-level annotations as well as to
element-level ones.

#### Example [](#example-4)

    definitions:
      Employees:
        kind: entity
        '@title': Mitarbeiter
        '@readonly': true
        elements:
          firstname:
            type: cds.String
            '@title': Vorname
          surname:
            type: cds.String
            '@title': Nachname

Annotations are used to add custom information to definitions, the
prefixed `@` acts as a protection against conflicts with
built-in/standard properties. They\'re flat lists of key-value pairs,
with keys being fully qualified property names and values being
represented as introduced in the section [Literals and
Expressions](#literals).

## Aspects [](#aspects)

In parsed-only models, the top-level property `extensions` holds an
array of unapplied extensions or annotations (→ see also [Aspects in
CDL](./cdl#aspects)). The entries are of this form:

    ext = { extend|annotate: <name>, <property>: <value>, … }

with:

-   `extend` or `annotate` referring to the definition to be extended or
    annotated
-   `<property>` being the property that should be extended, for
    example, `elements` if an entity should be extended with further
    elements

### Extend with \<named aspect\> [](#extend-with-named-aspect)

The most basic form allows to express an extension of a named definition
with another named definition (→ see [Named
Aspects](./cdl#named-aspects)):

    csn = { extensions:[
      { extend:"TargetDefinition", includes:["NamedAspect"]}
    ]}

### Extend with \<anonymous aspect\> [](#extend-with-anonymous-aspect)

The form `{ extend:<target>, <property>: <value>, … }` allows to add
elements to an existing [struct](#structured-types) definition as well
as to add or override annotations of the target definition:

    csn = { extensions:[

      // extend Foo with @foo { ..., bar: String; }
      {
        extend: "Foo",
        '@foo': true,
        elements: {
          // adds a new element 'bar'
          bar: { type: "cds.String", '@bar': true },
        }
      },

    ]}

### annotate with \<anonymous aspect\> [](#annotate-with-anonymous-aspect)

The form `{ annotate:<target>, <property>: <value>, … }` allows to add
or override annotations of the target definition as well as those of
nested elements:

    csn = {extensions:[

      // annotate Foo with @foo;
      { annotate:"Foo", '@foo':true },

      // annotate Foo with @foo { boo @boo }
      { annotate:"Foo", '@foo':true, elements: {
        // annotates existing element 'boo'
        boo: {'@boo':true },
      }},

    ]}

## Services [](#services)

Services are definitions with *kind =`'service'`*:

    definitions:
      MyOrders:
        kind: service

### Actios / Functions [​](#actions-functions)

Entity definitions (for *bound* actions/functions) can have an
additional property `actions`. The keys of these `actions` are the
(local) names of actions/functions. *Unbound* actions/functions of a
service are represented as top level definitions.

Example:

    definitions:
      OrderService:
        kind: service
      OrderService.Orders:
        kind: entity
        elements: {}
        actions:
          validate:
            kind: function
            returns:
              type: cds.Boolean
      OrderService.cancelOrder:
        kind: action
        params:
          orderID:
            type: cds.Integer
          reason:
            type: cds.String
        returns:
          elements:
            ack:
              enum:
                succeeded: {}
                failed: {}
            msg:
              type: cds.String

#### Proprties [​](#properties-5)

-   `kind` -- either `"action"` or `"function"` as in *OData*
-   `params` -- a dictionary with the values being [Type
    Definitions](#type-definitions)
-   `returns` -- a [Type Definition](#type-definitions) describing the
    response

> Note: The definition of the response can be a reference to a declared
> type or the inline definition of a new (structured) type.

## Import [​](#imports)

The `requires` property lists other models to import definitions from.
It is the CSN equivalent of the CDL [`using` directive](./cdl#using).

#### Examle [​](#example-5)

    requires:
      - '@sap/cds/common'
      - ./db/schema

As in Node.js the filenames are either absolute module names or relative
filenames, starting with `./` or `../`.

## i18n [](#i18n)

A CSN may optionally contain a top-level `i18n` property, which can
contain translated texts. The expected structure is as follows:

    i18n:
      language-key:
        text-key: some string
