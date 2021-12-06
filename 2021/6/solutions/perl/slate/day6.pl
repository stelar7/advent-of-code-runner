my @input = ();

while(<>) {
	push @input, $_;
}

my @in  = split(',' , @input[0]);

my @fish = (); $#fish = 9;

foreach (@in){
	@fish[$_]++;
}

my $counter = 0;
my $counter2 = 0;

for ($j = 0; $j < 80; $j++){
	$tmp = @fish[0];
	$length = (scalar @fish) - 1;
	for ($i = 0; $i <= $length; $i++){
		@fish[$i] = @fish[$i+1];
	}
	@fish[6] += $tmp;
	@fish[8] = $tmp;
}

foreach (@fish){
	$counter += $_;
}

print $counter, "\n";
$j = 0;

my @fish = (); $#fish = 9;
foreach (@in){
	@fish[$_]++;
}

for ($j = 0; $j < 256; $j++){
	$tmp = @fish[0];
	$length = (9) - 1;
	for ($i = 0; $i <= 9; $i++){
		@fish[$i] = @fish[$i+1];
	}
	@fish[6] += $tmp;
	@fish[8] = $tmp;
}

foreach (@fish){
	$counter2 += $_;
}

print $counter2, "\n";
