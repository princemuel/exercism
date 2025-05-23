function exchange_money(budget, exchange_rate)
    budget / exchange_rate
end

function get_change(budget, exchanging_value)
    budget - exchanging_value
end

function get_value_of_bills(denomination, number_of_bills)
    denomination * number_of_bills
end

function get_number_of_bills(amount, denomination)
    amount ÷ denomination
end

function get_leftover_of_bills(amount, denomination)
    amount % denomination
end

function exchangeable_value(budget, exchange_rate, spread, denomination)
    actual_exchange_rate = exchange_rate * (1 + spread / 100)
    exchanged_amount = exchange_money(budget, actual_exchange_rate)
    number_of_bills = get_number_of_bills(exchanged_amount, denomination)

    get_value_of_bills(denomination, number_of_bills)
end

