function raindrops(n)
    ping(divisor, label) = n % divisor == 0 ? label : ""
    result = ping(3, "Pling") * ping(5, "Plang") * ping(7, "Plong")
    return isempty(result) ? string(n) : result
end
