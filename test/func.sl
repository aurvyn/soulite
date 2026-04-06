+std:cout

mySimpleFunc :-> String
	"this is a function with no parameters and returns a string"

greet theirName myName: String String -> String?!Z64??
	(|(|)|)' <- theirName == "Tom" && myName == "Jerry" ; (|"Hello {theirName}! My name is {myName}."|)!

printGreet else: String
	result := "Unknown input: {else}"
	result = greet("Tom" "Jerry").unwrap_err().unwrap().unwrap() <- else == "simple" ; result
	result = greet("Alpha" "Beta").unwrap().unwrap() <- else == "what" ; result
	cout <| result

id x: 'T -> 'T
	x

\\ will complete once for loops are implemented
map nums func: [Z64] (Z64 -> Z64) -> [Z64]
	nums

square nums: [Z64] -> [Z64]
	map(nums _x*_x)

factorial n: Z64 -> Z64
	factorial(n-1) <- n > 1 ; 1

factorialTail n total: Z64 Z64 -> Z64
	factorialTail(n-1 total*n) <- n > 1 ; total

main args: [String]
	output := "invalid argument(s) `{args.join(" ")}`"
	output = "Usage: <exe_name> [-h] <command> <..args>" <- args.is_empty() ; output
	output = "{factorialTail(args[1].parse().unwrap() 1)}" <- args.len() == 2 && args[0] == "fac" ; output
	cout <| output
