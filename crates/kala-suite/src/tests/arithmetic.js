//1
(() => {
    return 3+4;
})();
7;

//2
(() => {
    return 3+4*6%7;
})();
6;

//3
(() => {
    return true ? 3 : 4;
})();
3;

//4
(() => {
    return true && false;
})();
false;

//5
(() => {
    return true || false;
})();
true;

//6
(() => {
    return 3 === 4;
})();
false;

//7
(() => {
    return 3 !== 4;
})();
true;

//8
(() => {
    return 3 < 4;
})();
true;

//9
(() => {
    return 3 <= 4;
})();
true;

//10
(() => {
    return 3 > 4;
})();
false;

//11
(() => {
    return 3 >= 4;
})();
false;