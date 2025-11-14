sap.ui.define(['sap/fe/test/ListReport'], function(ListReport) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ListReport(
        {
            appId: 'vp4',
            componentId: 'ComponentUsageList',
            contextPath: '/ComponentUsage'
        },
        CustomPageDefinitions
    );
});