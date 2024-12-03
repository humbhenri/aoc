use v5.38.0;

my $example = 'xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))';
my $inputFile = '03.input';
open my $file, '<', $inputFile or die $!;
my $input = do { local($/); <$file> };
close($file) or warn "Failed to close file $inputFile: $!";

sub part1($memory) {
  my $sum = 0;
  while($memory =~ /mul\((\d+),(\d+)\)/g) {
    $sum += $1 * $2;
  }
  say "Sum: $sum\n";
}

sub part2($memory) {
  my $sum = 0;
  my $do = 1;
  while ($memory =~ /(do\(\)|don't\(\)|mul\((\d+),(\d+)\))/g) {
      if ($1 eq 'do()' || $1 eq "don't()") {
          $do = $1 eq 'do()';
      } elsif ($do && defined $2 && defined $3) {
          $sum += $2 * $3;
      }
  }
  say "Sum: $sum\n";
}

part1($input);
part2($input);
