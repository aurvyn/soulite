Item =
	name String
	desc String

Person<T> =
	name String
	age Int
	items T[5]

	add_item | T
	item: self.items << item

	get_items |-> @T[5]
		@items

Animal:
	grow_up | Int -> Int

Person => Animal
	grow_up | Int -> Int
	years:
		age += years
		age
