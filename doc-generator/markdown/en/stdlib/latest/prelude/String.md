# String

A collection of UTF-16 characters.

---

## Overview

Strings are a primitive datatype that contain a series of UTF-16 encoded characters.

To create a string use double quotes:

```misti
val greeting = "Hello there!"

// With type annotation
Str greeting = "Hello there!"
```

```md-warning
Misti doesn't allow string creation with single quotes <code>''</code>.
```

Strings can be concatenated with the plus `+` operator:

```misti
val name = "John"
val greeting = "Hello"

val fullGreeting = greeting + " " + name + "!"
//  fullGreeting = "Hello John!"
```

In the future, the language will support string concatenation and multi-line strings.

## String comparison

To check if two strings are the same use the equal-to `==` operator.

```misti
"Hello" == "Hello"  //: true
"Hello" == "hello"  //: false
```

To check if two strings are different use the not-equal-to `!=` operator.

```misti
"abc" != "123"  //: true
"xyz" != "xyz"  //: false
```

<br>

Comparing strings with `<`, `<=`, `>` & `>=` will compare each character of the string,
based on the UTF-16 value.

```misti
"a" < "b"     //: true
"a" < "A"     //: false

"aab" > "aac" //: false
"aab" < "aac" //: true
```

## Automatic String conversion

Misti __does not__ automatically convert other datatypes to String.

```misti
val sentence = "My age is: " + 20  // This will throw an error
//                           ^
// Cannot concatenate a String and a Number.
```

To do so use the method `.toStr()` to explicitly convert to String.

```misti
val sentence = "My age is: " + 20.toStr()  // ok
```

In the future, string interpolation will coerce datatypes into strings.


## Escape characters

Misti supports the following escape characters:

- `\"`

- `\\`
- `\n`
- `\r`
- `\t`
- `\b`

---

## API

Prelude's `Str` contains methods and functions created for their usage within Misti.

To access the underlying JavaScript methods, use the `js.String` module.


### Constructor

```misti
class String[T](T value)
where T -> Printable
```

Creates a string from `value`, by calling its `toStr` method.

#### Parameters

- `T value`: Any value that implements `Printable`, and therefore has a `toStr` method.

#### Examples

```misti
val age = String(20)  //: "20"
val condition = String(false) //: "false"
val numbers = Array(1, 2, 3) |> String //: "1,2,3"
```


## Properties

### length

```misti
Num length
```

Returns the number of UTF-16 code units in the string.

#### Example

```misti
val name = "John"
name.length        //: 4
```

<br>

---

## Methods

### charAt

```misti
fun charAt(Num position) -> Str?
```

Returns the UTF-16 code unit located at `position`.

<div class="padded">

#### Parameters

- `Num position`: An integer between `0` and the string's `length - 1`.

#### Return

- `Str?`: The character at `position`, or `None` if it's invalid

#### Description

In a string with `length` characters, this method returns the character (UTF-16 code point)
located at `position`, if it is true that `0 <= position < length`.

If `position` is out of range, this method will return `None`.

#### Examples

```misti
val name = "John Doe"

name.charAt(0)  //: Some("J")
name.charAt(1)  //: Some("o")
name.charAt(7)  //: Some("e")

name.charAt(-1) //: None
name.charAt(8)  //: None
```

If you are sure that the position is valid, you can use the `!!` operator to get
the value directly, instead of a `Maybe`.

```misti
val greeting = "Hello!"

name.charAt(4)!!  //: "o"
```

</div>


### concat

```misti
fun concat[any T](T... values) -> Str
where T -> Printable
```

Concatenates the calling string and `values`, and returns the result
as a new `Str`

<div class="padded">

#### Type parameters

- `any T`: Any datatype that implements `Printable`

#### Parameters

- `T... values`: Zero or more values of type `T`

#### Return

- `Str`: A new string with the values concatenated

#### Description

`concat` concatenates the callee, and any arguments passed to it in
a new string. The callee is not modified.

If the arguments don't have type `Str`, they are converted with their
`toStr` method.

#### Examples

```misti
val greeting = "Hello "

greeting.concat("world", "!") //: "Hello world!"
greeting.concat(123)          //: "Hello 123"
greeting.concat(3, 2, 2)      //: "Hello 322"
```

</div>



### includes

```misti
fun includes(Str searchString, Num pos = 0) -> Bool
```

Returns whether the current string contains `searchString`,
searching from position `pos`

<div class="padded">

#### Parameters

- `Str searchString`: The string to search for
- `Num pos = 0`: The position from where to start searching.

#### Return

`Bool`: `true` if searchString is found, `false` otherwise

#### Additional

If `searchString` is the empty string `""` this method returns `true`.

If `pos` is negative, the search starts from the first character.

#### Examples

```misti
val loremIpsum = "Lorem ipsum dolor sit amet"

loremIpsum.includes("ipsum")     //: true
loremIpsum.includes("ipsum", 10) //: false
loremIpsum.includes("ipsum", -5) //: true
```



</div>










