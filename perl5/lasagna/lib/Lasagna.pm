package Lasagna;

use v5.40;

our $ExpectedMinutesInOven = 40;

sub remaining_minutes_in_oven ($actual_minutes_in_oven) {
    return $ExpectedMinutesInOven - $actual_minutes_in_oven;
}

sub preparation_time_in_minutes ($number_of_layers) {
    my $minutes_per_layer = 2;
    return $number_of_layers * $minutes_per_layer;
}

sub total_time_in_minutes ( $number_of_layers, $actual_minutes_in_oven ) {
    my $preparation_time = preparation_time_in_minutes($number_of_layers);
    return $preparation_time + $actual_minutes_in_oven;
}

sub oven_alarm () {
    return "Ding!";
}

1;
