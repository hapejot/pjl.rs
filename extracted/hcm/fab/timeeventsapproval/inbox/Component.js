f = function (U, D, m, a) {
  "use strict";
  return U.extend("hcm.fab.timeeventsapproval.inbox.Component", {
    metadata: { manifest: "json" },
    init: function () {
      this._dataProvider = new a();
      U.prototype.init.apply(this, arguments);
      this.setModel(m.createDeviceModel(), "device");
      this.getRouter().initialize();
    },
    getContentDensityClass: function () {
      if (!this._sContentDensityClass) {
        if (!sap.ui.Device.support.touch) {
          this._sContentDensityClass = "sapUiSizeCompact";
        } else {
          this._sContentDensityClass = "sapUiSizeCozy";
        }
      }
      return this._sContentDensityClass;
    },
    destroy: function () {
      this._dataProvider.destroy();
      U.prototype.destroy.apply(this, arguments);
    },
  });
};
