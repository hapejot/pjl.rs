sap.ui.define([
	"sap/ui/core/mvc/Controller",
	"sap/base/Log"
], function (Controller, Log) {
	"use strict";

	return Controller.extend("dit.controller.Detail", {
		onInit: function () {
			this.oOwnerComponent = this.getOwnerComponent();

			this.oRouter = this.oOwnerComponent.getRouter();
			this.oModel = this.oOwnerComponent.getModel();

			this.oRouter.getRoute("master").attachPatternMatched(this._onProductMatched, this);
			this.oRouter.getRoute("detail").attachPatternMatched(this._onProductMatched, this);
			this.oRouter.getRoute("detailDetail").attachPatternMatched(this._onProductMatched, this);
		},

		onSupplierPress: function (oEvent) {
			var supplierPath = oEvent.getSource().getBindingContext("svc").getPath(),
				supplier = supplierPath.split("/").slice(-1).pop(),
				oNextUIState;

			this.oOwnerComponent.getHelper().then(function (oHelper) {
				oNextUIState = oHelper.getNextUIState(2);
				this.oRouter.navTo("detailDetail", {
					layout: oNextUIState.layout,
					supplier: supplier,
					product: this._issue
				});
			}.bind(this));
		},

		_onProductMatched: function (oEvent) {
			var param = oEvent.getParameter("arguments");
			this._issue = param.issue || this._issue;
			if (this._issue !== undefined) {
				var view = this.getView();
				Log.info("Detail view bind to issue: " + this._issue);
				view.bindElement({
					path: "/" + this._issue,
					model: "svc"
				});
			}
		},

		onEditToggleButtonPress: function () {
			var oObjectPage = this.getView().byId("objectPage"),
				bCurrentShowFooterState = oObjectPage.getShowFooter();

			oObjectPage.setShowFooter(!bCurrentShowFooterState);
		},

		handleFullScreen: function () {
			var sNextLayout = this.oModel.getProperty("/actionButtonsInfo/midColumn/fullScreen");
			this.oRouter.navTo("detail", { layout: sNextLayout, product: this._issue });
		},

		handleExitFullScreen: function () {
			var sNextLayout = this.oModel.getProperty("/actionButtonsInfo/midColumn/exitFullScreen");
			this.oRouter.navTo("detail", { layout: sNextLayout, product: this._issue });
		},

		handleClose: function () {
			var sNextLayout = this.oModel.getProperty("/actionButtonsInfo/midColumn/closeColumn");
			this.oRouter.navTo("master", { layout: sNextLayout });
		},

		onSave: function () {
			console.log(this.getView().getModel("svc").getPendingChanges());
			this.getView().getModel("svc").submitChanges({
				success: function () {
					Log.info("Changes submitted successfully.");
				},
				error: function (oError) {
					Log.error("Error submitting changes: " + oError.message);
				}
			});
		},
		onNewComment: function () {
			var tbl = this.getView().byId("historyTable");
			var bdg = tbl.getBindingContext("svc");
			var ctx = bdg.create({content: "comment..."});
			// var oModel = this.getView().getModel("svc"),
			// 	oContext = oModel.createEntry(path + "/comments", {
			// 		properties: {
			// 		}
			// 	});

			// this.getView().setBindingContext(oContext, "svc");
			// this.getView().byId("commentTextArea").focus();
		},
		onExit: function () {
			this.oRouter.getRoute("master").detachPatternMatched(this._onProductMatched, this);
			this.oRouter.getRoute("detail").detachPatternMatched(this._onProductMatched, this);
		}
	});
});
