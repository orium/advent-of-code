#!/usr/bin/perl -w

use strict;

#   0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....
#
#   5:      6:      7:      8:      9:
#  aaaa    aaaa    aaaa    aaaa    aaaa
# b    .  b    .  .    c  b    c  b    c
# b    .  b    .  .    c  b    c  b    c
#  dddd    dddd    ....    dddd    dddd
# .    f  e    f  .    f  e    f  .    f
# .    f  e    f  .    f  e    f  .    f
#  gggg    gggg    ....    gggg    gggg

# map from number of segments to the numbers that can be represented with that number of segments.
my %numbers_by_segments_number = (
    2 => [1,],
    3 => [7,],
    4 => [4,],
    5 => [2, 3, 5],
    6 => [0, 6, 9],
    7 => [8,],
);

my $count = 0;

while (<>) {
    my ($p, $o) = split / \| /, $_;
    chomp $o;
    my @oo = split / /, $o;

    foreach (@oo) {
        my $s = $_;
        my @digits = @{$numbers_by_segments_number{length $s}};
        my $num_digits = scalar @digits;

        if ($num_digits == 1) {
            $count = $count + 1
        }
    }
}

print "$count\n";
