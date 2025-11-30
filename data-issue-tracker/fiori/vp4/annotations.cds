using ProcessorService as service from '../../srv/services';

annotate service.ComponentsUnpacked with @(
    UI.LineItem : [
        {
            $Type : 'UI.DataField',
            Value : Quantity,
            Label : 'Quantity',
        },
        {
            $Type : 'UI.DataField',
            Value : UsedQuantity,
            Label : 'UsedQuantity',
        },
        {
            $Type : 'UI.DataField',
            Value : Component.Title,
            Label : 'Title',
        },
        {
            $Type : 'UI.DataField',
            Value : Component.StatusLine,
            Label : 'StatusLine',
        },
    ]
);

