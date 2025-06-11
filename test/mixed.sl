; this is a comment by the way

; this imports `cout` from the standard library
+std-cout

myVar' "this is an immutable variable"

myMutable, "this is a mutable variable"

mySimpleFunc |-> String
	"this is a function with no parameters and returns a string"

; this function takes in two Strings and returns a String
greet | String String -> String
theirName myName =
	"Hello {theirName}! My name is {myName}."

; this function has a parameter but returns nothing
printGreet | String
"simple" =
	cout <| greet("Andy" "John")
"what" =
	cout <| greet("Beta" "Alpha")

; scenarios where pattern matching is more useful
factorial | Int -> Int
0 = 1
1 = 1
n = n * factorial(n-1)

; tail-recursive
factorial | Int Int -> Int
0 _ = 1
1 total = total
n total = factorial(n-1 total*n)

; struct declaration with generic type `T`
Person: T
	name String
	age Int
	items T[5]

	add_item | T
	item = self.items << item

	get_items |-> &T[5]
		&items

; trait declaration & implement for struct
Person => Animal
	grow_up | Int -> Int
	years =
		age += years
		age

init | [String]
[] = cout <| "Usage: main [-h] <command> <..args>"
["fac" n] = cout <| factorial(n 1)
["people"] =
	john, Person("John" 21 ["car keys" "credit card"])
	john.growUp(3)
	cout <| john.age ; should print "24"
["-h" "fac"] =
	cout <| "Calculates the factorial.\nUsage: main fac <Integer>"
args =
	cout <| "invalid input `main {args.join(" ")}`"
	init([])
