using ProcessorService as service from '../../srv/services';
annotate service.Components with @(
    UI.LineItem : [
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : 'Name',
            @UI.Importance : #High,
        },
        {
            $Type : 'UI.DataField',
            Value : Description,
            Label : 'Description',
            @UI.Importance : #Low,
        },
    ],
    UI.Facets : [
        {
            $Type : 'UI.ReferenceFacet',
            Label : 'FS1',
            ID : 'FS1',
            Target : '@UI.FieldGroup#FS1',
        },
    ],
    UI.FieldGroup #FS1 : {
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
                Value : Project_ID,
                Label : 'Project_ID',
                Criticality : Project.Status.criticality,
            },
            {
                $Type : 'UI.DataField',
                Value : ItemCategroy,
                Label : 'ItemCategroy',
            },
            {
                $Type : 'UI.DataField',
                Value : Plant,
                Label : 'Plant',
            },
            {
                $Type : 'UI.DataField',
                Value : Quantity,
                Label : 'Quantity',
            },
            {
                $Type : 'UI.DataField',
                Value : QuantityUnit,
                Label : 'QuantityUnit',
            },
            {
                $Type : 'UI.DataField',
                Value : RequestedDate,
                Label : 'RequestedDate',
            },
            {
                $Type : 'UI.DataField',
                Value : StatusLine,
                Label : 'StatusLine',
            },
        ],
    },
    UI.FieldGroup #FSS1 : {
        $Type : 'UI.FieldGroupType',
        Data : [
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
    UI.HeaderInfo : {
        TypeName : 'Component',
        TypeNamePlural : 'Components',
        Title : {
            $Type : 'UI.DataField',
            Value : Name,
        },
        Description : {
            $Type : 'UI.DataField',
            Value : 'Komponente',
        },
    },
);

annotate service.Components with {
    Description @UI.MultiLineText : true
};

annotate service.Packing with {
    ID @(
        Common.Text : Name,
        Common.Text.@UI.TextArrangement : #TextOnly,
)};

annotate service.Components with {
    Project @(
        Common.Text : Project.Title,
        Common.Text.@UI.TextArrangement : #TextOnly,
        Common.ValueList : {
            $Type : 'Common.ValueListType',
            CollectionPath : 'ShippingProjects',
            Parameters : [
                {
                    $Type : 'Common.ValueListParameterInOut',
                    LocalDataProperty : Project_ID,
                    ValueListProperty : 'ID',
                },
            ],
        },
        Common.ValueListWithFixedValues : true,
    )
};
annotate service.Components with {
    ActivityID @(
        Common.Text : ActivityID.Name,
        Common.Text.@UI.TextArrangement : #TextOnly,
        Common.ValueList : {
            $Type : 'Common.ValueListType',
            CollectionPath : 'Activities',
            Parameters : [
                {
                    $Type : 'Common.ValueListParameterInOut',
                    LocalDataProperty : ActivityID_ID,
                    ValueListProperty : 'ID',
                },
            ],
        },
        Common.ValueListWithFixedValues : true,
    )
}

annotate service.ShippingProjects with {
    ID @(
        Common.Text : Title,
        Common.Text.@UI.TextArrangement : #TextOnly,
)};

