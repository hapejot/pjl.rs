//@ui5-bundle preview/Component-preload.js
        jQuery.sap.registerPreloadedModules({
        "version":"2.0",
        "modules":{
          "preview/Component.js": function(){sap.ui.define([
    "sap/ui/core/UIComponent",
    "vp5/model/models"
], (UIComponent, models) => {
    "use strict";

    return UIComponent.extend("vp5.Component", {
        metadata: {
            manifest: "json",
            interfaces: [
                "sap.ui.core.IAsyncContentCreation"
            ]
        },

        init() {
            // call the base component's init function
            UIComponent.prototype.init.apply(this, arguments);

            // set the device model
            this.setModel(models.createDeviceModel(), "device");

            // enable routing
            this.getRouter().initialize();
        }
    });
});
        },
          "preview/manifest.json":{
  "_version": "1.65.0",
  "sap.app": {
    "id": "vp5",
    "type": "application",
    "i18n": "i18n/i18n.properties",
    "applicationVersion": {
      "version": "0.0.1"
    },
    "title": "{{appTitle}}",
    "description": "{{appDescription}}",
    "resources": "resources.json",
    "sourceTemplate": {
      "id": "@sap/generator-fiori:basic",
      "version": "1.17.4",
      "toolsId": "c74a9817-3c1d-4d64-8ecb-6f70e361bccb"
    },
    "dataSources": {
      "mainService": {
        "uri": "/odata/v4/processor/",
        "type": "OData",
        "settings": {
          "annotations": [],
          "odataVersion": "4.0"
        }
      }
    },
    "crossNavigation": {
      "inbounds": {
        "ShippingProject-overview": {
          "semanticObject": "ShippingProject",
          "action": "overview",
          "title": "{{ShippingProject-overview.flpTitle}}",
          "signature": {
            "parameters": {},
            "additionalParameters": "allowed"
          }
        }
      }
    }
  },
  "sap.ui": {
    "technology": "UI5",
    "fullWidth": true,
    "icons": {
      "icon": "",
      "favIcon": "",
      "phone": "",
      "phone@2": "",
      "tablet": "",
      "tablet@2": ""
    },
    "deviceTypes": {
      "desktop": true,
      "tablet": true,
      "phone": true
    }
  },
  "sap.ui5": {
    "flexEnabled": true,
    "dependencies": {
      "minUI5Version": "1.138.1",
      "libs": {
        "sap.m": {},
        "sap.ui.core": {}
      }
    },
    "contentDensities": {
      "compact": true,
      "cozy": true
    },
    "models": {
      "i18n": {
        "type": "sap.ui.model.resource.ResourceModel",
        "settings": {
          "bundleName": "vp5.i18n.i18n"
        }
      },
      "": {
        "dataSource": "mainService",
        "preload": true,
        "settings": {
          "operationMode": "Server",
          "autoExpandSelect": true,
          "earlyRequests": true
        }
      }
    },
    "resources": {
      "css": [
        {
          "uri": "css/style.css"
        }
      ]
    },
    "routing": {
      "config": {
        "routerClass": "sap.m.routing.Router",
        "controlAggregation": "pages",
        "controlId": "app",
        "transition": "slide",
        "type": "View",
        "viewType": "XML",
        "path": "vp5.view",
        "async": true,
        "viewPath": "vp5.view"
      },
      "routes": [
        {
          "name": "RouteVP5",
          "pattern": ":?query:",
          "target": [
            "TargetVP5"
          ]
        }
      ],
      "targets": {
        "TargetVP5": {
          "id": "VP5",
          "name": "VP5"
        }
      }
    },
    "rootView": {
      "viewName": "vp5.view.App",
      "type": "XML",
      "id": "App",
      "async": true
    }
  }
}

        }});