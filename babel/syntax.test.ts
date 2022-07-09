const syntax = require('./syntax')

test('works', () => {
    const code = `
const f=(a,b,...c)=>c.map(x => x(a, b))
`


    console.log(JSON.stringify(syntax.check(code), null, 4))
    console.log(code.length, syntax.protofy(code)[0].length)
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
    console.log(JSON.stringify(syntax.check(code), null, 4))
    console.log(code.length, syntax.protofy(code)[0].length)
})
