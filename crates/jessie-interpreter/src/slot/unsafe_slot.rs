// UnsafeSlot is a union of all possible values that can be stored in a slot

/*
A `slot` is a possible word-sized value that can be stored anywhere in the memory, either heap, stack or code.
Invalid access should have be prohibited by the type/lifetime checker
We use the following convention:
- Primitive values that have a fixed set of possible values(like `undefined`, `null`, `boolean`, `number`(short), `bigint`(short), `string`(short)) are stored directly in the slot.
- Primitive values that have a variable set of possible values(like `number`(long), `bigint`(long), `string`(long)) have the word-sized header, composed of the length and the pointer to the actual data(raw array of {datatype}_data). We can safely store the length in the header, instead of treating the data as a vector, because primitive values are immutable and always copied when modified.
- Reference values(like `Array`, `Object`, `Function`) are mutable and referenced, so they are stored as a pointer to the actual data.

We use the following naming convention:
- `Short` means that the value is stored directly in the slot, with a fixed size(16bit tag + 48bit value)
- `Header` is the word-sized header of a dynamic length value. It contains the length, followed by a series of corresponding `Data` words.
- `Data` is the word-sized data of a dynamic length value.
- `Pointer` is the word-sized pointer to the actual data of a reference value. 
 */
#[repr(C)]
union UnsafeSlot {
	// primitive values
	uninitialized: Tag<Uninitialized>, // tag
	undefined: Tag<Undefined>, // tag
	null: Tag<Null>, // tag
	boolean: Tag<Boolean>, // tag | 32bit boolean
	// number
	number_short: Tag<NumberShort>, // 16bit tag | 48bit number
	number_pointer: Tag<NumberPointer>, // tag 
	number_data_integer: NumberDataInteger, // 64bit integer part
	number_data_fraction: NumberDataFraction, // 64bit fraction part
	// bigint
	bigint_short: Tag<BigintShort>, // 16bit tag | 48bit number
	bigint_pointer: Tag<BigintPointer>, // tag | 16bit bigint length | 32bit bigint pointer
	bigint_data: BigintData, // 64bit bigint data(repeated)
	// string
	string_pointer: Tag<StringPointer>, // tag | 16bit string length | 32bit string pointer
	string_data: StringData, // 64bit string data

	// reference values
	object_pointer: Tag<ObjectPointer>, // 16bit tag | 16bit property size | 32bit property pointer // required

	// either one of type metadata is present
	object_metadata: ObjectMetadata, 
	/* 
	object_metadata_dynamic: ObjectMetadataDynamic, // 32bit dictionary pointer | 32bit hidden interface pointer // present if object type is not statically known
	object_metadata_array: ObjectMetadataArray, // 32bit array element type or typed array info // present if array object
	object_metadata_function: ObjectMetadataFunction, // 32bit function pointer | 32bit lexical scope pointer // present if function object	
	*/

	object_lifetime: ObjectLifetime, // 32bit reference count // present if object lifetime is not statically known

	// 
}

impl UnsafeSlot {
	pub fn new() -> Self {
		unsafe { std::mem::zeroed() }
	}

	pub fn set_uninitialized(&mut self) {
		unsafe { self.uninitialized = Uninitialized { tag: 0b_0000_0000, _empty0: 0, _empty1: 0 } }
	}

	pub fn set_undefined(&mut self) {
		unsafe { self.undefined = Undefined { tag: 0b_0000_0001, _empty0: 0, _empty1: 0 } }
	}

	pub fn set_null(&mut self) {
		unsafe { self.null = Null { tag: 0b_0000_0011, _empty0: 0, _empty1: 0 } }
	}

	pub fn set_number_short(&mut self, number_short: u32) {
		unsafe { self.number_short = NumberShort { tag: 0b_0000_0100, _empty: 0, number_short } }
	}

	pub fn set_boolean(&mut self, boolean: bool) {
		unsafe { self.boolean = Boolean { tag: 0b_0000_0101, _empty: 0, boolean: boolean as u32 } }
	}

	pub fn set_number_integer(&mut self, number_integer: i64) {
		unsafe { self.number_integer = NumberInteger { number_integer } }
	}

	pub fn set_number_fraction(&mut self, number_fraction: u64) {
		unsafe { self.number_fraction = NumberFraction { number_fraction } }
	}

	pub fn set_bigint_short(&mut self, bigint_short: u32) {
		unsafe { self.bigint_short = BigintShort { tag: 0b_0000_0110, _empty: 0, bigint_short } }
	}

	pub fn set_bigint_header(&mut self, bigint_length: u16, bigint_pointer: u32) {
		unsafe { self.bigint_header = BigintHeader { tag: 0b_0000_0110, bigint_length, bigint_pointer } }
	}

	pub fn set_bigint_data(&mut self, bigint_data: u64) {
		unsafe { self.bigint_data = BigintData { bigint_data } }
	}

	pub fn set_string_header(&mut self, string_length: u16, string_pointer: u32) {
		unsafe { self.string_header = StringHeader { tag: 0b_0000_0111, string_length, string_pointer } }
	}

	pub fn set_string_data(&mut self, string_data: u64) {
		unsafe { self.string_data = StringData { string_data } }
	}

	pub fn set_object_header(&mut self, property_size: u16, property_pointer: u32) {
		unsafe { self.object_header = ObjectHeader { tag: 0b_0000_1000, property_size, property_pointer } }
	}

	pub fn set_object_metadata_dynamic(&mut self, dictionary_pointer: u32, hidden_interface_pointer: u32) {
		unsafe { self.object_metadata = ObjectMetadataDynamic { dictionary_pointer, hidden_interface_pointer } }
	}

	pub fn set_object_metadata_array(&mut self, array_element_type: u32) {
		unsafe { self.object_metadata = ObjectMetadataArray { array_element_type } }
	}

	pub fn set_object_metadata_function(&mut self, function_pointer: u32, lexical_scope_pointer: u32) {
		unsafe { self.object_metadata = ObjectMetadataFunction { function_pointer, lexical_scope_pointer } }
	}

	pub fn set_object_lifetime(&mut self, reference_count: u32) {
		unsafe { self.object_lifetime = ObjectLifetime { reference_count } }
	}
}

#[repr(C)]
union Tag<T: Sized> {
	tag: u64,
	data: T,
}

#[repr(C)]
struct Uninitialized {}

#[repr(C)]
struct Undefined {}

#[repr(C)]
struct Null {}

#[repr(C)]
struct Boolean {
	boolean: u64,
}

#[repr(C)]
struct NumberShort {
	number: u64, // u48
}

#[repr(C)]
struct NumberInteger {
	number_integer: i64,
}

#[repr(C)]
struct NumberFraction {
	number_fraction: u64,
}

#[repr(C)]
struct BigintShort {
	bigint_short: u64, // u48
}

#[repr(C)]
struct BigintHeader {
	_padding: u32,
	bigint_pointer: u32,
}

#[repr(C)]
struct BigintData {
	bigint_data: [u8; 8],
}

#[repr(C)]
struct StringHeader {
	_padding: u16,
	string_length: u16,
	string_pointer: u32,
}

#[repr(C)]
struct StringData {
	string_data: [u8; 8],
}

#[repr(C)]
union ObjectMetadata {
	object_metadata_dynamic: ObjectMetadataDynamic,
	object_metadata_array: ObjectMetadataArray,
	object_metadata_function: ObjectMetadataFunction,
}

#[repr(C)]
struct ObjectMetadataDynamic {
	dictionary_pointer: u32,
	hidden_interface_pointer: u32,
}

#[repr(C)]
struct ObjectMetadataArray {
	_padding: u32,
	array_pointer: u32,
}

#[repr(C)]
struct ObjectMetadataFunction {
	function_pointer: u32,
	lexical_scope_pointer: u32,
}

#[repr(C)]
struct ObjectLifetime {
	owner_pointer: u32, 
	reference_count: u32,
}

#[repr(C)]
struct ObjectPointer {
	_padding: u32,
	property_pointer: u32,
}

#[repr(C)]
struct ObjectHeader {
	
}

#[repr(C)]
struct PropertyHeader {

}