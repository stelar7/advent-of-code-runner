#!/usr/bin/perl
my @arr = ();

while (<>) {
    push @arr , $_;
}

my $length;

foreach (@arr) {
    if (@arr[$length+1] > @arr[$length]) {
        $counter += 1;
    }
    if ((@arr[$length+1] + @arr[$length+2] + @arr[$length+3]) > (@arr[$length] + @arr[$length+1] + @arr[$length+2])) {
        $countertwo += 1;
    }
    $length++;
}

print $counter, "\n";
print $countertwo, "\n";
