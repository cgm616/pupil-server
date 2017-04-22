// pull in desired CSS/SASS files
require( './styles/main.scss' );

// inject bundled Elm app
var Elm = require( '../elm/Threshold' );
Elm.Threshold.embed( document.getElementById( 'threshold' ) );
