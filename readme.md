# Soulite

An experimental compiled programming language focused on making the syntax as compact as possible without sacrificing readability. The compiler toolchain is `SoulForge`, and the file extension is `.sl`. The following should be the ideal directory hierarchy for a simple project:

```
MyProject
- soulite.config
- src
  - main.soul
- forged
  - main
```

Anyhow, let's get into the syntax.

## Variables and Functions
```
; this is a comment by the way

; this imports `cout` from the standard library
+std-cout

myVar' "this is an immutable variable"

myMutable, "this is a mutable variable"

mySimpleFunc |-> String
	"this is a function with no parameters and returns a string"

; this function takes in two Strings and returns a String
greet | String String -> String
theirName myName:
	"Hello {theirName}! My name is {myName}."

; this function has a parameter but returns nothing
printGreet | String
"simple":
	cout <| greet("Andy" "John")
"what":
	cout <| greet("Beta" "Alpha")

; scenarios where pattern matching is more useful
factorial | Int -> Int
0: 1
1: 1
n: n * factorial(n-1)

; tail-recursive
factorial_tail | Int Int -> Int
0 _: 1
1 total: total
n total: factorial_tail(n-1 total*n)
```

A lot of syntax here is influenced by Haskell, so most of it would be self-explanatory if you know that language. However, a few things here are unique:

### `|->`
This is just saying that a function takes in no parameters and returns something.

### `<<`
Actually similar to how C++ behaves, this takes the item on the right hand side and "appends" it to the left hand side. It's shown in this example that we can append items to a list using this operator.

### `<|`
Similar to `<<`, except that it acts as a "closing version". This means that this operator does everything you expect `<<` to do, but also other operations that helps keep your codebase clean. For example, the following 2 snippets are pretty much identical in behavior:
> ```
> cout << "Hello World!\n"
> cout.flush()
> ```

> ```
> cout <| "Hello World!"
> ```

## Structs and Traits
```
; simple struct
Item =
	name String
	amount Int

; struct with generic type `T`
Person<T> =
	name String
	age Int
	items T[2]

	add_item | T
	item: self.items << item

	get_items |-> @T[2]
		@items

; simple trait
Animal:
	grow_up | Int -> Int

; implement trait for Person
Person => Animal
	grow_up | Int -> Int
	years:
		age += years
		age
```

This is where it gets similar to Rust. The `T` used here is a generic type, which would be inferred from the arguments passed into `Person`. Similarly, you can read `Person => Animal` as "implement Animal for Person".

## Main Function
```
main | [String]
[]: cout <| "Usage: <exe_name> [-h] <command> <..args>"
["fac" n]: cout <| factorial_tail(n.parse().unwrap() 1)
["people"]:
	john, Person("John" 21 ["car keys" "credit card"])
	john.grow_up(3)
	cout <| john.age ; should print "24"
["-h" "fac"]:
	cout <| "Calculates the factorial.\nUsage: <exe_name> fac <Integer>"
args:
	cout <| "invalid input `main {args.join(" ")}`"
	main([])
```

The main function acts as the "main" function that you might see in other languages. Here, it should always have `[String]` as a parameter.
