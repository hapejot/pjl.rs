sap.ui.require(
    [
        'sap/fe/test/JourneyRunner',
        'vp3/test/integration/FirstJourney',
		'vp3/test/integration/pages/TransportsList',
		'vp3/test/integration/pages/TransportsObjectPage',
		'vp3/test/integration/pages/PackingObjectPage'
    ],
    function(JourneyRunner, opaJourney, TransportsList, TransportsObjectPage, PackingObjectPage) {
        'use strict';
        var JourneyRunner = new JourneyRunner({
            // start index.html in web folder
            launchUrl: sap.ui.require.toUrl('vp3') + '/index.html'
        });

       
        JourneyRunner.run(
            {
                pages: { 
					onTheTransportsList: TransportsList,
					onTheTransportsObjectPage: TransportsObjectPage,
					onThePackingObjectPage: PackingObjectPage
                }
            },
            opaJourney.run
        );
    }
);