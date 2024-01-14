(() => {
    let x = {
        get y() {
            return 3;
        }
    };
    return x.y;
})();
3;

(() => {
    let v = 3;
    let x = {
        get y() {
            return v;
        },
        set y(v_) {
            v = v_;
        }
    };
    x.y = 4;
    return x.y;
})();
4;