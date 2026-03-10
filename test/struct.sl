Item =
	name String
	desc String

Person<T> =
	name String
	age Z64
	items T[5]

	add_item | T
	item: .items << item

	get_items |-> *T[5]
		*.items

Animal:
	grow_up | Z64 -> Z64

Person<T> => Animal
	grow_up | Z64 -> Z64
	years:
		.age += years
		.age
