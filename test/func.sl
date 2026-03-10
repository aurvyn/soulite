+std:cout

mySimpleFunc |-> String
	"this is a function with no parameters and returns a string"

greet | String String -> String?!Z64??
"Tom" "Jerry":
	(|(|)|)'
theirName myName:
	(|"Hello {theirName}! My name is {myName}."|)!

printGreet | String
"simple":
	result := greet("Tom" "Jerry").unwrap_err().unwrap().unwrap()
	cout <| result
"what":
	result := greet("Alpha" "Beta").unwrap().unwrap()
	cout <| result
else:
	cout << "Unknown input: " <| else

id<T> | T -> T
x: x

\\ will complete once for loops are implemented
map | [Z64] (Z64 -> Z64) -> [Z64]
nums func: nums

square | [Z64] -> [Z64]
nums: map(nums _x*_x)

factorial | Z64 -> Z64
n <- n < 2: 1
n: n * factorial(n-1)

factorialTail | Z64 Z64 -> Z64
0 _: 1
1 total: total
n total: factorialTail(n-1 total*n)

main | [String]
[]: cout <| "Usage: <exe_name> [-h] <command> <..args>"
["fac" n]: cout <| factorialTail(n.parse().unwrap() 1)
["-h" "fac"]:
	cout <| "Calculates the factorial.\nUsage: <exe_name> fac <Integer>"
args:
	args_str :- args.join(" ")
	cout <| "invalid argument(s) `{args_str}`"

	main([])
