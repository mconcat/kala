// https://github.com/Moddable-OpenSource/moddable/blob/public/xs/sources/xsCommon.h
pub enum Opcode {
    // xs opcodes that I think I know what they are doing(based on their names)

    // literals
    True,
    False,
    Bigint1,
	Bigint2,
	// 1 byte, 2 bytes, 4 bytes of integer data inlined in the opcode
    Integer1,
	Integer2,
	Integer4,
	// StringX: X bytes of string length, n bytes of inlined string data(where n is the length of the string from that X bytes)
	String0, // 16 bytes of string data inlined in the opcode
    String1,
	String2,
	String4,
	// StringArchive does not copy the string, it just stores the address of the string in the opcodes... I guess. Strings are immutable in JS so maybe okay?
	// But commenting out anyway, do it later
	//StringArchive1,
	//StringArchive2,
	//StringArchive4,
	// XS_CODE_NUMBER stores full 8 bytes of a double inlined in the opcode,
	// But we are storing 64.64 fixed point numbers
	Number, // 64.64, 16 bytes

	// [Opcode::ArrayX, Array Type, X bytes of array capacity]]
    Array1,
	Array2,

	// https://github.com/Moddable-OpenSource/moddable/blob/public/xs/sources/xsAPI.c#L469
	// XS_CODE_OBJECT creates an empty object, but we also need to have a type information for the object
	// pointer/type ID referring to the type information, 4 bytes, will be followed by the object. 
	// the properties could be added after with SetProperty(At), with fixed offsets.
	Object,

    Null,	
    Undefined,

	// XS_CODE_FUNCTION creates a new "empty" function, which means there is no code nor scoped variables in it yet(only the name of the function).
    Function,

    // type conversion
    ToNumeric,
	ToString,
    
    // operations
	// arithmetic operations and comparison operations(add sub bitand lessthan etc) take 2 operands from the stack and push the result back to the stack
	Assign,
	Add,
    BitAnd,
	BitNot,
	BitOr,
	BitXor,
    Div,
    BitLeftShift,
	LessThan,
	LessThanEqual,
    // Increment,
	Neg, // XS_CODE_MINUS
    Sub,
	Mul,
	Mod,
    GreaterThan,
	GreaterThanEqual,	
	Pos, // XS_CODE_PLUS
    Not,
    // EQUAL,
	// NOT_EQUAL,
	BitUnsignedRightShift,
    StrictEqual,
	StrictNotEqual,
    TypeOf,
    BitRightShift,
    // Decrement,
	Pow,

    // control flow
	// Call allocates new stack frame, pushes arguments to the stack, and jumps to the function
    Call,
	// BranchX jumps to the address inlined in the opcode
    Branch1,
	Branch2,
	Branch4,
	// BranchChainX is for optional chaining. If the value on the stack is null or undefined, it jumps to the address inlined in the opcode, otherwise it continues to the next instruction, without consuming the value on the stack
	BranchChain1,
	BranchChain2,
	BranchChain4,
	// BranchCoalesceX is for nullish coalescing. If the value on the stack is NOT null or undefined, it jumps to the address inlined in the opcode, otherwise it continues to the next instruction, WITH consuming the value on the stack
	BranchCoalesce1,
	BranchCoalesce2,
	BranchCoalesce4,

	// BranchIfX jumps to the address inlined in the opcode if the value on the stack is true, otherwise it continues to the next instruction 
	BranchIf1,
	BranchIf2,
	BranchIf4,
	// BranchElseX jumps to the address inlined in the opcode if the value on the stack is false, otherwise it continues to the next instruction
	BranchElse1,
	BranchElse2,
	BranchElse4,
	// Branches based on current exception / return status. TODO
	BranchStatus1,
	BranchStatus2,
	BranchStatus4,
	// Throw sets the exception status and immediately jumps to the innermost try-catch block
    Throw,
	// retrieves the exception status??? 
	ThrowStatus,
	// Pushes the try-catch block to the exception handler stack.
    Catch1,
	Catch2,
	Catch4,
	// Uncatch re-throws the exception inside the catch block
    Uncatch,
	// Restores frame pointer, code pointer, stack pointer
    Return,
    ForOf,

	// resets the used frame values to undefined (after finishing the function call?)
	Unwind1,
	Unwind2,

    // data stack
	// Duplicate
	Dup,
	// Duplicate from with the index stored in the stack
	DupAt,
	// Pops
    Pop,
	// Swap the top 2 values on the stack
    Swap,

    // variables
	// Declares a closure / local variable as const
	ConstClosure1,
	ConstClosure2,
	ConstLocal1,
	ConstLocal2,

	// Declares as let idk 
	LetClosure1,
	LetClosure2,
	LetLocal1,
	LetLocal2,

	// Gets the value of the closure / local variable from the environment
	GetClosure1,
	GetClosure2,
	GetLocal1,
	GetLocal2,

	// Sets the value of the closure / local variable in the environment
	SetClosure1,
	SetClosure2,
    SetLocal1,
	SetLocal2,

	// Resets to uninitialized. Why do we need this?
	// ResetClosure1,
	// ResetClosure2,
	// ResetLocal1,
	// ResetLocal2,

	// Pulls the value of the closure / local variable from the environment and pushes it to the stack..?
	// The difference with Get seems like Get pushes the variable kind, whereas Pull takes it from the environment 
	PullClosure1,
	PullClosure2,
	PullLocal1,
	PullLocal2,

	// get/set 
    GetVariable,
    SetVariable,

	GetProperty,
	GetPropertyAt,
	GetResult,

    SetProperty,
	SetPropertyAt,
	SetResult,

	// takes variable from the environment and stores it to a slot
	// or the other way?? idk c is hard to read
    Store1,
	Store2,
	StoreArrow,

    CopyObject,

// probably needed but didnt understand yet
/* 
	RUN,
	RUN_1,
	RUN_2,
	RUN_4,
	RUN_TAIL,
	RUN_TAIL_1,
	RUN_TAIL_2,
	RUN_TAIL_4,

	// Seems like retrieving nth field from the object linked list...
	RETRIEVE_1,
	RETRIEVE_2,

*/

    ///////
    // /////
    // ////
    // /
    // /
    // /
    // //
    // /
    // 
    // 
    // 
    // 
    // 
    // 
    // 
    // 
    // 
    // 
    // 
    // XS opcodes i have no idea

    /* 
    NO_CODE = 0,
	ARGUMENT,
	AT,
	CODE_1,
	CODE_2,
	CODE_4,
	CODE_ARCHIVE_1,
	CODE_ARCHIVE_2,
	CODE_ARCHIVE_4,
	CONSTRUCTOR_FUNCTION,

	CURRENT,
	DEBUGGER,
	END,
	END_ARROW,
	END_BASE,
	END_DERIVED,
	ENVIRONMENT,
	EXCEPTION,

	EXTEND,
	FILE,


	FUNCTION_ENVIRONMENT,
	GLOBAL,
	HOST,
	IMPORT,
	IMPORT_META,
	IN,
	LINE,
	MODULE,
	NAME,
	PROGRAM_ENVIRONMENT,
	PROGRAM_REFERENCE,
	REFRESH_CLOSURE_1,
	REFRESH_CLOSURE_2,
	REFRESH_LOCAL_1,
	REFRESH_LOCAL_2,
	REGEXP,
	RESERVE_1,
	RESERVE_2,

	RETHROW,

	RETRIEVE_TARGET,
	RETRIEVE_THIS,

	SET_HOME,

	TARGET,
	TEMPLATE,
	TEMPLATE_CACHE,
	TO_INSTANCE,
	TRANSFER,


	USED_1,
	USED_2,
	USING,
	PROFILE,
	COUNT,



    /////
    /// /
    /// /
    /// /
    /// /
    /// /
    /// /
    /// /////
    /// /
    /// //
    /// /
    /// /
    /// 


    // illegal in jessie
    VOID,
	WITH,
	WITHOUT,
	YIELD,
    VAR_CLOSURE_1,
	VAR_CLOSURE_2,
	VAR_LOCAL_1,
	VAR_LOCAL_2,
    SET_SUPER,
	SET_SUPER_AT,
	SET_THIS,
    ARGUMENTS,
	ARGUMENTS_SLOPPY,
	ARGUMENTS_STRICT,
    INSTANCEOF,
	INSTANTIATE, // check
    FOR_IN,
    ASYNC_FUNCTION,
	ASYNC_GENERATOR_FUNCTION,
    AWAIT,
	BEGIN_SLOPPY,
	BEGIN_STRICT,
	BEGIN_STRICT_BASE,
	BEGIN_STRICT_DERIVED,
	BEGIN_STRICT_FIELD,
    EVAL,
	EVAL_ENVIRONMENT,
	EVAL_PRIVATE,
	EVAL_REFERENCE,
	EVAL_TAIL,
    THIS,
    DELETE_PROPERTY,
	DELETE_PROPERTY_AT,
	DELETE_SUPER,
	DELETE_SUPER_AT,
    CHECK_INSTANCE,
	CLASS,
    GET_SUPER,
	GET_SUPER_AT,
	GET_THIS,
	GET_THIS_VARIABLE,
    NEW,
	NEW_CLOSURE,
	NEW_LOCAL,
	NEW_PRIVATE_1,
	NEW_PRIVATE_2,
	NEW_PROPERTY,
	NEW_PROPERTY_AT,
	NEW_TEMPORARY,
    SET_PRIVATE_1,
	SET_PRIVATE_2,
    START_ASYNC,
	START_ASYNC_GENERATOR,
	START_GENERATOR,
    SUPER,
	SYMBOL,
    HAS_PRIVATE_1,
	HAS_PRIVATE_2,
    FOR_AWAIT_OF,
    GENERATOR_FUNCTION,
    GET_PRIVATE_1,
	GET_PRIVATE_2,
    */
}