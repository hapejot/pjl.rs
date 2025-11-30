sap.ui.define(['sap/fe/test/ObjectPage'], function(ObjectPage) {
    'use strict';

    var CustomPageDefinitions = {
        actions: {},
        assertions: {}
    };

    return new ObjectPage(
        {
            appId: 'vp0',
            componentId: 'Project2ComponentsObjectPage',
            contextPath: '/ShippingProjects/Components'
        },
        CustomPageDefinitions
    );
});