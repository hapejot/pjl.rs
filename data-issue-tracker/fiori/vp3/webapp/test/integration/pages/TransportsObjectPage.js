sap.ui.define(['sap/fe/test/ObjectPage'], function(ObjectPage) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ObjectPage(
        {
            appId: 'vp3',
            componentId: 'TransportsObjectPage',
            contextPath: '/Transports'
        },
        CustomPageDefinitions
    );
});