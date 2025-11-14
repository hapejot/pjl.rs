//@ui5-bundle preview/Component-preload.js
        jQuery.sap.registerPreloadedModules({
        "version":"2.0",
        "modules":{
          "preview/Component.js": function(){sap.ui.define(
    ["sap/fe/core/AppComponent"],
    function (Component) {
        "use strict";

        return Component.extend("inw.vp2.vp2.Component", {
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
    "id": "inw.vp2.vp2",
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
      "toolsId": "ea32a6b0-4a86-4c22-955e-79237bc87a5c"
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
        "list": {
          "semanticObject": "Packing",
          "action": "list",
          "title": "{{Packing-list.flpTitle}}",
          "signature": {
            "parameters": {},
            "additionalParameters": "allowed"
          }
        },
        "maintain": {
          "semanticObject": "Packing",
          "action": "maintain",
          "title": "{{Packing-maintain.flpTitle}}",
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
          "bundleName": "inw.vp2.vp2.i18n.i18n"
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
          "name": "PackingList",
          "target": [
            "PackingList"
          ]
        },
        {
          "pattern": "Packing({key}):?query:",
          "name": "PackingObjectPage",
          "target": [
            "PackingObjectPage"
          ]
        },
        {
          "pattern": "Packing({key})/Components({key2}):?query:",
          "name": "ComponentsObjectPage",
          "target": [
            "PackingObjectPage",
            "ComponentsObjectPage"
          ]
        }
      ],
      "targets": {
        "PackingList": {
          "type": "Component",
          "id": "PackingList",
          "name": "sap.fe.templates.ListReport",
          "options": {
            "settings": {
              "contextPath": "/Packing",
              "variantManagement": "Page",
              "navigation": {
                "Packing": {
                  "detail": {
                    "route": "PackingObjectPage"
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
        "PackingObjectPage": {
          "type": "Component",
          "id": "PackingObjectPage",
          "name": "sap.fe.templates.ObjectPage",
          "options": {
            "settings": {
              "editableHeaderContent": false,
              "contextPath": "/Packing",
              "navigation": {
                "Components": {
                  "detail": {
                    "route": "ComponentsObjectPage"
                  }
                }
              },
              "sectionLayout": "Page",
              "additionalSemanticObjects": {
                "Packing": {
                  "allowedActions": [
                    "display",
                    "maintain"
                  ]
                },
                "Component": {
                  "allowedActions": [
                    "display"
                  ]
                }
              },
              "controlConfiguration": {
                "Components/@com.sap.vocabularies.UI.v1.LineItem#Components": {
                  "columns": {
                    "DataField::Component_ID": {
                      "width": "100%"
                    }
                  }
                }
              }
            }
          },
          "controlAggregation": "midColumnPages",
          "contextPattern": "/Packing({key})"
        },
        "ComponentsObjectPage": {
          "type": "Component",
          "id": "ComponentsObjectPage",
          "name": "sap.fe.templates.ObjectPage",
          "options": {
            "settings": {
              "editableHeaderContent": false,
              "contextPath": "/Packing/Components"
            }
          },
          "controlAggregation": "endColumnPages",
          "contextPattern": "/Packing({key})/Components({key2})"
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