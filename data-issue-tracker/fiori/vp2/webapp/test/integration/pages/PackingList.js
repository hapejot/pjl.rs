sap.ui.define(['sap/fe/test/ListReport'], function(ListReport) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ListReport(
        {
            appId: 'inw.vp2.vp2',
            componentId: 'PackingList',
            contextPath: '/Packing'
        },
        CustomPageDefinitions
    );
});