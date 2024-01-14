// Captured variables requires escape analysis, commented out for now

//12
(() => {
    const x = 3;
    function f() {
        return x;
    }
    //return f();
    return f()+1;
})();
4;
//13
(() => {
    function f() {
        return x;
    }
    const x = 3;
    return f();
})();
3;


//15
(() => {
    const o = {x:3};
    function f() {
        return o.x;
    }
    return f()+1;
})();
4;

//17
(() => {
    const o = {x:3};
    function f() {
        return o.x;
    }
    o.x = 4;
    return f()+1;
})();
5;

//18
(() => {
    function f() {
        return o.x;
    }
    const o = {x:3};
    return f()+1;
})();
4;

(() => {
    function f() {
        return o.x;
    }
    const o = {x:3};
    return f;
})()();
3;