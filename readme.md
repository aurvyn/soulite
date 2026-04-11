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

MY_CONST := "I'm a const variable"
myVar := "I'm a static variable"
myMutable ;= "I'm a mutable static variable"

\ adding an explicit type works too!
mySpecified: String = "I'm also a static variable"

\\ this is a multi-line doc-comment to be utilized by IDEs.
\\ no parameters, returns a string.
helloWorld :-> String
	"Hello world!"

\\ 2 parameters, returns a string.
greet theirName myName: String String -> String
	"Hello {theirName}! I'm {myName}."

\\ has a parameter, returns nothing.
\\ prints greeting to console based on input.
greetFriend creature: String
	output ;= "Unknown friend: {creature}"
	output = greet(creature "a crustacean too") <- creature == "Ferris" ; output
	output = greet(creature "in a repo") <- creature == "Octocat" ; output
	cout <| output
```

> [!TIP]
> It may also be helpful to read `<-` as `if`, since it's used in ternary operations and also in matching pattern guards. A shorter ternary operation would look like the following:
> ```
> response := "ready" <- is_ready ; "not yet"
> ```

Here are some more advanced examples:

```
factorial n: Z64 -> Z64
	factorial(n-1) <- n > 1 ; 1

\\ tail-recursive version.
factorialTail n total: Z64 Z64 -> Z64
	factorialTail(n-1 total*n) <- n > 1 ; total

\\ a simple struct.
Item =
	name: String
	amount; Z64

\\ now with generic type `t`!
Person =
	name: String
	age; Z64
	items; t[2]

	addItem: t
		.items << item

	getItems :-> *t[2]
		*.items

\\ a simple trait.
Animal:
	growUp years: Z64 -> Z64

\\ implement the Animal trait for Person struct...
Person => Animal
	growUp years: Z64 -> Z64
		.age += years
		.age
```

> [!IMPORTANT]
> To actually run some code in Soulite, you would want a `main` function:
> ```
> main args: [String]
> 	output ;= "invalid argument(s) `{args.join(" ")}`"
> 	output = "Usage: <exe_name> [-h] <command> <..args>" <- args.is_empty() ; output
> 	output = "{factorialTail(args[1].parse().unwrap() 1)}" <- args.len() == 2 && args[0] == "fac" ; output
> 	john ;= Person("John" 21 ["car keys" "credit card"])
> 	john.growUp(3)
> 	output = "{john.age}" <- args[0] == "people" ; output
> 	cout <| output
> ```

Check out the [wiki](https://github.com/aurvyn/soulite/wiki) for an in-depth exploration!

# Development

To use Soulite, compile and run by using cargo:

```bash
cargo run -- --help
```

To run the Rocq files for formal verification, generate the makefile and compile:

```bash
coq_makefile -f _CoqProject -o makefile
make
```

# Milestones

<details>
<summary>$\color{green}\text{Variables}$</summary>

>- [x] $\color{green}\text{Mutable}$
>- [x] $\color{green}\text{Immutable}$
>- [x] $\color{green}\text{Static}$
>- [x] $\color{green}\text{Const}$
</details>

<details>
<summary>$\color{green}\text{Comments}$</summary>

>- [x] $\color{green}\text{Single line}$
>- [x] $\color{green}\text{Multi line}$
</details>

<details>
<summary>$\color{green}\text{Functions}$</summary>

>- [x] $\color{green}\text{Parameter matching}$
><details>
><summary>$\color{green}\text{Pattern matching}$</summary>
>
>>- [x] $\color{green}\text{Literals}$
>>- [x] $\color{green}\text{Variables}$
>>- [x] $\color{green}\text{Wildcards}$
>>- [x] $\color{green}\text{Guards}$
>>- [x] $\color{green}\text{Generic types}$
></details>
</details>

<details>
<summary>$\color{green}\text{Structs}$</summary>

>- [x] $\color{green}\text{Fields}$
>- [x] $\color{green}\text{Generic types}$
><details>
><summary>$\color{green}\text{Methods}$</summary>
>
>>- [x] $\color{green}\text{Generic types}$
>>- [x] $\color{green}\text{Self field reference}$
></details>
</details>

<details>
<summary>$\color{green}\text{Traits}$</summary>

>- [x] $\color{green}\text{Method declaration}$
>- [x] $\color{green}\text{Generic types}$
><details>
><summary>$\color{green}\text{Struct implements}$</summary>
>
>>- [x] $\color{green}\text{Generic types}$
>>- [x] $\color{green}\text{Self field reference}$
></details>
</details>

<details>
<summary>$\color{grey}\text{Expressions}$</summary>

>- [x] $\color{green}\text{Ternary conditional}$
>- [x] $\color{green}\text{Anonymous functions}$
>- [x] $\color{green}\text{Option data type}$
>- [x] $\color{green}\text{Result data type}$
><details>
><summary>$\text{Data structures}$</summary>
>
>>- [x] $\color{green}\text{Arrays}$
>>- [x] $\color{green}\text{Lists}$
>>- [ ] $\text{Tuples}$
>>- [ ] $\text{HashSets}$
>>- [ ] $\text{HashMaps}$
></details>
><details>
><summary>$\text{Loops}$</summary>
>
>>- [ ] $\text{For loop}$
>>- [ ] $\text{While loop}$
>>- [ ] $\text{Infinite loop}$
></details>
</details>

<details>
<summary>$\color{grey}\text{Toolchain}$</summary>

>- [ ] $\texttt{soulite}&mdash;\text{completion of the milestones above}$
>- [ ] $\texttt{soulforge}&mdash;\text{package manager and build tool}$
>- [ ] $\texttt{soulfmt}&mdash;\text{code formatter}$
>- [ ] $\texttt{soulstd}&mdash;\text{standard library}$
>- [ ] $\texttt{souldocs}&mdash;\text{local docs webpage generator}$
>- [ ] $\texttt{soulsight}&mdash;\text{language server}$
>- [ ] $\texttt{soulsrc}&mdash;\text{local source code of standard library}$
</details>

<a href="https://www.star-history.com/#aurvyn/soulite&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=aurvyn/soulite&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=aurvyn/soulite&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=aurvyn/soulite&type=date&legend=top-left" />
 </picture>
</a>
