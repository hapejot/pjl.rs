sap.ui.define([
    "sap/m/MessageToast",
    "sap/base/Log"
], function (MessageToast, Log) {
    'use strict';

    return {

        packComponents: function (a, components, c) {

            var oNewEntity = {
                Name: "New Packings",
                Description: "This is a new packing"
            };

            // Define the success and error callbacks
            var fnSuccess = function (oData) {
                MessageToast.show("Entity created successfully!");
            };
            var fnError = function (oError) {
                MessageToast.show("Error creating entity.");
            };


            MessageToast.show("Custom handler invoked.");
            if (components.length > 0) {
                Log.setLevel(Log.Level.TRACE);
                Log.trace("creating a new Packing");
                const m = this.getModel();
                const bind = m.bindList("/Packing");
                let subcomps = [];
                for (let i = 0; i < components.length; i++) {
                    subcomps.push({
                        "Component_ID": components[i].getProperty("ID"),
                        "Quantity": 1,
                    });
                }
                const p = bind.create({
                    Name: "New Packing",
                    Description: "This is a new packing",
                    Components: subcomps
                });

                // m.submitBatch("wurst");
            }
        }
    }
});
