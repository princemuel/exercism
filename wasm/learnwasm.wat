;; learn-wasm.wast

(module
  ;; In WebAssembly, everything is included in a module. Moreover, everything
  ;; can be expressed as an s-expression. Alternatively, there is the
  ;; "stack machine" syntax, but that is not compatible with Binaryen
  ;; intermediate representation (IR) syntax.

  ;; The Binaryen IR format is *mostly* compatible with WebAssembly text format.
  ;; There are some small differences:
  ;; local_set -> local.set
  ;; local_get -> local.get

  ;; We have to enclose code in functions

  ;; Data Types
  (func $data_types
    ;; WebAssembly has only four types:
    ;; i32 - 32 bit integer
    ;; i64 - 64 bit integer (not supported in JavaScript)
    ;; f32 - 32 bit floating point
    ;; f64 - 64 bit floating point

    ;; We can declare local variables with the "local" keyword
    ;; We have to declare all variables before we start doing anything
    ;; inside the function

    (local $int_32 i32)
    (local $int_64 i64)
    (local $float_32 f32)
    (local $float_64 f64)

    ;; These values remain uninitialized.
    ;; To set them to a value, we can use <type>.const:

    (local.set $int_32 (i32.const 16))
    (local.set $int_64 (i64.const 128))
    (local.set $float_32 (f32.const 3.14))
    (local.set $float_64 (f64.const 1.28))
  )

  ;; Basic operations
  (func $basic_operations

    ;; In WebAssembly, everything is an s-expression, including
    ;; doing math, or getting the value of some variable

    (local $add_result i32)
    (local $mult_result f64)

    (local.set $add_result (i32.add (i32.const 2) (i32.const 4)))
    ;; the value of add_result is now 6!

    ;; We have to use the right data type for each operation:
    ;; (local.set $mult_result (f32.mul (f32.const 2.0) (f32.const 4.0))) ;; WRONG! mult_result is f64!
    (local.set $mult_result (f64.mul (f64.const 2.0) (f64.const 4.0)))

    ;; WebAssembly has some builtin operations, like basic math and bitshifting.
    ;; Notably, it does not have built in trigonometric functions.
    ;; In order to get access to these functions, we have to either
    ;; - implement them ourselves (not recommended)
    ;; - import them from elsewhere (later on)
  )

  ;; Functions
  ;; We specify arguments with the `param` keyword, and specify return values
  ;; with the `result` keyword
  ;; The current value on the stack is the return value of a function

  ;; We can call other functions we've defined with the `call` keyword

  (func $get_16 (result i32)
    (i32.const 16)
  )

  (func $add (param $param0 i32) (param $param1 i32) (result i32)
    (i32.add
      (local.get $param0)
      (local.get $param1)
    )
  )

  (func $double_16 (result i32)
    (i32.mul
      (i32.const 2)
      (call $get_16))
  )

  ;; Up until now, we haven't be able to print anything out, nor do we have
  ;; access to higher level math functions (pow, exp, or trig functions).
  ;; Moreover, we haven't been able to use any of the WASM functions in JavaScript!
  ;; The way we get those functions into WebAssembly
  ;; looks different whether we're in a Node.js or browser environment.

  ;; If we're in Node.js we have to do two steps. First we have to convert the
  ;; WASM text representation into actual webassembly. If we're using Binyaren,
  ;; we can do that with a command like the following:

  ;; wasm-as learn-wasm.wast -o learn-wasm.wasm

  ;; We can apply Binaryen optimizations to that file with a command like the
  ;; following:

  ;; wasm-opt learn-wasm.wasm -o learn-wasm.opt.wasm -O3 --rse

  ;; With our compiled WebAssembly, we can now load it into Node.js:
  ;; const fs = require('fs')
  ;; const instantiate = async function (inFilePath, _importObject) {
  ;;  var importObject = {
  ;;     console: {
  ;;       log: (x) => console.log(x),
  ;;     },
  ;;     math: {
  ;;       cos: (x) => Math.cos(x),
  ;;     }
  ;;   }
  ;;  importObject = Object.assign(importObject, _importObject)
  ;;
  ;;  var buffer = fs.readFileSync(inFilePath)
  ;;  var module = await WebAssembly.compile(buffer)
  ;;  var instance = await WebAssembly.instantiate(module, importObject)
  ;;  return instance.exports
  ;; }
  ;;
  ;; const main = function () {
  ;;   var wasmExports = await instantiate('learn-wasm.wasm')
  ;;   wasmExports.print_args(1, 0)
  ;; }

  ;; The following snippet gets the functions from the importObject we defined
  ;; in the JavaScript instantiate async function, and then exports a function
  ;; "print_args" that we can call from Node.js

  (import "console" "log" (func $print_i32 (param i32)))
  (import "math" "cos" (func $cos (param f64) (result f64)))

  (func $print_args (param $arg0 i32) (param $arg1 i32)
    (call $print_i32 (local.get $arg0))
    (call $print_i32 (local.get $arg1))
  )
  (export "print_args" (func $print_args))

  ;; Loading in data from WebAssembly memory.
  ;; Say that we want to apply the cosine function to a JavaScript array.
  ;; We need to be able to access the allocated array, and iterate through it.
  ;; This example will modify the input array inplace.
  ;; f64.load and f64.store expect the location of a number in memory *in bytes*.
  ;; If we want to access the 3rd element of an array, we have to pass something
  ;; like (i32.mul (i32.const 8) (i32.const 2)) to the f64.store function.

  ;; In JavaScript, we would call `apply_cos64` as follows
  ;; (using the instantiate function from earlier):
  ;;
  ;; const main = function () {
  ;;   var wasm = await instantiate('learn-wasm.wasm')
  ;;   var n = 100
  ;;   const memory = new Float64Array(wasm.memory.buffer, 0, n)
  ;;   for (var i=0; i<n; i++) {
  ;;     memory[i] = i;
  ;;   }
  ;;   wasm.apply_cos64(n)
  ;; }
  ;;
  ;; This function will not work if we allocate a Float32Array on the JavaScript
  ;; side.

  (memory (export "memory") 100)

  (func $apply_cos64 (param $array_length i32)
    ;; declare the loop counter
    (local $idx i32)
    ;; declare the counter that will allow us to access memory
    (local $idx_bytes i32)
    ;; constant expressing the number of bytes in a f64 number.
    (local $bytes_per_double i32)

    ;; declare a variable for storing the value loaded from memory
    (local $temp_f64 f64)

    (local.set $idx (i32.const 0))
    (local.set $idx_bytes (i32.const 0)) ;; not entirely necessary
    (local.set $bytes_per_double (i32.const 8))

    (block
      (loop
        ;; this sets idx_bytes to bytes offset of the value we're interested in.
        (local.set $idx_bytes (i32.mul (local.get $idx) (local.get $bytes_per_double)))

        ;; get the value of the array from memory:
        (local.set $temp_f64 (f64.load (local.get $idx_bytes)))

        ;; now apply the cosine function:
        (local.set $temp_64 (call $cos (local.get $temp_64)))

        ;; now store the result at the same location in memory:
        (f64.store
          (local.get $idx_bytes)
          (local.get $temp_64))

        ;; do it all in one step instead
        (f64.store
          (local.get $idx_bytes)
          (call $cos
            (f64.load
              (local.get $idx_bytes))))

        ;; increment the loop counter
        (local.set $idx (i32.add (local.get $idx) (i32.const 1)))

        ;; stop the loop if the loop counter is equal the array length
        (br_if 1 (i32.eq (local.get $idx) (local.get $array_length)))
        (br 0)
      )
    )
  )
  (export "apply_cos64" (func $apply_cos64))

  ;; Wasm is a stack-based language, but for returning values more complicated
  ;; than an int/float, a separate memory stack has to be manually managed. One
  ;; approach is to use a mutable global to store the stack_ptr. We give
  ;; ourselves 1MiB of memstack and grow it downwards.
  ;;
  ;; Below is a demonstration of how this C code **might** be written by hand
  ;;
  ;;   typedef struct {
  ;;       int a;
  ;;       int b;
  ;;   } sum_struct_t;
  ;;
  ;;   sum_struct_t sum_struct_create(int a, int b) {
  ;;     return (sum_struct_t){a, b};
  ;;   }
  ;;
  ;;   int sum_local() {
  ;;     sum_struct_t s = sum_struct_create(40, 2);
  ;;     return s.a + s.b;
  ;;   }

  ;; Unlike C, we must manage our own memory stack. We reserve 1MiB
  (global $memstack_ptr (mut i32) (i32.const 65536))

  ;; Structs can only be returned by reference
  (func $sum_struct_create
        (param $sum_struct_ptr i32)
        (param $var$a i32)
        (param $var$b i32)
    ;; c// sum_struct_ptr->a = a;
    (i32.store
      (get_local $sum_struct_ptr)
      (get_local $var$a)
    )

    ;; c// sum_struct_ptr->b = b;
    (i32.store offset=4
      (get_local $sum_struct_ptr)
      (get_local $var$b)
    )
  )

  (func $sum_local (result i32)
    (local $var$sum_struct$a i32)
    (local $var$sum_struct$b i32)
    (local $local_memstack_ptr i32)

    ;; reserve memstack space
    (i32.sub
      (get_global $memstack_ptr)
      (i32.const 8)
    )
    tee_local $local_memstack_ptr ;; tee both stores and returns given value
    set_global $memstack_ptr

    ;; call the function, storing the result in the memstack
    (call $sum_struct_create
      ((;$sum_struct_ptr=;) get_local $local_memstack_ptr)
      ((;$var$a=;) i32.const 40)
      ((;$var$b=;) i32.const 2)
    )

    ;; retrieve values from struct
    (set_local $var$sum_struct$a
      (i32.load offset=0 (get_local $local_memstack_ptr))
    )
    (set_local $var$sum_struct$b
      (i32.load offset=4 (get_local $local_memstack_ptr))
    )

    ;; unreserve memstack space
    (set_global $memstack_ptr
        (i32.add
          (get_local $local_memstack_ptr)
          (i32.const 8)
        )
    )

    (i32.add
      (get_local $var$sum_struct$a)
      (get_local $var$sum_struct$b)
    )
  )
  (export "sum_local" (func $sum_local))
)
