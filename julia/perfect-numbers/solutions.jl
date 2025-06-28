function aliquout_factors(n::Integer)::Array{Integer,1}
    n < 1 && throw(DomainError(n))
    n == 1 && return []
    n == 2 && return [1]
    lst = [x for x in range(2, stop=ceil(sqrt(n))) if n % x == 0]
    return sort(unique(cat(dims=1, [1,], lst, [n ÷ x for x in lst])))
end

isperfect(x)::Bool = x == sum(aliquout_factors(x))
isabundant(x)::Bool = x < sum(aliquout_factors(x))
isdeficient(x)::Bool = x > sum(aliquout_factors(x))


@inline is_positive_int(n::Integer) =
    n <= 0 && throw(DomainError(n, "Only positive integers are allowed"))

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

isperfect(n::Integer) = n == aliquot_sum(n)
isabundant(n::Integer) = n < aliquot_sum(n)
isdeficient(n::Integer) = n > aliquot_sum(n)

