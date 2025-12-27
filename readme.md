> [!NOTE]
> This language is still under active development!
> Some features may not be set in stone yet and new features are still being added.
> This project will enter the [alpha](https://en.wikipedia.org/wiki/Software_release_life_cycle#Alpha) phase once all [milestones](#milestones) are met.

# Soulite

An experimental compiled programming language focused on making the syntax as compact as possible without sacrificing readability.

```
\ this is a comment!

\ this imports `cout` from the standard library...
+std:cout

myConst :: "I'm a const variable"
myVar :- "I'm an immutable variable"
myMutable := "I'm a mutable variable"

\ adding an explicit type works too!
mySpecified: String = "I'm also a mutable variable"

\\ this is a multi-line doc-comment to be utilized by IDEs.
\\ no parameters, returns a string.
helloWorld |-> String
	"Hello world!"

\\ 2 parameters, returns a string.
greet | String String -> String
theirName myName:
	"Hello {theirName}! I'm {myName}."

\\ has a parameter, returns nothing.
\\ prints greeting to console based on input.
greetFriend | String
"Ferris":
	cout <| greet("Ferris" "a crustacean too")
"Octocat":
	cout <| greet("Octocat" "in a repo")
creature:
	cout << "Unknown friend: " <| creature
```

> [!TIP]
> Functions are capable of pattern matching, in a style similar to Haskell.
> ```
> <functionName> | [argTypes...] [-> returnTypes...]
> <patterns...> [<- guard]:
> 	<returnValues...>
> ```
> It may also be helpful to read `<-` as `if`, since it's also used in ternary operations:
> ```
> response := "ready" <- is_ready ; "not yet"
> ```

Here are some more advanced examples:

```
\\ a scenario where pattern matching is more useful!
factorial | Int -> Int
n <- n < 2: 1
n: n * factorial(n-1)

\\ tail-recursive version.
factorialTail | Int Int -> Int
n total <- n < 2: total
n total: factorialTail(n-1 total*n)

\\ a simple struct.
Item =
	name String
	amount Int

\\ now with generic type `T`!
Person<T> =
	name String
	age Int
	items T[2]

	addItem | T
	item: .items << item

	getItems |-> *T[2]
		*.items

\\ a simple trait.
Animal:
	growUp | Int -> Int

\\ implement the Animal trait for Person struct...
Person<T> => Animal
	growUp | Int -> Int
	years:
		.age += years
		.age
```

> [!IMPORTANT]
> To actually run some code in Soulite, you would want a `main` function:
> ```
> main | [String]
> []: cout <| "Usage: <programName> [-h] <command> <..args>"
> ["fac" n]: cout <| factorialTail(n.parse().unwrap() 1)
> ["people"]:
> 	john := Person("John" 21 ["car keys" "credit card"])
> 	john.growUp(3)
> 	cout <| john.age  \ should print "24"
> ["-h" "fac"]:
> 	cout <| "Calculates the factorial.\nUsage: <exe_name> fac <Integer>"
> args:
> 	cout <| "invalid input `{args.join(" ")}`"
> 	main([])
> ```

Check out the [wiki](https://github.com/aurvyn/soulite/wiki) for an in-depth exploration!

# Milestones

<details>
<summary>$${\color{green}\text{Variables}}$$</summary>

>- [x] $${\color{green}\text{Mutable}}$$
>- [x] $${\color{green}\text{Immutable}}$$
>- [x] $${\color{green}\text{Static}}$$
>- [x] $${\color{green}\text{Const}}$$
</details>

<details>
<summary>$${\color{green}\text{Comments}}$$</summary>

>- [x] $${\color{green}\text{Single line}}$$
>- [x] $${\color{green}\text{Multi line}}$$
</details>

<details>
<summary>$${\color{green}\text{Functions}}$$</summary>

>- [x] $${\color{green}\text{Parameter matching}}$$
><details>
><summary>$${\color{green}\text{Pattern matching}}$$</summary>
>
>>- [x] $${\color{green}\text{Literals}}$$
>>- [x] $${\color{green}\text{Variables}}$$
>>- [x] $${\color{green}\text{Wildcards}}$$
>>- [x] $${\color{green}\text{Guards}}$$
>>- [x] $${\color{green}\text{Generic types}}$$
></details>
</details>

<details>
<summary>$${\color{green}\text{Structs}}$$</summary>

>- [x] $${\color{green}\text{Fields}}$$
>- [x] $${\color{green}\text{Generic types}}$$
><details>
><summary>$${\color{green}\text{Methods}}$$</summary>
>
>>- [x] $${\color{green}\text{Generic types}}$$
>>- [x] $${\color{green}\text{Self field reference}}$$
></details>
</details>

<details>
<summary>$${\color{green}\text{Traits}}$$</summary>

>- [x] $${\color{green}\text{Method declaration}}$$
>- [x] $${\color{green}\text{Generic types}}$$
><details>
><summary>$${\color{green}\text{Struct implements}}$$</summary>
>
>>- [x] $${\color{green}\text{Generic types}}$$
>>- [x] $${\color{green}\text{Self field reference}}$$
></details>
</details>

<details>
<summary>$${\color{grey}\text{Expressions}}$$</summary>

>- [x] $${\color{green}\text{Ternary conditional}}$$
>- [ ] $${\color{grey}\text{Anonymous functions}}$$
>- [x] $${\color{green}\text{Option data type}}$$
>- [x] $${\color{green}\text{Result data type}}$$
</details>

<details>
<summary>$${\color{grey}\text{Toolchain}}$$</summary>

>- [ ] $${\color{grey}\text{soulite&mdash;completion of the milestones above}}$$
>- [ ] $${\color{grey}\text{soulforge&mdash;package manager and build tool}}$$
>- [ ] $${\color{grey}\text{soulfmt&mdash;code formatter}}$$
>- [ ] $${\color{grey}\text{soulstd&mdash;standard library}}$$
>- [ ] $${\color{grey}\text{souldocs&mdash;local docs webpage generator}}$$
>- [ ] $${\color{grey}\text{soulsight&mdash;language server}}$$
>- [ ] $${\color{grey}\text{soulsrc&mdash;local source code of standard library}}$$
</details>
