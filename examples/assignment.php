<?php

$a = 0;
$b = 1;

// Will trigger a php-hound warning.
if ($a = $b) {}

// Will trigger a php-hound warning.
while ($a = $b) {}

// Will not trigger a php-hound warning.
for ($i = 0; $i < 10; $i++) {}
