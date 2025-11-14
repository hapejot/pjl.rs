using ProcessorService as service from '../../srv/services';
annotate service.WBSElements with @(
    UI.LineItem : [
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : 'Name',
        },
        {
            $Type : 'UI.DataField',
            Value : Description,
            Label : 'Description',
        },
    ],
    UI.Facets : [
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'Main',
            ID : 'Main',
            Target : '@UI.FieldGroup#Main',
        },
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'Activities',
            ID : 'Activities',
            Target : 'Activities/@UI.LineItem#Activities',
        },
    ],
    UI.FieldGroup #Main : {
        $Type : 'UI.FieldGroupType',
        Data : [
            {
                $Type : 'UI.DataField',
                Value : Name,
                Label : 'Name',
            },
            {
                $Type : 'UI.DataField',
                Value : Description,
                Label : 'Description',
            },
            {
                $Type : 'UI.DataField',
                Value : createdAt,
            },
            {
                $Type : 'UI.DataField',
                Value : createdBy,
            },
            {
                $Type : 'UI.DataField',
                Value : modifiedAt,
            },
            {
                $Type : 'UI.DataField',
                Value : modifiedBy,
            },
        ],
    },
);

annotate service.Activities with @(
    UI.LineItem #Activities : [
        {
            $Type : 'UI.DataField',
            Value : Activity,
            Label : 'Activity',
        },
        {
            $Type : 'UI.DataField',
            Value : Description,
            Label : 'Description',
        },
        {
            $Type : 'UI.DataField',
            Value : Duration,
            Label : 'Duration',
        },
        {
            $Type : 'UI.DataField',
            Value : DurationUnit,
            Label : 'DurationUnit',
        },
        {
            $Type : 'UI.DataField',
            Value : EarlyStart,
            Label : 'EarlyStart',
            @UI.Importance : #Low,
            
        },
        {
            $Type : 'UI.DataField',
            Value : EarlyFinish,
            Label : 'EarlyFinish',
        },
        {
            $Type : 'UI.DataField',
            Value : LateFinish,
            Label : 'LateFinish',            
        },
        {
            $Type : 'UI.DataField',
            Value : LateStart,
            Label : 'LateStart',
        },
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : 'Name',
        },
        {
            $Type : 'UI.DataField',
            Value : ObjectClass,
            Label : 'ObjectClass',
        },
        {
            $Type : 'UI.DataField',
            Value : Plant,
            Label : 'Plant',
        },
        {
            $Type : 'UI.DataField',
            Value : Components.Material,
            Label : 'Material',
        },
    ]
);

