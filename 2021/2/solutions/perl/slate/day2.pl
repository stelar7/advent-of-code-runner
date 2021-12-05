$horiz = 0;
$depth = 0;
$depth2 = 0;
$horiz2 = 0;
$aim = 0;

@text = ();

while (<>){
    push @text, $_;
}

for $i (0 .. $#text) {
    $strlength = length(@text[$i]);
    if ($strlength == 10){
        $horizon = substr(@text[$i], -2);
        $horiz += $horizon;
    } elsif ($strlength == 7) {
        $deep = substr(@text[$i], -2);
        $depth += $deep;
    } elsif ($strlength == 5) {
        $deep2 = substr(@text[$i], -2);
        $depth -= $deep2;
    }
}

for $j (0 .. $#text){
    $strlength = length(@text[$j]);
    if ($strlength == 10){
        $horizon = substr(@text[$j], -2);
        $horiz2 += $horizon;
        $depth2 += ($horizon * $aim)
    } elsif ($strlength == 7) {
        $deep = substr(@text[$j], -2);
        $aim += $deep;
    } elsif ($strlength == 5) {
        $deep2 = substr(@text[$j], -2);
        $aim -= $deep2;
    }
}

print $horiz * $depth, "\n";
print $horiz2 * $depth2, "\n";