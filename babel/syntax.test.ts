const syntax = require('./syntax')

test('works', () => {
    const code = `
const f=(a,b,...c)=>c.map(x => x(a, b))
`


    //console.log(JSON.stringify(syntax.check(code), null, 4))
    //console.log(code.length, syntax.protofy(code)[0].length)
})
test('zcf snippet test', () => {
    const code = `
    const start = zcf => {
        const refund = seat => {
            seat.exit();
            return 'The offer was accepted';
        };
        const makeRefundInvitation = () => zcf.makeInvitation(refund, 'getRefund');
    
        const publicFacet = {
            makeInvitation: makeRefundInvitation,
        };
        const creatorInvitation = makeRefundInvitation();
        return { creatorInvitation, publicFacet };
    };
    `
    //console.log(JSON.stringify(syntax.check(code), null, 4))
    //console.log(code.length, syntax.protofy(code)[0].length)
})

test('a+b', () => {
    const code = `
    (function f() {
        let a = 1;
        let b = 2;
        return a+b;
    })()
    `
    console.log(JSON.stringify(syntax.check(code), null, 4))
    console.log(syntax.protofy(code)[0].toString('hex'))
})

test('parameter function', () => {
    const code = `
    (function f(a) {
        return a;
    })(1)
    `
    console.log(JSON.stringify(syntax.check(code), null, 4))
    console.log(syntax.protofy(code)[0].toString('hex'))
})

test('objects', () => {
    const code = `
    (function f() {
        const o = {
            a: 1,
            b: 2,
        };
        o.x = 3;
        return o;
    })()
    `
    console.log(JSON.stringify(syntax.check(code), null, 4))
    console.log(syntax.protofy(code)[0].toString('hex'))
})