sap.ui.define([
    "sap/ui/core/mvc/Controller"
], (Controller) => {
    "use strict";

    return Controller.extend("vp5.controller.VP5", {
        onInit() {
        },

        onWBSLinkPress: function (oEvent) {
            var context = oEvent.getSource().getBindingContext();
            var wbs_id = context.getProperty("WBSID/ID");

            sap.ui.require(["sap/ushell/Container"], async function (Container) {
                const nav = await Container.getServiceAsync("Navigation");
                const intent = {
                    target: {
                        semanticObject: "WBSElement",
                        action: "detail"
                    },
                    params: {
                        ID: wbs_id
                    }
                };
                nav.navigate(intent);
            });
        }
    });
});