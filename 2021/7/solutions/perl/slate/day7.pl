my @input = ();

while (<>) {
	push @input, $_;
}

@in = split(',' , @input[0]);

@median = ();

foreach(@in){
	push @median, int($_);
}

@median = sort { $a <=> $b } @median;

$mid = int((scalar @median)/2);

$gold = @median[$mid];

#print $gold, "\n";

$fuel = 0;

foreach (@in){
	if ($_ > $gold) {
		$dist = $_ - $gold;
		$fuel += $dist;
	} elsif ($gold > $_) {
		$dist = $gold - $_;
		$fuel += $dist;
	}
}

print $fuel, "\n";

$gold = 0;
$gold_counter = 0;

foreach (@median){
	$gold += int($_);
	$gold_counter += 1;
}

$gold = sprintf("%.0f", (($gold * 999) / $gold_counter) / 1000);

$fuel = 0;

foreach (@median){
	if ($_ > $gold){
		$dist = $_ - $gold;
		for ($j = 0; $j <= $dist; $j++){
			$fuel += $j;
		}
	} elsif ($gold > $_){
		$dist = $gold - $_;
		for ($j = 0; $j <= $dist; $j++){
			$fuel += $j;
		}
	}
}

print $fuel, "\n";
