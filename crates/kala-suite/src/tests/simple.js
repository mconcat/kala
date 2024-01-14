
//0
3+4;
7;

//1
(function() {
    if (true) {
        return 3;
    } else {
        return 4;
    }
})();
3;

//2
(function() {
    let x = 3;
    return x;
})();
3;

//3
(function(arg) {
    let x = 3;
    return x+arg;
})(4);
7;

//4
(function(arg1, arg2) {
    let local1 = 3;
    const local2 = 4;
    return local1+local2+arg1+arg2;
})(5, 6);
18;

//5
((function(arg1){
    return function(arg2) {
        return arg1+arg2;
    };
})(3))(4);
7;


//6
(function(obj) {
    return obj.x+obj.y;
})({x:3, y:4});
7;

//7
(function(){
    function f() {
        return 3;
    }
    return f();
})();
3;

//8
(function(){
    function f() {
        return 3;
    }
    return f;
})()();
3;

//9
(function(){
    console.log('console log is working');
})();
undefined;

//10
(() => 3)();
3;

//11
(() => {
    return 3;
})();
3;
