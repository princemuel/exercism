function success_rate(speed)
    speed < 1 ? 0.0 :
    speed < 5 ? 1.0 :
    speed < 9 ? 0.9 :
    speed < 10 ? 0.8 : 0.77
end


function production_rate_per_hour(speed)
    speed * 221.0 * success_rate(speed)
end

function working_items_per_minute(speed)
    Int(production_rate_per_hour(speed) ÷ 60)
end
