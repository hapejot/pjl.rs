f = function (J, D) {
  "use strict";
  return {
    createDeviceModel: function () {
      var m = new J(D);
      m.setDefaultBindingMode("OneWay");
      return m;
    },
  };
};
