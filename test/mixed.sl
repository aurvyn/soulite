\ this is a comment by the way

\ this imports `cout` from the standard library
+std:cout

myConst :: "this is a const variable"  \ only valid in global scope

myVar :- "this is an immutable variable"

myMutable := "this is a mutable variable"

mySimpleFunc :-> String
	"this is a function with no parameters and returns a string"

\ this function takes in two Strings and returns a String
greet theirName myName: String String -> String
	"Hello {theirName}! My name is {myName}."

\ this function has a parameter but returns nothing
printGreet else: String
	result := "Unknown input: {else}"
	result = greet("Andy" "John") <- else == "simple" ; result
	result = greet("Alpha" "Beta") <- else == "what" ; result
	cout <| result

\ scenarios where pattern matching is more useful
factorial n: Z64 -> Z64
	factorial(n-1) <- n > 1 ; 1

\ tail-recursive
factorialTail n total: Z64 Z64 -> Z64
	factorialTail(n-1 total*n) <- n > 1 ; total

\ struct declaration with generic type `T`
Person<T> =
	name String
	age Z64
	items T[2]

	addItem item: 'T
		.items << item

	getItems :-> *'T[2]
		*.items

\ simple trait
Animal:
	growUp years: Z64 -> Z64

\ implement trait for struct
Person<T> => Animal
	growUp years: Z64 -> Z64
		.age += years
		.age

main args: [String]
	output := "invalid argument(s) `{args.join(" ")}`"
	output = "Usage: <exe_name> [-h] <command> <..args>" <- args.is_empty() ; output
	output = "{factorialTail(args[1].parse().unwrap() 1)}" <- args.len() == 2 && args[0] == "fac" ; output
	john := Person("John" 21 ["car keys" "credit card"])
	john.growUp(3)
	output = "{john.age}" <- args[0] == "people" ; output
	cout <| output
