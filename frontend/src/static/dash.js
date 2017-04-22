// pull in desired CSS/SASS files
require( './styles/main.scss' );

// inject bundled Elm app
var Elm = require( '../elm/Dash' );
Elm.Dash.embed( document.getElementById( 'app' ) );
