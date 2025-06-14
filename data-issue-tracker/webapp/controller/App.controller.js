sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/m/MessageToast",
	"sap/ui/model/json/JSONModel" /*,
	"sap/ui/model/odata/v2/ODataModel" */
], (Controller, MessageToast, JSONModel /*, ODataModel */ ) => {
	"use strict";

	return Controller.extend("data-issue-tracker.App", {
		onPress() {
			MessageToast.show("Hello UI5!");
			this.byId("app").to(this.byId("intro"));
		},

		onInit() {
			var m1 = new JSONModel({
					features: [
						"Enterprise-Ready Web Toolkit",
						"Powerful Development Concepts",
						"Feature-Rich UI Controls",
						"Consistent User Experience",
						"Free and Open Source",
						"Responsive Across Browsers and Devices"
					]
				});
			// var m2 = new ODataModel({serviceUrl: "/api/", useBatch: false});
			this.getView().setModel(m1, "features");
		},

		onChange(oEvent) {
			const bState = oEvent.getParameter("state");
			this.byId("ready").setVisible(bState);
		}
	});

});