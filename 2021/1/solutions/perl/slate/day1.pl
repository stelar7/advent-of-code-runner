#!/usr/bin/perl

$past = 0;
$current = 0;
$counter = 0;

$threepast = 0;
$twopast = 0;
$countertwo = 0;

my @arr = ();

while (<>) {
    push @arr , $_;
    if ($current > $past) {
        $counter += 1;
    }
    $past = $current;
    $current = $_;
}

$length = @arr

for ($i=0; $i<@arr; ++$i) {
    if ((@arr[i+1] + @arr[i+2] + @arr[i+3]) < (@arr[i+2] + @arr[i+1] + @arr[i])) {
        $countertwo += 1;
    }
}

print $counter, "\n";
print $countertwo, "\n";
