((state) => {
    return state.get('value') ?? 0;
})(state);
0;

((state) => {
    return state.set('value', state.get('value') ?? 0 + 1);
})(state);
undefined;

((state) => {
    return state.get('value') ?? 0;
})(state);
1;