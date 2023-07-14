# Interpreter

simple interpreter for jessie.

some assumptions:

- no inheritance, no prototypes. the set of possible prototypes are predetermined by the language specification(Array, Object, etc...) and cannot be extended by the user.
- therefore no 'holed' array - any hole can be safely assumed as undefined.
- the type checker will optionally provide the inferred known type of the variables.
- as we can know the type of the variable in prior, the value could be accessed with a known offset.
- as scoping logic already knows the all the variables declared in a scope, we can access the variables of in a scope with a known offset.
- as borrow inference already knows when the variables will be dropped(planned), we can omit the `Rc` for them. for the vars that cannot be determined, we can fallback to `Rc`. no gc, but rc cycles will be handled by a separate cycle detector(planned).
- timer functions(`setTimeout`, `setInterval`, `setImmediate`) will be considered to have the same behavior with Node.js. in blockchain contexts, `setTimeout` and `setInterval` will put the function call transaction in the virtual mempool, and `setImmediate` will invoke the transaction immediately. btw, the interpreter is working on single threaded event loop model, where the event queue takes transactions from both internal events and external transactions. also for the `setTimeout` and `setInterval`, the contract which invoked the functions need to pay the incurring gas price.
- Futures will work as something similar with above
- when we have a spread operator in objects(`{...someObject}`) it could be either a field-copying operation, or an attempt to emulate class inheritance. 
- the class inheritance could be emulated in the following way:
```js
// ECMA
// prototype chain
// new Class() => Class => Parent => Object => null
class Class extends Parent {
    constructor(a) {
        super(a)
        this.a = a
    }

    get a() {
        return this.a
    }

    set a(a) {
        this.a = a
    }

    f() {
        doSomething()
    }
}

// Jessie
// We still have Object as a valie prototype, as it is not a runtime definition.
// only the new Class() => Class => Parent part should be translated
const Class = (a) => {
    let parent = Parent(a);
    let self = {
        ...parent,
        a,
        get a() {
            return self.a
        },
        set a(a) {
            self.a = a
        }
        f() {
            doSomething()
        }
    };
    return self;
}

// note the `...parent` comes in the first; when there are conflicting names, the order matters.
```

- so anyway in this case the spread parent object can have a large size and would be more efficient to be not flatten, especially when other functions takes `T extends Parent` type(they are expecting the memory layout of Parent). take care about this case in type checker and determine whether to flatten or embed.
