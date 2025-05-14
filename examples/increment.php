<?php

$a = 0;

// Will trigger a php-hound warning.
if ($a++ == 1) {}

// Will trigger a php-hound warning.
if ($a-- == 1) {}

// Will trigger a php-hound warning.
if (++$a == 1) {}

// Will trigger a php-hound warning.
if (--$a == 1) {}
