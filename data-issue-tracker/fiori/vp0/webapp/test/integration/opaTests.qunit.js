sap.ui.require(
    [
        'sap/fe/test/JourneyRunner',
        'vp0/test/integration/FirstJourney',
		'vp0/test/integration/pages/ShippingProjectsList',
		'vp0/test/integration/pages/ShippingProjectsObjectPage',
		'vp0/test/integration/pages/Project2ComponentsObjectPage'
    ],
    function(JourneyRunner, opaJourney, ShippingProjectsList, ShippingProjectsObjectPage, Project2ComponentsObjectPage) {
        'use strict';
        var JourneyRunner = new JourneyRunner({
            // start index.html in web folder
            launchUrl: sap.ui.require.toUrl('vp0') + '/index.html'
        });

       
        JourneyRunner.run(
            {
                pages: { 
					onTheShippingProjectsList: ShippingProjectsList,
					onTheShippingProjectsObjectPage: ShippingProjectsObjectPage,
					onTheProject2ComponentsObjectPage: Project2ComponentsObjectPage
                }
            },
            opaJourney.run
        );
    }
);