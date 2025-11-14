sap.ui.define(['sap/fe/test/ListReport'], function(ListReport) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ListReport(
        {
            appId: 'vp1',
            componentId: 'ComponentsList',
            contextPath: '/Components'
        },
        CustomPageDefinitions
    );
});