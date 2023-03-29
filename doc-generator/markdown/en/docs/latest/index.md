# Welcome

Misti is _yet another_ toy language to replace JavaScript.

__Misti is indentation based.__

It's objectives are:

- Reduce compilation times using Rust.

- Improve code quality by making the language more expressive.
- Make the language easy to understand by using a consistent syntax (at the expense of familiarity).
- Integrate with existing TypeScript definitions by importing and exporting `.d.ts` files.
- Serve as a side project.

The purpose of the language is to address many of the limitations of JS.
To serve that end, __many concepts from JS may be completely omitted
or replaced with different syntax/semantics__.

Such things will be noted in the documentation where neccesary.

## Syntax summary

```misti
//
// Variables and constants
//
var aVariable = 20
val aConstant = 30
aVariable = 40
//            | semi colons not required


// Specify the datatype of a constant
Num aConstant = 30       //  <- `val` is optional

// Specify the datatype of a variable
Num var aVariable = 20   // <- `var` required

// You can assign the result of many operations to a variable
val roi =
    val income = someIncomeCalculation()
    val investment = 25000
    income / investment   // This will be the value of `roi`
```

```misti
//
// Basic datatypes
//
Num number = 40.12345
Bool boolean = true
Str string = "John Doe"
```

```misti
//
// Conditionals
//
if name == "John Doe" do
    val message = "Hello John"
    console.log(message)
else if name == "Mark" do
    console.log("Hi Mark!")
else
    console.log("Hello there")


// You can use conditionals as expressions
val response = if risk < 0.2 do "Go ahead" else "Don't"

// There is no ternary conditional
```

```misti
//
// Arrays
//
val dates = Array(1990, 1995, 2014, 2015, 2017)
//          | There isn't special syntax for array declaration
//            so you can't do `[1990, 1995, ...]`

val firstDate = dates.[0]
//                   | Notice the dot for access                  

dates.[4] = 2018
//   | Dot for mutation

// Array signature
Array[Num] dates = Array(1990, 1995, 2014, 2015, 2017)
//   | Square brackets are used for generics
//     instead of angle brackes.
```

```misti
//
// Tuples
//
val person = #("John", 30, true)

// Destructuring
var #(name, age, isMarried) = person

// Tuple signature
#(Str, Num, Bool) signature = #("John", 30, true)
```

```misti
//
// Loops
//
for key in object do
    console.log("key: {key}, value: {object.[key]}")


for value of array do
    console.log("value: {value}")


while condition do
    print("while")
```

```misti
//
// Functions
//
console.log("Enclose the parameters in parens")

add(10, 20)

// Named parameters
substring(input: "Hello, world!", start: 7, end: 12)

// Funtion declaration
fun add(Num x, Num y) -> Num =
    x + y


// Function with default value
fun calculate(Num price, Num discount = 0.0) =
    val total = price * (1.0 - discount)
    console.log("Your total is {total}$")


calculate(100, 0.25)  // "Your total is 75$"
calculate(100)        // "Your total is 100$"
```


```misti
//
// Objects
//

type Person = #{
    Str name,
    Num age,
}

val john = Person #{
    name: "John",
    age: 21,
}

// An object with arbitrary keys/values
val randomObject = #{
    key1: "Any value"
    key2: 322,
    key3: true,
    key4: #{
        key5: "zzz",
    },
    key6: Person #{
        name: "Sarah",
        age: 20,
    },
}
```


```misti
//
// Classes
//

// Declare a simple class
class Shape


// Classes can not be extended by default.
// To allow inheritance, use @open
@open
class Shape =
    // By default methods can not be overrided.
    // To allow it, use @open
    @open
    fun printName() =
        print("Generic Shape")


val shape = Shape()
//          | There's no `new` keyword, just call the class

shape.printName()   // "Generic Shape"


@open
class Rectangle(Num height, Num length) -> Shape() =
//             | Constructor parameters

    // Properties are always private
    val vertexCount = 4

    // Methods are private by default
    fun perimeter() -> Num =
        (height + length) * 2
    

    // To make a method public add @pub
    @pub
    fun area() -> Num =
        height * length
    

    // Method override
    @override
    fun printName() =
        print("A rectangle")


val rectangle = Rectangle(10, 20)
rectangle.area()       // 200
rectangle.printName()  // "A rectangle"


class Square(Num length) -> Rectangle(length, length) =
//                       | Inheritance

    @override
    fun printName() =
        console.log("A square")
    

    fun printInfo() =
        // Use $ to refer to methods/properties of the parent class
        console.log("A square with perimeter = {$perimeter()} and area = {$area()}")
```

```misti
//
// Null safety
//

// Operations that may fail return an Option value
fun divide(Int numerator, Int denominator) -> Option[Num] =
    if denominator == 0 do
        None    // Equivalent to `null`
    else
        Some(numerator / denominator)


val possibleResult = divide(10, 5)

if val Some(result) = possibleResult do
    print("The result of the division is {result}")
else
    print("Division by zero")


// `Type?` is syntax sugar for Option[Type]
Num? roi = divide(income, investment)
```

```misti
//
// Error handling
//

// A recoverable error
fun testVersionNumber(Str version) -> Result[Int, Str] =
    if version == "10" do
        Ok(10)
    else if version == "11" do
        Ok(11)
    else
        Err("Invalid version")


// Legacy try-catch (may change)
try problematicExpression() with
| Error(e) ->
    // do something
    // must return an expression
    10
```

```misti
//
// Pattern matching
//

match age with
| 10 ->
    // executes when age == 10
| 11 | 12 | 13 ->
    // when age == 11, 12 or 13
| _ ->
    // when none of the conditions match


Str? result = someOperation()
match result with
| Some(value) ->
    // value is a Str
    // do operations with v
| None ->
    // Handle empty return


Result[Num, Str] result = someOtherOperation()
match result with
| Ok(number) ->
    // number is Num
| Err(reason) ->
    // reason is Str


Result[Num, Str] result = someOtherOperation()
match result with
| Ok(number) if number > 18 ->
    // This matches if number > 18
| Ok(number) ->
    // This matches if number <= 18
| Err(reason) ->
    // reason is Str
```

```misti
//
// JSX
//

val element = &lt;div>This is JSX&lt;/div>
val list = items.map fun (item, count) {&lt;li key={count}>{item}&lt;/li>}
```









