using ProcessorService as service from '../../srv/services';
annotate service.Transports with @(
    UI.LineItem : [
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : 'Name',
            @UI.Importance : #High,
        },
    ],
    UI.Facets : [
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'Main Data',
            ID : 'MainData',
            Target : '@UI.FieldGroup#MainData',
        },
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'Packings',
            ID : 'Packings',
            Target : 'Packings/@UI.LineItem#Packings',
        },
    ],
    UI.FieldGroup #MainData : {
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
                Value : TransportType,
                Label : 'TransportType',
            },
        ],
    },
);

annotate service.Packing with @(
    UI.LineItem #Packings : [
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : 'Name',
        },
        {
            $Type : 'UI.DataField',
            Value : PackingType,
            Label : 'PackingType',
        },
    ]
);

