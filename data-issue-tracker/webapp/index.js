sap.ui.define([
	"sap/ui/core/mvc/XMLView"
], (XMLView) => {
	"use strict";

	XMLView.create({
		viewName: "data-issue-tracker.App"
	}).then((oView) => oView.placeAt("content"));
});