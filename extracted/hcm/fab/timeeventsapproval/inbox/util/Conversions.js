f = function (D, L) {
  "use strict";
  return {
    formatterTimestampToDate: function (t) {
      var d, o;
      d = D.getInstance({ style: "medium" });
      if (!t) {
        return "";
      }
      if (typeof t === "string") {
        if (t.indexOf("Date") >= 0) {
          o = this.convertDateStringToDate(t);
        } else {
          o = this.convertTimestampToDate(t);
        }
      } else {
        o = new Date(t);
      }
      o = new Date(o.getUTCFullYear(), o.getUTCMonth(), o.getUTCDate());
      return d.format(o);
    },
    formatterTimestampToTime: function (t) {
      if (t) {
        var l = sap.ui
          .getCore()
          .getConfiguration()
          .getFormatSettings()
          .getFormatLocale();
        var f = L.getInstance(l);
        var a = sap.ui.core.format.DateFormat.getTimeInstance({
          pattern: f.getTimePattern(),
          style: "short",
        });
        var b = new Date(0).getTimezoneOffset() * 60 * 1000;
        var c = a.format(new Date(t.ms + b));
        return c;
      }
    },
    _formatImageURL: function (s, m) {
      if (s && m && typeof m === "string") {
        var u = "",
          l = document.createElement("a"),
          e = function (S) {
            return S.replace(new RegExp("'", "g"), "%27");
          };
        l.href = m;
        u = l.pathname.charAt(0) === "/" ? l.pathname : "/" + l.pathname;
        return e(u);
      }
      return "";
    },
    numberWithoutZeros: function (s, e) {
      var E = s ? parseInt(e, 10).toString() : e;
      return E;
    },
  };
};
