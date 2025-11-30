sap.ui.require(
    [
        'sap/fe/test/JourneyRunner',
        'inw/vp2/vp2/test/integration/FirstJourney',
		'inw/vp2/vp2/test/integration/pages/PackingList',
		'inw/vp2/vp2/test/integration/pages/PackingObjectPage',
		'inw/vp2/vp2/test/integration/pages/ComponentsObjectPage'
    ],
    function(JourneyRunner, opaJourney, PackingList, PackingObjectPage, ComponentsObjectPage) {
        'use strict';
        var JourneyRunner = new JourneyRunner({
            // start index.html in web folder
            launchUrl: sap.ui.require.toUrl('inw/vp2/vp2') + '/index.html'
        });

       
        JourneyRunner.run(
            {
                pages: { 
					onThePackingList: PackingList,
					onThePackingObjectPage: PackingObjectPage,
					onTheComponentsObjectPage: ComponentsObjectPage
                }
            },
            opaJourney.run
        );
    }
);