#!/usr/bin/perl

my @in = ();

while (<>) {
	push @in , $_;
}

my $word;
$final = ""; #gamma
$finaluc = ""; #eplison

for ($i = 0; $i <= 11; $i = $i + 1 ){
	$ones = 0;
	$zeroes = 0;

	for my $l (0 .. $#in) {
		$current = @in[$l];
		$letter = substr($current, $i, 1);
		if ($letter == "0"){
			$zeroes += 1;
		}
		if ($letter == "1"){
			$ones += 1;
		}
	}
	if ($ones > $zeroes){
		$final .= "1";
		$finaluc .= "0";
	} else {
		$final .= "0";
		$finaluc .= "1";
	}
}
$decimal = eval("0b$final");
$decimaluc = eval("0b$finaluc");
print $decimal * $decimaluc, "\n";