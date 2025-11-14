sap.ui.require(
    [
        'sap/fe/test/JourneyRunner',
        'wbs/test/integration/FirstJourney',
		'wbs/test/integration/pages/WBSElementsList',
		'wbs/test/integration/pages/WBSElementsObjectPage'
    ],
    function(JourneyRunner, opaJourney, WBSElementsList, WBSElementsObjectPage) {
        'use strict';
        var JourneyRunner = new JourneyRunner({
            // start index.html in web folder
            launchUrl: sap.ui.require.toUrl('wbs') + '/index.html'
        });

       
        JourneyRunner.run(
            {
                pages: { 
					onTheWBSElementsList: WBSElementsList,
					onTheWBSElementsObjectPage: WBSElementsObjectPage
                }
            },
            opaJourney.run
        );
    }
);