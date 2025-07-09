f = function (C, J, a, M) {
  "use strict";
  return C.extend("hcm.fab.timeeventsapproval.inbox.controller.S3", {
    Conversions: a,
    extHookHandleHeader: null,
    extHookHandleInfo: null,
    onInit: function () {
      this.resourceBundle = this.getOwnerComponent()
        .getModel("i18n")
        .getResourceBundle();
      this.oRouter = this.getOwnerComponent().getRouter();
      this.oDataModel = this.getOwnerComponent().getModel();
      this._bMessageOpen = false;
      this.instanceIdFliter = [];
      this.oObjectHeader = this.getView().byId("S3_DETAIL_HEADER");
      this.oObjectInfo = this.getView().byId("S3_DETAIL_INFO");
      this.oView.addEventDelegate({
        onBeforeHide: function (e) {
          if (this.oBusyDialog) {
            this.oBusyDialog.close();
          }
        },
      });
      this.oRouter.attachRouteMatched(function (e) {
        var i = e.getParameter("arguments").InstanceID;
        if (i) {
          this.instanceIdFliter.push(
            new sap.ui.model.Filter(
              "InstanceId",
              sap.ui.model.FilterOperator.EQ,
              i,
            ),
          );
          if (this.endswith(i, ":")) {
            i = i.substring(0, i.length - 1);
          }
          this.oDataModel.read("/TimeEventSet", {
            urlParameters: { $expand: "toAdditionalFields,toRequesterDetails" },
            filters: this.instanceIdFliter,
            success: this._handleSuccess.bind(this),
            error: this._handleFailure.bind(this),
          });
        }
      }, this);
      this.oBusyDialog = new sap.m.BusyDialog();
      this.getView().addDependent(this.oBusyDialog);
      this.oBusyDialog.open();
      if (jQuery.sap.getUriParameters().get("responderOn") === "true") {
        this.instanceIdFliter.push(
          new sap.ui.model.Filter(
            "InstanceId",
            sap.ui.model.FilterOperator.EQ,
            "000000427123",
          ),
        );
        this.oDataModel.read("/TimeEventSet", {
          urlParameters: { $expand: "toAdditionalFields,toRequesterDetails" },
          filters: this.instanceIdFliter,
          success: this._handleSuccess.bind(this),
          error: this._handleFailure.bind(this),
        });
      }
    },
    startswith: function (t, p) {
      return t.indexOf(p) === 0;
    },
    endswith: function (t, s) {
      return t.indexOf(s, t.length - s.length) !== -1;
    },
    _handleSuccess: function (d) {
      this.getOwnerComponent()._dataProvider.setAllModels(
        this.getView(),
        d.results[0],
      );
      if (
        this.getView().getModel("time").getData().CancellationStatus !==
          undefined &&
        this.getView().getModel("time").getData().CancellationStatus !== null &&
        this.getView().getModel("time").getData().CancellationStatus
      ) {
        this.byId("S3_CANCELLATION").setText(
          this.getView().getModel("time").getData().CancellationStatus,
        );
      }
      if (this.extHookHandleHeader) {
        this.extHookHandleHeader(this.oObjectHeader);
      }
      if (this.extHookHandleInfo) {
        this.extHookHandleInfo(this.oObjectInfo);
      }
      this.oBusyDialog.close();
    },
    _handleFailure: function (e) {
      this.oBusyDialog.close();
      this._showServiceError(e.responseText);
    },
    onEmailClick: function (e) {
      var E = e.getSource().getProperty("text");
      var s = "";
      sap.m.URLHelper.triggerEmail(E, s);
    },
    onPhoneClick: function (e) {
      var p = e.getSource().getProperty("text");
      sap.m.URLHelper.triggerTel(p);
    },
    onExit: function () {
      this.destroy();
      if (this.oBusyDialog) {
        this.oBusyDialog.close();
      }
    },
    _showServiceError: function (d) {
      if (this._bMessageOpen) {
        return;
      }
      this._bMessageOpen = true;
      M.error(this._sErrorText, {
        details: d,
        styleClass: this._oComponent.getContentDensityClass(),
        actions: [M.Action.CLOSE],
        onClose: function () {
          this._bMessageOpen = false;
        }.bind(this),
      });
    },
  });
};
