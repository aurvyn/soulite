Item =
	name: String
	amount; N32

Person =
	name: String
	age; N8
	items; t[5]

	addItem item: t
		.items << item

	getItems :-> *t[5]
		*.items

Animal:
	growUp years: N8 -> N8

Person => Animal
	growUp years: N8 -> N8
		.age += years
		.age
