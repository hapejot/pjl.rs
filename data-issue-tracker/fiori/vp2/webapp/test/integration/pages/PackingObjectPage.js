sap.ui.define(['sap/fe/test/ObjectPage'], function(ObjectPage) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ObjectPage(
        {
            appId: 'inw.vp2.vp2',
            componentId: 'PackingObjectPage',
            contextPath: '/Packing'
        },
        CustomPageDefinitions
    );
});