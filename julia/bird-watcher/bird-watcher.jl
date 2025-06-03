today(birds_per_day) = birds_per_day[end]

function increment_todays_count(birds_per_day)
    result = copy(birds_per_day)
    result[end] += 1
    return result
end

has_day_without_birds(birds_per_day) = 0 in birds_per_day


function count_for_first_days(birds_per_day, num_days)
    sum(@view birds_per_day[1:min(num_days, length(birds_per_day))])
end

busy_days(birds_per_day) = sum(birds_per_day .>= 5)


average_per_day(week1, week2) = (week1 .+ week2) ./ 2

