using ProcessorService as service from '../../srv/services';
using from '../../db/schema';

annotate service.ShippingProjects with @(
    UI.LineItem : [
        {
            $Type : 'UI.DataField',
            Value : Description,
            Label : '{i18n>Description}',
        },
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : '{i18n>Name}',
        },
        {
            $Type : 'UI.DataField',
            Value : Status_code,
            Label : '{i18n>Status}',
            Criticality : Status.criticality,
        },
    ],
    UI.SelectionPresentationVariant #tableView : {
        $Type : 'UI.SelectionPresentationVariantType',
        PresentationVariant : {
            $Type : 'UI.PresentationVariantType',
            Visualizations : [
                '@UI.LineItem',
            ],
        },
        SelectionVariant : {
            $Type : 'UI.SelectionVariantType',
            SelectOptions : [
            ],
        },
        Text : 'Table View',
    },
    UI.LineItem #tableView : [
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
        {
            $Type : 'UI.DataField',
            Value : Name,
            Label : 'Name',
        },
    ],
    UI.SelectionPresentationVariant #tableView1 : {
        $Type : 'UI.SelectionPresentationVariantType',
        PresentationVariant : {
            $Type : 'UI.PresentationVariantType',
            Visualizations : [
                '@UI.LineItem#tableView',
            ],
        },
        SelectionVariant : {
            $Type : 'UI.SelectionVariantType',
            SelectOptions : [
            ],
        },
        Text : 'Table View 1',
    },
    UI.Facets : [
        {
            $Type : 'UI.ReferenceFacet',
            Label : '{i18n>Main}',
            ID : 'Main',
            Target : '@UI.FieldGroup#Main',
        },
        {
            $Type : 'UI.ReferenceFacet',
            Label : '{i18n>Components}',
            ID : 'i18nComponents',
            Target : 'Components/@UI.LineItem#i18nComponents',
        },
    ],
    UI.FieldGroup #Main : {
        $Type : 'UI.FieldGroupType',
        Data : [
            {
                $Type : 'UI.DataField',
                Value : Name,
                Label : '{i18n>Name}',
            },
            {
                $Type : 'UI.DataField',
                Value : Description,
                Label : '{i18n>Description}',
            },
            {
                $Type : 'UI.DataField',
                Value : Status_code,
                Label : 'Status_code',
            },
        ],
    },
    UI.HeaderInfo : {
        TypeName : '{i18n>ShippingProject}',
        TypeNamePlural : '{i18n>ShippingProjects}',
        Title : {
            $Type : 'UI.DataField',
            Value : Name,
        },
        Description : {
            $Type : 'UI.DataField',
            Value : Description,
        },
    },
);



annotate service.Components with {
    ID @(
        Common.Text : Title,
        Common.Text.@UI.TextArrangement : #TextOnly,
)};

annotate service.ShippingProjects with {
    Status @(
        Common.Text : Status.descr,
        Common.Text.@UI.TextArrangement : #TextFirst,
        Common.ValueListWithFixedValues : true,
    )
};

annotate service.Status with {
    code @Common.Text : descr
};

annotate service.Components with @(
    UI.LineItem #i18nComponents : [
        {
            $Type : 'UI.DataField',
            Value : Project.Components.Name,
            Label : 'Name',
        },
        {
            $Type : 'UI.DataField',
            Value : Project.Components.Description,
            Label : 'Description',
        },
        {
            $Type : 'UI.DataField',
            Value : Activity,
            Label : 'Activity',
        },
        {
            $Type : 'UI.DataField',
            Value : Item,
            Label : 'Item',
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
    Communication.Contact #contact : {
        $Type : 'Communication.ContactType',
        fn : createdBy,
    },
);

annotate service.Status with @(
    UI.DataPoint #criticality : {
        Value : criticality,
        Visualization : #Progress,
        TargetValue : 100,
    }
);

