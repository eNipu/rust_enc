# Calculating Average of Two integer without Overflow

Let's calculate average of two integers and return the average as an `INT`.
The usual approach is integer divison and take `floor` of the of the floating point result.

                avg = floor((a + b) / 2)

But if any of the two values of  `a` or `b` is the `INT_MAX` for C\C++ or  `std::i32::MAX` in Rust;
it will cause a overflow.

Rust compiler will show something as follows:

***thread 'main' panicked at 'attempt to add with overflow',***

So the following pice of code will give us the average without any overflow.

```Rust
fn average_with_overflow(a: i32, b: i32) -> i32 {
    return (a & b) + ((a ^ b) >> 1);
}
```

---

## How this trick works?

The sum of an integer can be written as follows:

    sum = carries + sum without carries

To calculate the above we can do the following:

    a + b == ((a & b) << 1) + (a ^ b)        [Eq.1]

Now dividing by 2 is same as 1 bit right shift.

Therefore,

    (a + b) / 2 = (a + b) >> 1              [Eq.2]

Combining `[Eq.1]` and `[Eq.2]`:

    (a + b) / 2 = (a + b) >> 1 
                = ((( a & b ) << 1) + (a ^ b)) >> 1
                = (a & b) + ((a ^ b) >> 1)  [Eq.3]
Where `>>` is distributive.

## Example:

Let's imagine we have a calculator that has only three registers to hande data. Therefore the max decimal digit it will be able to handle in `9` or `111` in binary.

Let's take two int `a = 5` and `b = 6`
So their binaries are. Here `b'` denotes binary representation.

    a = b'101
    b = b'110

Their integer sum will be

        a + b = 11 (decimal)
              = b'1011 (binary. We can see it overflows.) 

So we have no space to keep the most singnigican bit here. (Asume our number are big-endian).

Let's apply the `[Eq.3]`

    a ^ b = b'101 ^ b'110
          = b'011

    a & b = b'101 & b'110
          = b'100

    (a ^ b) >> 1 = b'011 >> 1
                 = b'001 (so dividing 1 by 2 making the MSB 0)  
    
Finally,

    (a & b)  (a ^ b) >> 1 = b'100 + b'001
                          = b'101
                          = 5


Hope this example helps to get the clear picture.

The Rust code can be found here. <https://gist.github.com/eNipu/6e8c3917865a4eb1e8cb34184d954d28>
It will also work for C/C++. But not python since Python ints are arbitrary precision.
We can verify the correctnes of this trick using Python.