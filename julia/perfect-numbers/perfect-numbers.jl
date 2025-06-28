@inline function is_positive_int(n::Integer)
    n <= 0 && throw(DomainError(n, "Only positive integers are allowed"))
end

function aliquot_sum(n::Integer)
    is_positive_int(n)
    n == 1 && return 0

    sum_divisors = 1
    sqrt_n = isqrt(n)
    # We check each divisor (i) up to √n
    @inbounds for i in 2:sqrt_n
        if n % i ≡ 0
            sum_divisors += i
            complement = n ÷ i
            (complement ≠ i) & (complement ≠ n) && (sum_divisors += complement)
        end
    end

    return sum_divisors
end

isperfect(n::Integer) = aliquot_sum(n) == n
isabundant(n::Integer) = aliquot_sum(n) > n
isdeficient(n::Integer) = aliquot_sum(n) < n

