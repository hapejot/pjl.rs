//@ui5-bundle preview/Component-preload.js
        jQuery.sap.registerPreloadedModules({
        "version":"2.0",
        "modules":{
          "preview/Component.js": function(){sap.ui.define(
    ["sap/fe/core/AppComponent"],
    function (Component) {
        "use strict";

        return Component.extend("vp0.Component", {
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
    "id": "vp0",
    "type": "application",
    "i18n": "i18n/i18n.properties",
    "applicationVersion": {
      "version": "0.0.1"
    },
    "title": "{{appTitle}}",
    "description": "{{appDescription}}",
    "resources": "resources.json",
    "sourceTemplate": {
      "id": "@sap/generator-fiori:worklist",
      "version": "1.17.4",
      "toolsId": "ad0e239d-e2ce-4d8a-9387-ea4c808c2b5e"
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
        "ShippingProject-list": {
          "semanticObject": "ShippingProject",
          "action": "list",
          "title": "{{ShippingProject-list.flpTitle}}",
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
          "bundleName": "vp0.i18n.i18n"
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
          "defaultTwoColumnLayoutType": "TwoColumnsMidExpanded",
          "defaultThreeColumnLayoutType": "ThreeColumnsEndExpanded"
        },
        "routerClass": "sap.f.routing.Router"
      },
      "routes": [
        {
          "pattern": ":?query:",
          "name": "ShippingProjectsList",
          "target": [
            "ShippingProjectsList"
          ]
        },
        {
          "pattern": "ShippingProjects({key}):?query:",
          "name": "ShippingProjectsObjectPage",
          "target": [
            "ShippingProjectsList",
            "ShippingProjectsObjectPage"
          ]
        }
      ],
      "targets": {
        "ShippingProjectsList": {
          "type": "Component",
          "id": "ShippingProjectsList",
          "name": "sap.fe.templates.ListReport",
          "options": {
            "settings": {
              "contextPath": "/ShippingProjects",
              "hideFilterBar": false,
              "navigation": {
                "ShippingProjects": {
                  "detail": {
                    "route": "ShippingProjectsObjectPage"
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
              "defaultTemplateAnnotationPath": "com.sap.vocabularies.UI.v1.SelectionPresentationVariant#tableView",
              "initialLoad": "Enabled"
            }
          },
          "controlAggregation": "beginColumnPages",
          "contextPattern": ""
        },
        "ShippingProjectsObjectPage": {
          "type": "Component",
          "id": "ShippingProjectsObjectPage",
          "name": "sap.fe.templates.ObjectPage",
          "options": {
            "settings": {
              "editableHeaderContent": false,
              "contextPath": "/ShippingProjects",
              "navigation": {},
              "controlConfiguration": {
                "Components/@com.sap.vocabularies.UI.v1.LineItem#i18nComponents": {
                  "columns": {
                    "DataField::Project::Components::Name": {
                      "width": "100px"
                    },
                    "DataField::Project::Components::Description": {
                      "width": "400px"
                    },
                    "DataField::Activity": {
                      "width": "250px"
                    },
                    "DataField::Item": {
                      "width": "50px"
                    },
                    "DataField::ItemCategroy": {
                      "width": "20px"
                    },
                    "DataField::Plant": {
                      "width": "50px"
                    },
                    "DataField::Quantity": {
                      "width": "100px"
                    },
                    "DataField::QuantityUnit": {
                      "width": "50px"
                    },
                    "DataField::RequestedDate": {
                      "width": "100px"
                    },
                    "DataField::StatusLine": {
                      "width": "100px"
                    }
                  }
                }
              }
            }
          },
          "controlAggregation": "midColumnPages",
          "contextPattern": "/ShippingProjects({key})"
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