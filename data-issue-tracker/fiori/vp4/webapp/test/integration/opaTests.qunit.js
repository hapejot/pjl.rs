sap.ui.require(
    [
        'sap/fe/test/JourneyRunner',
        'vp4/test/integration/FirstJourney',
		'vp4/test/integration/pages/ComponentUsageList',
		'vp4/test/integration/pages/ComponentUsageObjectPage'
    ],
    function(JourneyRunner, opaJourney, ComponentUsageList, ComponentUsageObjectPage) {
        'use strict';
        var JourneyRunner = new JourneyRunner({
            // start index.html in web folder
            launchUrl: sap.ui.require.toUrl('vp4') + '/index.html'
        });

       
        JourneyRunner.run(
            {
                pages: { 
					onTheComponentUsageList: ComponentUsageList,
					onTheComponentUsageObjectPage: ComponentUsageObjectPage
                }
            },
            opaJourney.run
        );
    }
);