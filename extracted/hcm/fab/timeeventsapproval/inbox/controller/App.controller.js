f = function (C, M, J, R) {
  return C.extend("hcm.fab.timeeventsapproval.inbox.controller.App", {
    onInit: function () {
      this.getView().addStyleClass(
        this.getOwnerComponent().getContentDensityClass(),
      );
    },
  });
};
