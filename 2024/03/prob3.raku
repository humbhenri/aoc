my $part1 = 0;
my $part2 = 0;

my $do = True;
for "03.input".IO.lines -> $line {
    for $line ~~ m:g/ mul\((\d+)\,(\d+)\) / {
        my ($a, $b) = $_.list;
        $part1 += $a.Int * $b.Int;
    }
    for $line ~~ m:g/ (do\(\) || "don't"\(\) || mul\((\d+)\,(\d+)\)) / {
        if $_.Str eq "do()" || $_.Str eq "don't()" {
            $do = $_.Str eq "do()";
        } elsif $do {
            my ($a, $b) = $_.list[0].list;
            $part2 += $a.Int * $b.Int;
        }
    }
}

say "part 1 = ", $part1;
say "part 2 = ", $part2;
