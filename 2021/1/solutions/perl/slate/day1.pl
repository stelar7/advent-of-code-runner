#!/usr/bin/perl

$past = 0;
$current = 0;
$counter = 0;

while (<>) {
    if ($current > $past) {
        $counter += 1;
    }
    $past = $current;
    $current = $_;
}

print $counter;