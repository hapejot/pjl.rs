using ProcessorService as service from '../../srv/services';
using from '../../db/schema';


annotate service.Packing with @(
    UI.LineItem        : [
        {
            $Type: 'UI.DataField',
            Value: Name,
            Label: 'Name',
        },
        {
            $Type: 'UI.DataField',
            Value: Description,
            Label: 'Description',
        },
        {
            $Type : 'UI.DataField',
            Value : Components.Quantity,
            Label : 'Quantity',
        },
    ],
    UI.HeaderInfo      : {
        TypeName      : 'Packing',
        TypeNamePlural: 'Packings',
        Title         : {
            $Type: 'UI.DataField',
            Value: Name,
        },
    },
    UI.Facets          : [
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'Main',
            ID    : 'Main',
            Target: '@UI.FieldGroup#Main',
        },
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'Components',
            ID : 'Components',
            Target : 'Components/@UI.LineItem#Components',
        },
    ],
    UI.FieldGroup #Main: {
        $Type: 'UI.FieldGroupType',
        Data : [
            {
                $Type: 'UI.DataField',
                Value: Description,
                Label: 'Description',
            },
            {
                $Type: 'UI.DataField',
                Value: Name,
                Label: 'Name',
            },
            {
                $Type: 'UI.DataField',
                Value: TransportID_ID,
                Label: 'TransportID_ID',
            },
            {
                $Type: 'UI.DataField',
                Value: PackingType_ID,
                Label: '{i18n>Packingtypeid}',
            },
        ]
    }
);


annotate service.Packing with {
    TransportID @(
        Common.Text                    : TransportID.Name,
        Common.Text.@UI.TextArrangement: #TextOnly,
        Common.ValueList               : {
            $Type         : 'Common.ValueListType',
            CollectionPath: 'Transports',
            Parameters    : [{
                $Type            : 'Common.ValueListParameterInOut',
                LocalDataProperty: TransportID_ID,
                ValueListProperty: 'ID',
            }, ],
            Label         : 'Transports',
        },
        Common.ValueListWithFixedValues: true,
    )
};

annotate service.Transports with {
    ID @(
        Common.Text                    : Name,
        Common.Text.@UI.TextArrangement: #TextOnly,
    )
};

annotate service.Packing with {
    PackingType @(
        Common.ValueList               : {
            $Type         : 'Common.ValueListType',
            CollectionPath: 'PackingTypes',
            Parameters    : [{
                $Type            : 'Common.ValueListParameterInOut',
                LocalDataProperty: PackingType_ID,
                ValueListProperty: 'ID',
            }, ],
            Label         : 'Packing Types',
        },
        Common.ValueListWithFixedValues: true,
        Common.Text                    : PackingType.Description,
        Common.Text.@UI.TextArrangement: #TextOnly,
        Common.SemanticObject          : 'PackingType',
    )
};

annotate service.PackingTypes with {
    ID @(
        Common.Text                    : Description,
        Common.Text.@UI.TextArrangement: #TextOnly,
    )
};


annotate service.PackingComponents with @(
    UI.LineItem #Components : [
        {
            $Type : 'UI.DataField',
            Value : Quantity,
            Label : 'Quantity',
        },
        {
            $Type : 'UI.DataField',
            Value : Component_ID,
            Label : 'Component_ID',
        },
        {
            $Type : 'UI.DataField',
            Value : Component.Name,
            Label : 'Name',
        },
        {
            $Type : 'UI.DataField',
            Value : Component.Description,
            Label : 'Description',
        },
        {
            $Type : 'UI.DataField',
            Value : Packing.Title,
            Label : 'Title',
        },
    ]
);

annotate service.PackingComponents with {
    Component @(
        Common.Text : Component.Title,
        Common.Text.@UI.TextArrangement : #TextOnly,
        Common.ValueList : {
            $Type : 'Common.ValueListType',
            CollectionPath : 'Components',
            Parameters : [
                {
                    $Type : 'Common.ValueListParameterInOut',
                    LocalDataProperty : Component_ID,
                    ValueListProperty : 'ID',
                },
            ],
        },
        Common.ValueListWithFixedValues : true,
        Common.FieldControl : #Mandatory,
        Common.SemanticObject : 'Component',
        Common.SemanticObjectMapping : [
            {
                $Type : 'Common.SemanticObjectMappingType',
                LocalProperty : Component_ID,
                SemanticObjectProperty : 'ID',
            },
        ],
    )
};

annotate service.Components with {
    Name @Common.FieldControl : #ReadOnly
};

annotate service.Components with {
    Description @Common.FieldControl : #ReadOnly
};

annotate service.Packing with {
    Title @Common.FieldControl : #ReadOnly
};

