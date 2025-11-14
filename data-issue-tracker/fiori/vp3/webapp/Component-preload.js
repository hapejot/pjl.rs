//@ui5-bundle preview/Component-preload.js
        jQuery.sap.registerPreloadedModules({
        "version":"2.0",
        "modules":{
          "preview/Component.js": function(){sap.ui.define(
    ["sap/fe/core/AppComponent"],
    function (Component) {
        "use strict";

        return Component.extend("vp3.Component", {
            metadata: {
                manifest: "json"
            }
        });
    }
);
        },
          "preview/manifest.json":{
  "_version": "1.65.0",
  "sap.app": {
    "id": "vp3",
    "type": "application",
    "i18n": "i18n/i18n.properties",
    "applicationVersion": {
      "version": "0.0.1"
    },
    "title": "{{appTitle}}",
    "description": "{{appDescription}}",
    "resources": "resources.json",
    "sourceTemplate": {
      "id": "@sap/generator-fiori:lrop",
      "version": "1.17.4",
      "toolsId": "83d4823b-fa07-4598-ba22-583b8f9dc35f"
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
        "Transport-manage": {
          "semanticObject": "Transport",
          "action": "manage",
          "title": "{{Transport-manage.flpTitle}}",
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
      "minUI5Version": "1.138.0",
      "libs": {
        "sap.m": {},
        "sap.ui.core": {},
        "sap.fe.templates": {},
        "sap.f": {}
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
          "bundleName": "vp3.i18n.i18n"
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
      },
      "@i18n": {
        "type": "sap.ui.model.resource.ResourceModel",
        "uri": "i18n/i18n.properties"
      }
    },
    "resources": {
      "css": []
    },
    "routing": {
      "config": {
        "flexibleColumnLayout": {
          "defaultTwoColumnLayoutType": "TwoColumnsBeginExpanded",
          "defaultThreeColumnLayoutType": "ThreeColumnsMidExpanded"
        },
        "routerClass": "sap.f.routing.Router"
      },
      "routes": [
        {
          "pattern": ":?query:",
          "name": "TransportsList",
          "target": [
            "TransportsList"
          ]
        },
        {
          "pattern": "Transports({key}):?query:",
          "name": "TransportsObjectPage",
          "target": [
            "TransportsList",
            "TransportsObjectPage"
          ]
        },
        {
          "pattern": "Transports({key})/Packings({key2}):?query:",
          "name": "PackingObjectPage",
          "target": [
            "TransportsList",
            "TransportsObjectPage",
            "PackingObjectPage"
          ]
        }
      ],
      "targets": {
        "TransportsList": {
          "type": "Component",
          "id": "TransportsList",
          "name": "sap.fe.templates.ListReport",
          "options": {
            "settings": {
              "contextPath": "/Transports",
              "variantManagement": "Page",
              "navigation": {
                "Transports": {
                  "detail": {
                    "route": "TransportsObjectPage"
                  }
                }
              },
              "controlConfiguration": {
                "@com.sap.vocabularies.UI.v1.LineItem": {
                  "tableSettings": {
                    "type": "ResponsiveTable"
                  }
                }
              },
              "initialLoad": "Enabled"
            }
          },
          "controlAggregation": "beginColumnPages",
          "contextPattern": ""
        },
        "TransportsObjectPage": {
          "type": "Component",
          "id": "TransportsObjectPage",
          "name": "sap.fe.templates.ObjectPage",
          "options": {
            "settings": {
              "editableHeaderContent": false,
              "contextPath": "/Transports",
              "navigation": {
                "Packings": {
                  "detail": {
                    "route": "PackingObjectPage"
                  }
                }
              },
              "sectionLayout": "Page"
            }
          },
          "controlAggregation": "midColumnPages",
          "contextPattern": "/Transports({key})"
        },
        "PackingObjectPage": {
          "type": "Component",
          "id": "PackingObjectPage",
          "name": "sap.fe.templates.ObjectPage",
          "options": {
            "settings": {
              "editableHeaderContent": false,
              "contextPath": "/Transports/Packings"
            }
          },
          "controlAggregation": "endColumnPages",
          "contextPattern": "/Transports({key})/Packings({key2})"
        }
      }
    },
    "rootView": {
      "viewName": "sap.fe.templates.RootContainer.view.Fcl",
      "type": "XML",
      "async": true,
      "id": "appRootView"
    }
  },
  "sap.fiori": {
    "registrationIds": [],
    "archeType": "transactional"
  }
}

        }});