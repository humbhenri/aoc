use v5.38.0;

my $example = 'xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))';
my $inputFile = '03.input';
open my $file, $inputFile or die $!;
my $input = do { local($/); <$file> };

sub part1($memory) {
  my $sum = 0;
  while($memory =~ m/mul\((\d+),(\d+)\)/g) {
    $sum += $1 * $2;
  }
  say "Sum: $sum\n";
}

#part1($input);

sub part2($memory) {
  my $sum = 0;
  my $do = 1;
  while($memory =~ /(do\(\)|don't\(\)|mul\(\d+,\d+\))/g) {
    if ($1 eq "do()" || $1 eq "don't()") {
      $do = $1 eq "do()";
    } elsif ($do) {
      $1 =~ /(\d+),(\d+)/;
      $sum += $1 * $2;
    }
  }
  say "Sum: $sum\n";
}

my $example2 = 'xmul(2,4)&mul[3,7]!^don\'t()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))';
part2($input);


close($file);
