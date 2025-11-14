sap.ui.require(
    [
        'sap/fe/test/JourneyRunner',
        'vp1/test/integration/FirstJourney',
		'vp1/test/integration/pages/ComponentsList',
		'vp1/test/integration/pages/ComponentsObjectPage'
    ],
    function(JourneyRunner, opaJourney, ComponentsList, ComponentsObjectPage) {
        'use strict';
        var JourneyRunner = new JourneyRunner({
            // start index.html in web folder
            launchUrl: sap.ui.require.toUrl('vp1') + '/index.html'
        });

       
        JourneyRunner.run(
            {
                pages: { 
					onTheComponentsList: ComponentsList,
					onTheComponentsObjectPage: ComponentsObjectPage
                }
            },
            opaJourney.run
        );
    }
);