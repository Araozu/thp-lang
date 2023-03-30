# Console

## Overview

The console module contains functions useful for debugging.

## Functions

### `fun log(Any ...objects)`

Outputs a message to the console of the system, with a new line.

When called without parameters, it prints just the new line.

```
Console.log()  // Prints: `\n`
```

When called with one or more parameters, it first converts those
parameters to strings, joins them with white space, then prints them.

```
Console.log("message")  // Prints: `message\n`

Console.log("message", "value")  // Prints: `message value\n`
```
