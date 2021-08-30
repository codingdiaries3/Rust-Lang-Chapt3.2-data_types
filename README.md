### Data Types

1. There are 2 data types subsets: scalar and compound

2. Rust is a statically typed language. It must know the types of all variables at compile time.

3. When changing the value type from string to numeric use parse and add a annotation type. Example:

```
let guess = 'string'
let guess: u32 = string.parse().expect("Not a number!"); //u32 is the annotation type
```

There are different type annotations for other data types.

## Scalar Types

1. A scalar type represents a single value.

2. There are 4 primary scalar types:

- integers
- floating-point numbers
- Booleans
- Characters

# Integer Types

1. Integer is a number without fractional component.

2. There are unsigned and signed integers. Unsigned integers start with 'u' and signed integers start with 'i'
   <pre>
   Length	Signed	Unsigned
   8-bit	  i8	     u8
   16-bit	  i16	     u16
   32-bit	  i32	     u32
   64-bit	  i64	     u64
   128-bit	i128	   u128
   arch	    isize	   usize
   </pre>

   note: u8 can hold values between 0 and 255.\
   i8 can store numbers between -128 to 127

3. Signed integers refers to whether the number needs to have a sign with it '-' or '+'. It has the possibility to go into the negative numbers.

4. Unsigned integers refers to whether it will only ever be positive and can therefore can be represented without a sign. If its safe to assume the number is positive use unsigned integers.

5. If you don't know what type of integer to use rust's defaults are generally a good choice. Its default is i32.

6. The primary situation in which you'd use 'isize' or 'usize' is when indexing some sort of collection.

**Integer Overflow**

1. Integer overflow is when the variable value goes outside of its range. For instance, u8 can hold values between 0 and 255. If the value goes to 256 its an integer overflow.

2. Rust has some rules when this happens. In debug mode, Rust checks for integer overflow and causes your program to panic at runtime.

3. When compiling in release mode with the `--release` flag, Rust will not check for integer overflow. Instead it performs two complement wrapping. \
   Instead values greater than the maximum value type "wraps around" and starts at the minimum. For instance, u8 can hold values between 0 and 255. If the value goes to 256, 256 becomes 0, 257 become 1 and so on.

4. Its considered an error when relying on integer overflow wrapping behavior.

5. To handles overflow you can use some methods that the standard library provides on primitive numeric types. \

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`
- Return the None value if there is overflow with the `checked_*` methods
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods
- Saturate at the value's minimum or maximum values with `saturating_*` methods

# Floating-Point Types

1. Floating-point numbers are numbers with decimal points.

2. Rust has 2 primitive types for floating-point numbers.

- f32 which is 32bit and it's a single-precision float.
- f64 which is 64bit and it's a double precision float.(Rust defaluts to using f64)
  > view code to see floating-point examples

# Numeric Operations

Rust supports the basic mathematical operations:

- addition
- subtraction
- multiplication
- division
- ramainder
  > view code to see numeric operation examples

# The Boolean Type

Booleans are one byte size and is specified using `bool`.

> view code to see boolean examples

# The Character Type

1. Character types in rust represent a Unicode Scalar Value, which means its more than just ASCII. Accented letters; Chinese, Japanese, Korean characters; emoji; and zero-width spaces are all valid `char` values.

> view code to see character type examples

## Compound Types

Compound Types group multiple values into one type.\
Rust has 2 primitive compound types, which are \

- tuples
- arrays

# The Tuple Type

1. tuples have a fixed length, once declared, they cannot grow or shrink in size.

2. A tuple is considered a single compound element. You can assign it a variety of types.

   > view code to see an example

3. The variable `tup` binds to the entire tuple and we can use pattern matching to destructure a tuple value and access whatever value you want to use.

   > view code to see an example

4. You can also access a tuple element directly by using a period(.) followed by the index of the value we want to access.The first index starts at 0.
   > view code to see an example

# The Array type

1. An array is another way of having a collection of multiple values. There are a few differences with arrays.

2. Every element of an array must have the same type. It also has a fix length. It can't grow or shrink.

   > view code for an example of an array

3. Arrays are good to use when you want

- your data allocated on the stack rather than on the heap.
- to ensure you always have a fixed number of elements.

4. Array's aren't as flexible as vector type. A vector is a similar collection type provided in the standard library that is allowed to grow and shrink in size. If you're ever unsure whether to use an array or a vector, you should use a vector.

> An example where you'd want to use an array rather than a vector is in a program thats needs to know the names of the months of the year. In this case it would be unlikely the program would need to add or remove 12 elements.

5. To specify an array's type use square brackets. Within the brackets specify the type (i32, u8, etc), and then the number of elements the array will have.

   > view code to see an example.

6. There is an alternative syntax to initiate an array. If you want to create an array where all elements are the same you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets.

   > view code to see an example

   **Accessing Array Elements**

7. To access and element in an array use its index in square brackets.

   > view code to see an example

   **Invalid Array Element Access**

If you try to access an index that doesn't exist, the program will result in a runtime error. The program will exit and will no long execute the remaining code. This check happen at runtime. This is a rust safety principle. Rust protects against invalid memory access by immediately exiting the program instead of allowing access to the memory.
