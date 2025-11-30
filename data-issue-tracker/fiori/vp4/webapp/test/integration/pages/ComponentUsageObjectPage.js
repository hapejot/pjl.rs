sap.ui.define(['sap/fe/test/ObjectPage'], function(ObjectPage) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ObjectPage(
        {
            appId: 'vp4',
            componentId: 'ComponentUsageObjectPage',
            contextPath: '/ComponentUsage'
        },
        CustomPageDefinitions
    );
});