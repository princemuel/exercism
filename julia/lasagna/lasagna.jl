# Define the `expected_bake_time` constant`
const expected_bake_time = 60

# Define the `preparation_time(layers)` function.
preparation_time(layers::Int)::Int = layers * 2

# Define the `remaining_time(time_in_oven)` function.
remaining_time(time_in_oven::Int)::Int = expected_bake_time - time_in_oven

# Define the `total_working_time(layers, time_in_oven)` function.
total_working_time(layers::Int, time_in_oven::Int)::Int = preparation_time(layers) + time_in_oven
