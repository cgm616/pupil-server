// pull in desired CSS/SASS files
require( './styles/main.scss' );
var $ = jQuery = require( '../../node_modules/jquery/dist/jquery.js' );           // <--- remove if jQuery not needed

// inject bundled Elm app into div#main
var Elm = require( '../elm/Threshold' );
Elm.Threshold.embed( document.getElementById( 'threshold' ) );
