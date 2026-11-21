# Built-in functions

Built-in functions are [predeclared](declarations-and-scope.md#predeclared-identifiers). They are called like any other function but some of them accept a type instead of an expression as the first argument.

The built-in functions do not have standard Go types, so they can only appear in [call expressions](expressions.md#calls); they cannot be used as function values.

### Appending to and copying slices

The built-in functions `append` and `copy` assist in common slice operations. For both functions, the result is independent of whether the memory referenced by the arguments overlaps.

The [variadic](types.md#function-types) function `append` appends zero or more values `x` to a slice `s` of type `S` and returns the resulting slice, also of type `S`. The values `x` are passed to a parameter of type `...E` where `E` is the element type of `S` and the respective [parameter passing rules](expressions.md#passing-arguments-to--parameters) apply. As a special case, `append` also accepts a slice whose type is assignable to type `[]byte` with a second argument of `string` type followed by `...`. This form appends the bytes of the string.

``` text
append(s S, x ...E) S  // E is the element type of S
```

If `S` is a [type parameter](declarations-and-scope.md#type-parameter-declarations), all types in its type set must have the same underlying slice type `[]E`.

If the capacity of `s` is not large enough to fit the additional values, `append` [allocates](#allocation) a new, sufficiently large underlying array that fits both the existing slice elements and the additional values. Otherwise, `append` re-uses the underlying array.

``` go
s0 := []int{0, 0}
s1 := append(s0, 2)                // append a single element     s1 is []int{0, 0, 2}
s2 := append(s1, 3, 5, 7)          // append multiple elements    s2 is []int{0, 0, 2, 3, 5, 7}
s3 := append(s2, s0...)            // append a slice              s3 is []int{0, 0, 2, 3, 5, 7, 0, 0}
s4 := append(s3[3:6], s3[2:]...)   // append overlapping slice    s4 is []int{3, 5, 7, 2, 3, 5, 7, 0, 0}

var t []interface{}
t = append(t, 42, 3.1415, "foo")   //                             t is []interface{}{42, 3.1415, "foo"}

var b []byte
b = append(b, "bar"...)            // append string contents      b is []byte{'b', 'a', 'r' }
```

The function `copy` copies slice elements from a source `src` to a destination `dst` and returns the number of elements copied. Both arguments must have [identical](properties-of-types-and-values.md#type-identity) element type `E` and must be assignable to a slice of type `[]E`. The number of elements copied is the minimum of `len(src)` and `len(dst)`. As a special case, `copy` also accepts a destination argument assignable to type `[]byte` with a source argument of a `string` type. This form copies the bytes from the string into the byte slice.

``` text
copy(dst, src []T) int
copy(dst []byte, src string) int
```

If the type of one or both arguments is a [type parameter](declarations-and-scope.md#type-parameter-declarations), all types in their respective type sets must have the same underlying slice type `[]E`.

Examples:

``` go
var a = [...]int{0, 1, 2, 3, 4, 5, 6, 7}
var s = make([]int, 6)
var b = make([]byte, 5)
n1 := copy(s, a[0:])            // n1 == 6, s is []int{0, 1, 2, 3, 4, 5}
n2 := copy(s, s[2:])            // n2 == 4, s is []int{2, 3, 4, 5, 4, 5}
n3 := copy(b, "Hello, World!")  // n3 == 5, b is []byte("Hello")
```

### Clear

The built-in function `clear` takes an argument of [map](types.md#map-types), [slice](types.md#slice-types), or [type parameter](declarations-and-scope.md#type-parameter-declarations) type, and deletes or zeroes out all elements \[[Go 1.21](appendix.md#go-121)\].

``` text
Call        Argument type     Result

clear(m)    map[K]T           deletes all entries, resulting in an
                              empty map (len(m) == 0)

clear(s)    []T               sets all elements up to the length of
                              s to the zero value of T

clear(t)    type parameter    see below
```

If the type of the argument to `clear` is a [type parameter](declarations-and-scope.md#type-parameter-declarations), all types in its type set must be maps or slices, and `clear` performs the operation corresponding to the actual type argument.

If the map or slice is `nil`, `clear` is a no-op.

### Close

For a channel `ch`, the built-in function `close(ch)` records that no more values will be sent on the channel. It is an error if `ch` is a receive-only channel. Sending to or closing a closed channel causes a [run-time panic](run-time-panics.md). Closing the nil channel also causes a [run-time panic](run-time-panics.md). After calling `close`, and after any previously sent values have been received, receive operations will return the zero value for the channel's type without blocking. The multi-valued [receive operation](expressions.md#receive-operator) returns a received value along with an indication of whether the channel is closed.

If the type of the argument to `close` is a [type parameter](declarations-and-scope.md#type-parameter-declarations), all types in its type set must be channels. It is an error if any of those channels is a receive-only channel.

### Manipulating complex numbers

Three functions assemble and disassemble complex numbers. The built-in function `complex` constructs a complex value from a floating-point real and imaginary part, while `real` and `imag` extract the real and imaginary parts of a complex value.

``` text
complex(realPart, imaginaryPart floatT) complexT
real(complexT) floatT
imag(complexT) floatT
```

The type of the arguments and return value correspond. For `complex`, the two arguments must be of the same [floating-point type](types.md#numeric-types) and the return type is the [complex type](types.md#numeric-types) with the corresponding floating-point constituents: `complex64` for `float32` arguments, and `complex128` for `float64` arguments. If one of the arguments evaluates to an untyped constant, it is first implicitly [converted](expressions.md#conversions) to the type of the other argument. If both arguments evaluate to untyped constants, they must be non-complex numbers or their imaginary parts must be zero, and the return value of the function is an untyped complex constant.

For `real` and `imag`, the argument must be of complex type, and the return type is the corresponding floating-point type: `float32` for a `complex64` argument, and `float64` for a `complex128` argument. If the argument evaluates to an untyped constant, it must be a number, and the return value of the function is an untyped floating-point constant.

The `real` and `imag` functions together form the inverse of `complex`, so for a value `z` of a complex type `Z`, `z == Z(complex(real(z), imag(z)))`.

If the operands of these functions are all constants, the return value is a constant.

``` go
var a = complex(2, -2)             // complex128
const b = complex(1.0, -1.4)       // untyped complex constant 1 - 1.4i
x := float32(math.Cos(math.Pi/2))  // float32
var c64 = complex(5, -x)           // complex64
var s int = complex(1, 0)          // untyped complex constant 1 + 0i can be converted to int
_ = complex(1, 2<<s)               // illegal: 2 assumes floating-point type, cannot shift
var rl = real(c64)                 // float32
var im = imag(a)                   // float64
const c = imag(b)                  // untyped constant -1.4
_ = imag(3 << s)                   // illegal: 3 assumes complex type, cannot shift
```

Arguments of type parameter type are not permitted.

### Deletion of map elements

The built-in function `delete` removes the element with key `k` from a [map](types.md#map-types) `m`. The value `k` must be [assignable](properties-of-types-and-values.md#assignability) to the key type of `m`.

``` text
delete(m, k)  // remove element m[k] from map m
```

If the type of `m` is a [type parameter](declarations-and-scope.md#type-parameter-declarations), all types in that type set must be maps, and they must all have identical key types.

If the map `m` is `nil` or the element `m[k]` does not exist, `delete` is a no-op.

### Length and capacity

The built-in functions `len` and `cap` take arguments of various types and return a result of type `int`. The implementation guarantees that the result always fits into an `int`.

``` text
Call      Argument type    Result

len(s)    string type      string length in bytes
          [n]T, *[n]T      array length (== n)
          []T              slice length
          map[K]T          map length (number of defined keys)
          chan T           number of elements queued in channel buffer
          type parameter   see below

cap(s)    [n]T, *[n]T      array length (== n)
          []T              slice capacity
          chan T           channel buffer capacity
          type parameter   see below
```

If the argument type is a [type parameter](declarations-and-scope.md#type-parameter-declarations) `P`, the call `len(e)` (or `cap(e)` respectively) must be valid for each type in `P`'s type set. The result is the length (or capacity, respectively) of the argument whose type corresponds to the type argument with which `P` was [instantiated](expressions.md#instantiations).

The capacity of a slice is the number of elements for which there is space allocated in the underlying array. At any time the following relationship holds:

``` go
0 <= len(s) <= cap(s)
```

The length of a `nil` slice, map or channel is 0. The capacity of a `nil` slice or channel is 0.

The expression `len(s)` is [constant](constants.md) if `s` is a string constant. The expressions `len(s)` and `cap(s)` are constants if the type of `s` is an array or pointer to an array and the expression `s` does not contain [channel receives](expressions.md#receive-operator) or (non-constant) [function calls](expressions.md#calls); in this case `s` is not evaluated. Otherwise, invocations of `len` and `cap` are not constant and `s` is evaluated.

``` go
const (
    c1 = imag(2i)                    // imag(2i) = 2.0 is a constant
    c2 = len([10]float64{2})         // [10]float64{2} contains no function calls
    c3 = len([10]float64{c1})        // [10]float64{c1} contains no function calls
    c4 = len([10]float64{imag(2i)})  // imag(2i) is a constant and no function call is issued
    c5 = len([10]float64{imag(z)})   // invalid: imag(z) is a (non-constant) function call
)
var z complex128
```

### Making slices, maps and channels

The built-in function `make` takes a type `T`, which must be a slice, map or channel type, or a type parameter, optionally followed by a type-specific list of expressions. It returns a value of type `T` (not `*T`). The memory is initialized as described in the section on [initial values](program-initialization-and-execution.md#the-zero-value).

``` text
Call             Type T            Result

make(T, n)       slice             slice of type T with length n and capacity n
make(T, n, m)    slice             slice of type T with length n and capacity m

make(T)          map               map of type T
make(T, n)       map               map of type T with initial space for approximately n elements

make(T)          channel           unbuffered channel of type T
make(T, n)       channel           buffered channel of type T, buffer size n

make(T, n)       type parameter    see below
make(T, n, m)    type parameter    see below
```

If the first argument is a [type parameter](declarations-and-scope.md#type-parameter-declarations), all types in its type set must have the same underlying type, which must be a slice or map type, or, if there are channel types, there must only be channel types, they must all have the same element type, and the channel directions must not conflict.

Each of the size arguments `n` and `m` must be of [integer type](types.md#numeric-types), have a [type set](types.md#interface-types) containing only integer types, or be an untyped [constant](constants.md). A constant size argument must be non-negative and [representable](properties-of-types-and-values.md#representability) by a value of type `int`; if it is an untyped constant it is given type `int`. If both `n` and `m` are provided and are constant, then `n` must be no larger than `m`. For slices and channels, if `n` is negative or larger than `m` at run time, a [run-time panic](run-time-panics.md) occurs.

``` go
s := make([]int, 10, 100)       // slice with len(s) == 10, cap(s) == 100
s := make([]int, 1e3)           // slice with len(s) == cap(s) == 1000
s := make([]int, 1<<63)         // illegal: len(s) is not representable by a value of type int
s := make([]int, 10, 0)         // illegal: len(s) > cap(s)
c := make(chan int, 10)         // channel with a buffer size of 10
m := make(map[string]int, 100)  // map with initial space for approximately 100 elements
```

Calling `make` with a map type and size hint `n` will create a map with initial space to hold `n` map elements. The precise behavior is implementation-dependent.

### Min and max

The built-in functions `min` and `max` compute the smallest—or largest, respectively—value of a fixed number of arguments of [ordered types](expressions.md#comparison-operators). There must be at least one argument \[[Go 1.21](appendix.md#go-121)\].

The same type rules as for [operators](expressions.md#operators) apply: for [ordered](expressions.md#comparison-operators) arguments `x` and `y`, `min(x, y)` is valid if `x + y` is valid, and the type of `min(x, y)` is the type of `x + y` (and similarly for `max`). If all arguments are constant, the result is constant.

``` go
var x, y int
m := min(x)                 // m == x
m := min(x, y)              // m is the smaller of x and y
m := max(x, y, 10)          // m is the larger of x and y but at least 10
c := max(1, 2.0, 10)        // c == 10.0 (floating-point kind)
f := max(0, float32(x))     // type of f is float32
var s []string
_ = min(s...)               // invalid: slice arguments are not permitted
t := max("", "foo", "bar")  // t == "foo" (string kind)
```

For numeric arguments, assuming all NaNs are equal, `min` and `max` are commutative and associative:

``` go
min(x, y)    == min(y, x)
min(x, y, z) == min(min(x, y), z) == min(x, min(y, z))
```

For floating-point arguments negative zero, NaN, and infinity the following rules apply:

``` go
   x        y    min(x, y)    max(x, y)

  -0.0    0.0         -0.0          0.0    // negative zero is smaller than (non-negative) zero
  -Inf      y         -Inf            y    // negative infinity is smaller than any other number
  +Inf      y            y         +Inf    // positive infinity is larger than any other number
   NaN      y          NaN          NaN    // if any argument is a NaN, the result is a NaN
```

For string arguments the result for `min` is the first argument with the smallest (or for `max`, largest) value, compared lexically byte-wise:

``` go
min(x, y)    == if x <= y then x else y
min(x, y, z) == min(min(x, y), z)
```

### Allocation

The built-in function `new` creates a new, initialized [variable](variables.md) and returns a [pointer](types.md#pointer-types) to it. It accepts a single argument, which may be either a type or an expression.

If the argument is a type `T`, then `new(T)` allocates a variable of type `T` initialized to its [zero value](program-initialization-and-execution.md#the-zero-value).

If the argument is an expression `x`, then `new(x)` allocates a variable of the type of `x` initialized to the value of `x`. If that value is an untyped constant, it is first implicitly [converted](expressions.md#conversions) to its [default type](constants.md); if it is an untyped boolean value, it is first implicitly converted to type bool. The predeclared identifier `nil` cannot be used as an argument to `new`.

For example, `new(int)` and `new(123)` each return a pointer to a new variable of type `int`. The value of the first variable is `0`, and the value of the second is `123`. Similarly

``` go
type S struct { a int; b float64 }
new(S)
```

allocates a variable of type `S`, initializes it (`a=0`, `b=0.0`), and returns a value of type `*S` containing the address of the variable.

### Handling panics

Two built-in functions, `panic` and `recover`, assist in reporting and handling [run-time panics](run-time-panics.md) and program-defined error conditions.

``` text
func panic(interface{})
func recover() interface{}
```

While executing a function `F`, an explicit call to `panic` or a [run-time panic](run-time-panics.md) terminates the execution of `F`. Any functions [deferred](statements.md#defer-statements) by `F` are then executed as usual. Next, any deferred functions run by `F`'s caller are run, and so on up to any deferred by the top-level function in the executing goroutine. At that point, the program is terminated and the error condition is reported, including the value of the argument to `panic`. This termination sequence is called *panicking*.

``` go
panic(42)
panic("unreachable")
panic(Error("cannot parse"))
```

The `recover` function allows a program to manage behavior of a panicking goroutine. Suppose a function `G` defers a function `D` that calls `recover` and a panic occurs in a function on the same goroutine in which `G` is executing. When the running of deferred functions reaches `D`, the return value of `D`'s call to `recover` will be the value passed to the call of `panic`. If `D` returns normally, without starting a new `panic`, the panicking sequence stops. In that case, the state of functions called between `G` and the call to `panic` is discarded, and normal execution resumes. Any functions deferred by `G` before `D` are then run and `G`'s execution terminates by returning to its caller.

The return value of `recover` is `nil` when the goroutine is not panicking or `recover` was not called directly by a deferred function. Conversely, if a goroutine is panicking and `recover` was called directly by a deferred function, the return value of `recover` is guaranteed not to be `nil`. To ensure this, calling `panic` with a `nil` interface value (or an untyped `nil`) causes a [run-time panic](run-time-panics.md).

The `protect` function in the example below invokes the function argument `g` and protects callers from run-time panics caused by `g`.

``` go
func protect(g func()) {
    defer func() {
        log.Println("done")  // Println executes normally even if there is a panic
        if x := recover(); x != nil {
            log.Printf("run time panic: %v", x)
        }
    }()
    log.Println("start")
    g()
}
```

### Bootstrapping

Current implementations provide several built-in functions useful during bootstrapping. These functions are documented for completeness but are not guaranteed to stay in the language. They do not return a result.

``` text
Function   Behavior

print      prints all arguments; formatting of arguments is implementation-specific
println    like print but prints spaces between arguments and a newline at the end
```

Implementation restriction: `print` and `println` need not accept arbitrary argument types, but printing of boolean, numeric, and string [types](types.md) must be supported.
