<?php
//setting the timezone below
date_default_timezone_set("Asia/Kolkata");
//name input below 
echo "ENTER YOUR NAME: ";
$name = trim(fgets(STDIN));
//combined output
echo "HELLO $name, right now the time is " . date("H:i:s A");
?>
