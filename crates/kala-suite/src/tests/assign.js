//1
(() => {
    let x = 3;
    x = 4;
    return x;
})();
4;

(() => {
    let x = { y: 3 };
    x.y = 4;
    return x.y;
})();
4;

(() => {
    let x = [3];
    x[0] = 4;
    return x[0];
})();
4;

(() => {
    let x = { y: [3] };
    x.y[0] = 4;
    return x.y[0];
})();
4;

(() => {
    let x = { y: { z: 3 } };
    x.y.z = 4;
    return x.y.z;
})();
4;

(() => {
    let x = { y: 3 };
    x.y += 1;
    return x.y;
})();
4;

(() => {
    let x = { y: 3 };
    x.y -= 1;
    return x.y;
})();
2;