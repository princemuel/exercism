function time_to_mix_juice(juice::String)::Float64
    mix_times = Dict(
        zip(
            ("Pure Strawberry Joy", "Energizer", "Green Garden", "Tropical Island", "All or Nothing"),
            (0.5, 1.5, 1.5, 3, 5))
    )

    get(mix_times, juice, 2.5)
end

function wedges_from_lime(size)
    wedge_counts = Dict("small" => 6, "medium" => 8, "large" => 10)
    get(wedge_counts, size, 0)  # Returns 0 for unknown sizes
end

function limes_to_cut(needed, limes)
    limes_cut = 0
    limes = copy(limes)

    while needed > 0 && !isempty(limes)
        limes_cut += 1
        needed -= wedges_from_lime(popfirst!(limes))
    end

    limes_cut
end



function order_times(orders)
    [time_to_mix_juice(order) for order in orders]
end

function remaining_orders(time_left, orders)
    orders = copy(orders)

    while time_left > 0 && !isempty(orders)
        time_left -= time_to_mix_juice(popfirst!(orders))
    end

    orders
end
