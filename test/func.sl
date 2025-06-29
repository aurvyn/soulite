+std:cout

mySimpleFunc |-> String
	"this is a function with no parameters and returns a string"

greet | String String -> String
theirName myName =
	"Hello {theirName}! My name is {myName}."

printGreet | String
"simple" =
	cout <| greet("Andy" "John")
"what" =
	cout <| greet("Beta" "Alpha")
else =
	cout << "Unknown input: " <| else

factorial | Int -> Int
0 = 1
1 = 1
n = n * factorial(n-1)

factorial_tail | Int Int -> Int
0 _ = 1
1 total = total
n total = factorial_tail(n-1 total*n)

main | [String]
[] = cout <| "Usage: <exe_name> [-h] <command> <..args>"
["fac" n] = cout <| factorial_tail(n.parse().unwrap() 1)
["-h" "fac"] =
	cout <| "Calculates the factorial.\nUsage: <exe_name> fac <Integer>"
args =
	args_str' args.join(" ")
	cout <| "invalid argument(s) `{args_str}`"
	main([])
