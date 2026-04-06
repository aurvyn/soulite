Item =
	name String
	desc String

Person<T> =
	name String
	age Z64
	items T[5]

	addItem item: 'T
		.items << item

	getItems :-> *T[5]
		*.items

Animal:
	growUp years: Z64 -> Z64

Person<T> => Animal
	growUp years: Z64 -> Z64
		.age += years
		.age
