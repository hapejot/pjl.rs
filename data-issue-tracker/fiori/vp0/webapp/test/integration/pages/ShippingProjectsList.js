sap.ui.define(['sap/fe/test/ListReport'], function(ListReport) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ListReport(
        {
            appId: 'vp0',
            componentId: 'ShippingProjectsList',
            contextPath: '/ShippingProjects'
        },
        CustomPageDefinitions
    );
});