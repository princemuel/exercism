(module
  ;;
  ;; Determine if a number is an Armstrong number.
  ;;
  ;; @param {i32} candidate - The number to check.
  ;;
  ;; @return {i32} 1 if the number is an Armstrong number, 0 otherwise.
  ;;
  ;; Helper function to calculate integer power (base^exponent)
  (func $pow (param $base i32) (param $exponent i32) (result i32)
    (local $result i32) (local $counter i32)

    ;; Initialize result to 1
    (local.set $result
      (i32.const 1))
    (local.set $counter
      (i32.const 0))

    ;; Handle special case: anything^0 = 1
    (if
      (i32.eqz
        (local.get $exponent))
      (then
        (return
          (i32.const 1))))

    ;; Power calculation loop
    (loop $power_loop
      (if
        (i32.lt_u
          (local.get $counter)
          (local.get $exponent))
        (then
          (local.set $result
            (i32.mul
              (local.get $result)
              (local.get $base)))
          (local.set $counter
            (i32.add
              (local.get $counter)
              (i32.const 1)))
          (br $power_loop))))

    (local.get $result))

  ;; Helper function to count digits in a number
  (func $count_digits (param $num i32) (result i32)
    (local $count i32) (local $temp i32)

    ;; Handle zero case
    (if
      (i32.eqz
        (local.get $num))
      (then
        (return
          (i32.const 1))))

    (local.set $temp
      (local.get $num))
    (local.set $count
      (i32.const 0))

    ;; Count digits by dividing by 10 until we reach 0
    (loop $count_loop
      (if
        (i32.gt_u
          (local.get $temp)
          (i32.const 0))
        (then
          (local.set $count
            (i32.add
              (local.get $count)
              (i32.const 1)))
          (local.set $temp
            (i32.div_u
              (local.get $temp)
              (i32.const 10)))
          (br $count_loop))))

    (local.get $count))

  ;; Main function to check if a number is an Armstrong number
  (func (export "isArmstrongNumber") (param $candidate i32) (result i32)
    (local $digit_count i32) (local $sum i32) (local $temp i32) (local $digit i32)

    ;; Get the number of digits
    (local.set $digit_count
      (call $count_digits
        (local.get $candidate)))

    ;; Initialize sum and temp
    (local.set $sum
      (i32.const 0))
    (local.set $temp
      (local.get $candidate))

    ;; Process each digit
    (loop $digit_loop
      (if
        (i32.gt_u
          (local.get $temp)
          (i32.const 0))
        (then
          ;; Extract the rightmost digit
          (local.set $digit
            (i32.rem_u
              (local.get $temp)
              (i32.const 10)))

          ;; Add digit^digit_count to sum
          (local.set $sum
            (i32.add
              (local.get $sum)
              (call $pow
                (local.get $digit)
                (local.get $digit_count))))

          ;; Remove the rightmost digit
          (local.set $temp
            (i32.div_u
              (local.get $temp)
              (i32.const 10)))

          (br $digit_loop))))

    ;; Return 1 if sum equals candidate, 0 otherwise
    (if (result i32)
      (i32.eq
        (local.get $sum)
        (local.get $candidate))
      (then
        (i32.const 1))
      (else
        (i32.const 0)))))
