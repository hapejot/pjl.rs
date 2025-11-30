//@ui5-bundle preview/Component-preload.js
        jQuery.sap.registerPreloadedModules({
        "version":"2.0",
        "modules":{
          "preview/Component.js": function(){sap.ui.define(
    ["sap/fe/core/AppComponent"],
    function (Component) {
        "use strict";

        return Component.extend("vp1.Component", {
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
    "id": "vp1",
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
      "toolsId": "0fc866bd-035e-42c6-b996-b80295ebfd4a"
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
        "Component-manage": {
          "semanticObject": "Component",
          "action": "manage",
          "title": "{{Component-manage.flpTitle}}",
          "indicatorDataSource": {
            "dataSource": "mainService",
            "path": "Components/$count",
            "refresh": 1
          },
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
          "bundleName": "vp1.i18n.i18n"
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
        "type": "View",
        "flexibleColumnLayout": {
          "defaultTwoColumnLayoutType": "TwoColumnsMidExpanded",
          "defaultThreeColumnLayoutType": "ThreeColumnsMidExpanded"
        },
        "routerClass": "sap.f.routing.Router"
      },
      "routes": [
        {
          "pattern": ":?query:",
          "name": "ComponentsList",
          "target": [
            "ComponentsList"
          ]
        },
        {
          "pattern": "Components({key}):?query:",
          "name": "ComponentsObjectPage",
          "target": [
            "ComponentsList",
            "ComponentsObjectPage"
          ]
        }
      ],
      "targets": {
        "ComponentsList": {
          "type": "Component",
          "id": "ComponentsList",
          "name": "sap.fe.templates.ListReport",
          "options": {
            "settings": {
              "contextPath": "/Components",
              "variantManagement": "Page",
              "navigation": {
                "Components": {
                  "detail": {
                    "route": "ComponentsObjectPage"
                  }
                }
              },
              "controlConfiguration": {
                "@com.sap.vocabularies.UI.v1.LineItem": {
                  "tableSettings": {
                    "type": "ResponsiveTable"
                  },
                  "actions": {
                    "pack": {
                      "press": "vp1.ext.controller.ComponentHandler.packComponents",
                      "visible": true,
                      "enabled": true,
                      "requiresSelection": true,
                      "text": "{i18n>pack}"
                    }
                  }
                }
              },
              "initialLoad": "Enabled"
            }
          },
          "controlAggregation": "beginColumnPages",
          "contextPattern": ""
        },
        "ComponentsObjectPage": {
          "type": "Component",
          "id": "ComponentsObjectPage",
          "name": "sap.fe.templates.ObjectPage",
          "options": {
            "settings": {
              "editableHeaderContent": false,
              "contextPath": "/Components",
              "sectionLayout": "Tabs",
              "content": {
                "header": {
                  "anchorBarVisible": true
                }
              }
            }
          },
          "controlAggregation": "midColumnPages",
          "contextPattern": "/Components({key})"
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