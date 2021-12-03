#!/usr/bin/perl

$past = 0;
$current = 0;
$counter = 0;

$threepast = 0;
$twopast = 0;
$countertwo = -2;

while (<>) {
    if ($current > $past) {
        $counter += 1;
    }
    if (($threepast + $twopast + $past) < ($twopast + $past + $current)) {
	    $countertwo += 1;
    }

    $threepast = $twopast;
    $twopast = $past;
    $past = $current;
    $current = $_;
}

print $counter, "\n";
print $countertwo, "\n";
