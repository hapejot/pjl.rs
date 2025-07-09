f = function (U, J, C, N, D, a) {
  "use strict";
  return U.extend("hcm.fab.timeeventsapproval.inbox.util.DataProvider", {
    constructor: function () {
      this._eDataType = {
        T: sap.ui.core.format.DateFormat.getTimeInstance({
          pattern: "HH:mm:ss",
        }),
        D: sap.ui.core.format.DateFormat.getDateInstance({ style: "medium" }),
        N: sap.ui.core.format.NumberFormat.getIntegerInstance({
          maxFractionDigits: 0,
          groupingEnabled: true,
        }),
        P: sap.ui.core.format.NumberFormat.getFloatInstance({
          maxFractionDigits: 2,
          groupingEnabled: true,
        }),
      };
    },
    setAllModels: function (v, r) {
      this._setAllModels(v, r);
    },
    _setAllModels: function (v, r) {
      v.setModel(this._getHeaderModel(r), "header");
      v.setModel(this._getTimeEventsModel(r), "time");
      v.setModel(this._getAdditionalFieldsModel(r), "additionalFields");
      this._renderAdditionalFields(v, r);
    },
    _getHeaderModel: function (r) {
      var d = {
        RequestId: r.toRequesterDetails.RequestId,
        MimeType: r.toRequesterDetails.__metadata.media_src,
        EmployeeId: r.toRequesterDetails.EmployeeId,
        EmployeeName: r.toRequesterDetails.EmployeeName,
        WorkMobile: r.toRequesterDetails.WorkMobile,
        WorkPhone: r.toRequesterDetails.WorkPhone,
        WorkEmail: r.toRequesterDetails.WorkEmail,
        CompanyName: r.toRequesterDetails.CompanyName,
        CostCenterText: r.toRequesterDetails.CostCenterText,
        EmployeePositionText: r.toRequesterDetails.EmployeePositionText,
        OfficeAddress: r.toRequesterDetails.OfficeAddress,
        showEmployeePicture: true,
        showEmployeeNumber: true,
        bShowEmployeeNumberWithoutZeros: false,
      };
      var m = new J(d);
      a.getDefaultAssignment("APPROVETIMEEVENTS").then(function (b) {
        m.setProperty("/assignmentId", b.EmployeeId);
        if (b.hasOwnProperty("ShowEmployeePicture")) {
          m.setProperty("/showEmployeePicture", b.ShowEmployeePicture);
          m.setProperty("/showEmployeeNumber", b.ShowEmployeeNumber);
          m.setProperty(
            "/bShowEmployeeNumberWithoutZeros",
            b.ShowEmployeeNumberWithoutZeros,
          );
        }
      });
      return m;
    },
    _getTimeEventsModel: function (r) {
      var d = {
        EmployeeId: r.EmployeeId,
        InstanceId: r.InstanceId,
        RequestId: r.RequestId,
        TimeType: r.TimeType,
        TimeTypeText: r.TimeTypeText,
        EventDate: C.formatterTimestampToDate(r.EventDate),
        EventTime: C.formatterTimestampToTime(r.EventTime),
        Note: r.Note,
        NoteFromApprover: r.NoteFromApprover,
        CancellationStatus: "",
      };
      if (
        r.CancellationStatus !== undefined &&
        r.CancellationStatus !== null &&
        r.CancellationStatus
      ) {
        d.CancellationStatus = r.CancellationStatus;
      }
      return new J(d);
    },
    _getAdditionalFieldsModel: function (r) {
      var d = { AdditionalFieldsCollection: r.toAdditionalFields.results };
      return new J(d);
    },
    _renderAdditionalFields: function (v, r) {
      var f = v.byId("formTimeEvent");
      var F = r.toAdditionalFields.results;
      var t = this;
      if (F.length > 0) {
        for (var i = 0; i < F.length; i++) {
          f.addContent(new sap.m.Label({ text: F[i].FieldLabel }));
          switch (F[i].TypeKind) {
            case "P":
              f.addContent(
                new sap.m.Text({ text: t._eDataType.P.format(F[i].Value) }),
              );
              break;
            case "C":
              f.addContent(new sap.m.Text({ text: F[0].Value }));
              break;
            case "T":
              var h = F[i].Value.substring(0, 2);
              var m = F[i].Value.substring(2, 4);
              var s = F[i].Value.substring(4, 6);
              var d = new Date(null, null, null, h, m, s);
              f.addContent(new sap.m.Text({ text: t._eDataType.T.format(d) }));
              break;
            case "N":
              f.addContent(new sap.m.Text({ text: F[i].Value }));
              break;
            case "D":
              var y = F[i].Value.substring(0, 4);
              var b = parseInt(F[i].Value.substring(4, 6), 10) - 1;
              var c = F[i].Value.substring(6, 8);
              var o = new Date(y, b, c);
              f.addContent(new sap.m.Text({ text: t._eDataType.D.format(o) }));
          }
          f.addContent(new sap.m.Label({}));
        }
      }
    },
  });
};
