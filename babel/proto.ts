import * as types from '@babel/types';
import * as ast from './gen/ast';

/*
Program [ Statement | ModuleDeclaration ]

Statement = ExpressionStatement | BlockStatement | EmptyStatement | ReturnStatement | LabeledStatement | BreakStatement | ContinueStatement | IfStatement | SwitchStatement | ThrowStatement | TryStatement | WhileStatement | ForStatement | ForOfStatement | Declaration

Declaration = FunctionDeclaration | VariableDeclaration 

Expression = Identifier | Literal | ArrowFunctionExpression | ArrayExpression | ObjectExpression | RecordExpression | TupleExpression | FunctionExpression | UnaryExpression | UpdateExpression | BinaryExpression | AssignmentExpression | LogicalExpression | MemberExpression | OptionalMemberExpression | ConditionalExpression | CallExpression | OptionalCallExpression | SequenceExpression | ParenthesizedExpression 

ModuleDeclaration = ImportDeclaration | ExportNamedDeclaration | ExportDefaultDeclaration | ExportAllDeclaration | 
*/
/*
export type TSType =
  | types.TSAnyKeyword
  | types.TSBooleanKeyword
  | types.TSBigIntKeyword
  // | types.TSNeverKeyword
  | types.TSNullKeyword
  | types.TSNumberKeyword
  | types.TSObjectKeyword
  | types.TSStringKeyword
  // | TSSymbolKeyword
  | types.TSUndefinedKeyword
  | types.TSUnknownKeyword
  | types.TSVoidKeyword
  | types.TSFunctionType
  | types.TSTypeReference
  // | TSTypePredicate
  // | TSTypeQuery
  | types.TSTypeLiteral
  | types.TSArrayType
  | types.TSTupleType
  | types.TSOptionalType
  | types.TSRestType
  | types.TSUnionType
  // | TSIntersectionType
  | types.TSConditionalType
  // | TSInferType
  | types.TSParenthesizedType
  // | TSTypeOperator
  | types.TSLiteralType
  // | types.TSExpressionWithTypeArguments
*/
/*
export function ToTSType(node: types.TSType): TSType {
    switch (node.type) {
    case 'TSAnyKeyword':
        return ToTSAnyKeyword(node)
    case 'TSBooleanKeyword':
        return ToTSBooleanKeyword(node)
    case 'TSBigIntKeyword':
        return ToTSBigIntKeyword(node)
    // case 'TSNeverKeyword':
    //     return ToTSNeverKeyword(node)
    case 'TSNullKeyword':
        return ToTSNullKeyword(node)
    case 'TSNumberKeyword':
        return ToTSNumberKeyword(node)
    case 'TSObjectKeyword':
        return ToTSObjectKeyword(node)
    case 'TSStringKeyword':
        return ToTSStringKeyword(node)
    // case 'TSSymbolKeyword':
    //     return ToTSSymbolKeyword(node)
    case 'TSUndefinedKeyword':
        return ToTSUndefinedKeyword(node)
    case 'TSUnknownKeyword':
        return ToTSUnknownKeyword(node)
    case 'TSVoidKeyword':
        return ToTSVoidKeyword(node)
    case 'TSFunctionType':
        return ToTSFunctionType(node)
    case 'TSTypeReference':
        return ToTSTypeReference(node)
    // case 'TSTypePredicate':
    //     return ToTSTypePredicate(node)
    // case 'TSTypeQuery':
    //     return ToTSTypeQuery(node)
    case 'TSTypeLiteral':
        return ToTSTypeLiteral(node)
    case 'TSArrayType':
        return ToTSArrayType(node)
    case 'TSTupleType':
        return ToTSTupleType(node)
    case 'TSOptionalType':
        return ToTSOptionalType(node)
    case 'TSRestType':
        return ToTSRestType(node)
    case 'TSUnionType':
        return ToTSUnionType(node)
    // case 'TSIntersectionType':
    //     return ToTSIntersectionType(node)
    case 'TSConditionalType':
        return ToTSConditionalType(node)
    // case 'TSInferType':
    //     return ToTSInferType(node)
    case 'TSParenthesizedType':
        return ToTSParenthesizedType(node)
    // case 'TSTypeOperator':
    //     return ToTSTypeOperator(node)
    case 'TSLiteralType':
        return ToTSLiteralType(node)
    default: 
        throw 'Unrecognized TSType'
    }
}
*/

/*
export type Statement =
  | BlockStatement
  | BreakStatement
  | ContinueStatement
  | EmptyStatement
  | ExpressionStatement
  | ForStatement
  | ForOfStatement
  | FunctionDeclaration
  | IfStatement
  | ReturnStatement
  | SwitchStatement
  | ThrowStatement
  | TryStatement
  | VariableDeclaration
  | WhileStatement
  */
  //TODO
  /*
  //| ExportAllDeclaration
  //| ExportDefaultDeclaration
  | ExportNamedDeclaration
  | ImportDeclaration
  | DeclareFunction
  | DeclareInterface
  //| DeclareModuleExports
  | DeclareTypeAlias
  //| DeclareOpaqueType
  | DeclareVariable
  | DeclareExportDeclaration
  // | DeclareExportAllDeclaration
  | TSDeclareFunction
  | TSInterfaceDeclaration
  | TSTypeAliasDeclaration
  | TSEnumDeclaration
  | TSModuleDeclaration
  | TSImportEqualsDeclaration
  | TSExportAssignment
  | TSNamespaceExportDeclaration;
  */

export function ToStatement(node: types.Statement): ast.Statement {
    const result: ast.Statement = {} as ast.Statement
    switch (node.type) {
    case 'BlockStatement':
        result.blockStatement = ToBlockStatement(node);
        break
    case 'BreakStatement':
        result.breakStatement = ToBreakStatement(node);
        break
    case 'ContinueStatement':
        result.continueStatement = ToContinueStatement(node);
        break
    case 'ExpressionStatement':
        result.expressionStatement = { expression: ToExpression(node.expression) };
        break
    case 'ForStatement':
        result.forStatement = ToForStatement(node)
        break
    case 'ForOfStatement':
        result.forOfStatement = ToForOfStatement(node)
        break
    case 'FunctionDeclaration':
        result.functionDeclaration = ToFunctionDeclaration(node)
        break
    case 'IfStatement':
        result.ifStatement = ToIfStatement(node)
        break
    case 'ReturnStatement':
        result.returnStatement = ToReturnStatement(node)
        break
    case 'SwitchStatement':
        result.switchStatement = ToSwitchStatement(node)
        break
    case 'ThrowStatement':
        result.throwStatement = ToThrowStatement(node)
        break
    case 'TryStatement':
        result.tryStatement = ToTryStatement(node)
        break
    case 'VariableDeclaration':
        result.variableDeclaration = ToVariableDeclaration(node)
        break
    case 'WhileStatement':
        result.whileStatement = ToWhileStatement(node)
        break
    default:
        throw 'Node type not recognized by Tessie grammar' 
    }
    return result
}

export function ToBlockStatement(node: types.BlockStatement): ast.BlockStatement {
    return {
        body: node.body.map(ToStatement),
    }
}

export function ToBreakStatement(node: types.BreakStatement): ast.BreakStatement {
    return {}
}

export function ToContinueStatement(node: types.ContinueStatement): ast.ContinueStatement {
    return {}
}

const MapDeclarationKind = {
    'const': ast.DeclarationKind.CONST,
    'let': ast.DeclarationKind.LET,
}

export function ToForStatement(node: types.ForStatement): ast.ForStatement {
    if (node?.init?.type !== 'VariableDeclaration') {
        throw 'ForStatement init must be a VariableDeclaration'
    }

    if (node.init.kind !== 'let' && node.init.kind !== 'const') {
        throw 'For loop must use let or const'
    }

    if (node.init.declarations.length !== 1) {
        // TODO: support multiple decls
        throw 'ForStatement init must have exactly one declaration'
    }

    const declaration = node.init.declarations[0]

    const init = {
        initDeclaration: node.init?.type === 'VariableDeclaration' ? ToVariableDeclaration(node.init) : undefined,
    }

    return {
        kind: MapDeclarationKind[node.init.kind],
        init: ToVariableDeclarator(declaration),
        test: node.test ? ToExpression(node.test) : undefined,
        update: node.update ? ToExpression(node.update) : undefined,
        body: ToStatement(node.body),
    }
}

export function ToForOfStatement(node: types.ForOfStatement): ast.ForOfStatement {
    /*
    const left = {
        leftDeclaration: node.left.type === 'VariableDeclaration' ? ToVariableDeclaration(node.left) : undefined,
        leftLval: node.left.type !== 'VariableDeclaration' ? ToLVal(node.left) : undefined
    }

    return {
        ...left,
        right: ToExpression(node.right),
        body: ToStatement(node.body),
    }
    */
   throw 'Not implemented forof'
}

// export type Pattern = AssignmentPattern | ArrayPattern | ObjectPattern;

export function ToParameterPattern(node: types.Pattern | types.PatternLike | null): ast.ParameterPattern {
    const result = {} as ast.ParameterPattern
    if (!node) {
        throw 'Node is undefined'
    }
    switch (node.type) {
        case "Identifier":
            result.pattern = { identifier: ToIdentifier(node) } as ast.Pattern
            break
        case "ArrayPattern":
            result.pattern = { binding: { array: ToArrayPattern(node) } } as ast.Pattern
            break
        case "ObjectPattern":
            result.pattern = { binding: { object: ToObjectPattern(node) } } as ast.Pattern
            break
        case "AssignmentPattern":
            result.optional = ToAssignmentPattern(node)
            break
        case "RestElement": 
            result.pattern = { isRest: true } as ast.Pattern
            switch (node.argument.type) {
                case "Identifier":
                    result.pattern.identifier = ToIdentifier(node.argument)
                    break
                case "ArrayPattern":
                    result.pattern.binding = { array: ToArrayPattern(node.argument) } as ast.BindingPattern
                    break
                case "ObjectPattern":
                    result.pattern.binding = { object: ToObjectPattern(node.argument) } as ast.BindingPattern
                    break
                case "AssignmentPattern":
                    result.optional = ToAssignmentPattern(node.argument)
                    break
                case "RestElement":
                    throw 'i dont know what to do with this'
            }
            break
        default:
            throw 'Node type not recognized by Tessie grammar'
    }

    return result
}

export function ToAssignmentPattern(node: types.AssignmentPattern): ast.OptionalPattern {
    if (node.left.type !== 'Identifier') {
        throw 'AssignmentPattern left must be an Identifier'
    }

    return {
        identifier: ToIdentifier(node.left),
        expression: ToExpression(node.right), 
    }
}

export function ToArrayPattern(node: types.ArrayPattern): ast.ArrayPattern {
    return {
        elements: node.elements.map(ToParameterPattern),
    }
}

export function ToObjectPattern(node: types.ObjectPattern): ast.ObjectPattern {
    /*
    return {
        elements: node.properties.map(
            x => {
                if (x.type === 'RestElement') {
                    throw 'RestElement not implemented'
                }
                // {restPattern: ToPattern(x.argument)} as ast.ObjectPattern_Element
                return ToObjectProperty(x)
            }
    }
    */
    throw 'Not implemented toobjectpattern'
}

export function ToObjectProperty(node: types.ObjectProperty): ast.ObjectExpression_Property {
    if (node.key.type === 'PrivateName') {
        throw 'PrivateName is not a valid Tessie grammar'
    }

    if (node.value.type === 'TSAsExpression' ||
        node.value.type === 'TSTypeAssertion' ||
        node.value.type === 'TSNonNullExpression'
    ) {
        // TODO: implement
        throw 'TSAsExpression, TSTypeAssertion, and TSNonNullExpression are not a valid Tessie grammar'
    }

    const result = {} as ast.ObjectExpression_Property

    switch (node.key.type) {
        case 'Identifier':
            result.name = { identifier: ToIdentifier(node.key) } as ast.PropName
            break
        case 'StringLiteral':
            result.name = { stringLiteral: node.key.value } as ast.PropName
            break
        case 'NumericLiteral':
            result.name = { numberLiteral: node.key.value } as ast.PropName
            break
        default:
            throw 'Node type not recognized by Tessie grammar'
    }

    switch (node.value.type) {
        case 'Identifier':
            result.value = { variable: { name: ToIdentifier(node.value) } } as ast.Expression
            break
        case 'RestElement':
        case 'AssignmentPattern':
        case 'ArrayPattern':
        case 'ObjectPattern':
            throw 'Not implemented'
        default:
            result.value = ToExpression(node.value)
    }

    return result

    /*
    const key = {
        keyIdentifier: 
            node.key.type === 'Identifier' 
                ? ToIdentifier(node.key) 
                : undefined,
        keyStringLiteral: 
            node.key.type === 'StringLiteral' 
                ? node.key.value 
                : undefined,
        keyNumericLiteral: 
            node.key.type === 'NumericLiteral'  
                ? node.key.value 
                : undefined,
        keyBigintLiteral: 
            node.key.type === 'BigIntLiteral' 
                ? node.key.value 
                : undefined,
        keyExpression:  
            node.key.type !== 'Identifier' && 
            node.key.type !== 'StringLiteral' && 
            node.key.type !== 'NumericLiteral' && 
            node.key.type !== 'BigIntLiteral' 
                ? ToExpression(node.key) 
                : undefined,
    }

    const value = {
        valuePattern: 
            node.value.type === 'Identifier' ||
            node.value.type === 'RestElement' ||
            node.value.type === 'AssignmentPattern' ||
            node.value.type === 'ArrayPattern' ||
            node.value.type === 'ObjectPattern'
                ? ToParameterPattern(node.value) 
                : undefined,
        valueExpression: 
            node.value.type !== 'Identifier' &&
            node.value.type !== 'RestElement' &&
            node.value.type !== 'AssignmentPattern' &&
            node.value.type !== 'ArrayPattern' &&
            node.value.type !== 'ObjectPattern'
                ? ToExpression(node.value) 
                : undefined,
    }

    return {
        ...key,
        ...value,
        computed: node.computed,
        shorthand: node.shorthand ,
    }
    */
}

export function ToFunctionDeclaration(node: types.FunctionDeclaration): ast.FunctionDeclaration {
    const parameters = node.params.map(ToParameterPattern)

    return { function: {
        identifier: ToIdentifier(node.id),
        parameters,
        body: { body: node.body.body.map(ToStatement) },
    }}
}

export function ToIfStatement(node: types.IfStatement): ast.IfStatement {
    return {
        test: ToExpression(node.test),
        consequent: ToStatement(node.consequent),
        alternate: node.alternate ? ToStatement(node.alternate) : undefined,
    }
}

export function ToReturnStatement(node: types.ReturnStatement): ast.ReturnStatement {
    return {
        argument: node.argument ? ToExpression(node.argument) : undefined,
    }
}

export function ToSwitchStatement(node: types.SwitchStatement): ast.SwitchStatement {
    return {
        discriminant: ToExpression(node.discriminant),
        cases: node.cases.map(ToSwitchCase),
    }
}

export function ToSwitchCase(node: types.SwitchCase): ast.SwitchStatement_Case {
    /*
    return {
        test: node.test ? ToExpression(node.test) : undefined,
        consequent: node.consequent.map(ToStatement),
    }
    */
    throw 'Not implemented switchcase'
}

export function ToThrowStatement(node: types.ThrowStatement): ast.ThrowStatement {
    return {
        argument: ToExpression(node.argument),
    }
}

export function ToTryStatement(node: types.TryStatement): ast.TryStatement {
    /*
    return {
        block: ToStatement(node.block),
        handler: node.handler ? ToCatchClause(node.handler) : undefined,
        finalizer: node.finalizer ? ToBlockStatement(node.finalizer) : undefined,
    }
    */
    throw 'Not implemented trystatement'
}

export function ToCatchClause(node: types.CatchClause): ast.TryStatement_CatchClause {
    /*
    const param = {
        paramIdentifier: node.param?.type === 'Identifier' ? ToIdentifier(node.param) : undefined,
        paramArray: node.param?.type === 'ArrayPattern' ? ToArrayPattern(node.param) : undefined,
        paramObject: node.param?.type === 'ObjectPattern' ? ToObjectPattern(node.param) : undefined,
    }

    return {
        ...param,
        body: ToBlockStatement(node.body),
    }
    */
   throw 'Not implemented catchclause'
}

const VariableDeclarationKindMap = {
    'let': ast.DeclarationKind.LET,
    'const': ast.DeclarationKind.CONST,
}

export function ToVariableDeclaration(node: types.VariableDeclaration): ast.VariableDeclaration {
    if (node.kind === 'var') {
        throw 'Var cannot be used inside of a variable declaration'
    }

    return {
        kind: VariableDeclarationKindMap[node.kind],
        declarators: node.declarations.map(ToVariableDeclarator),
    }
}

export function ToVariableDeclarator(node: types.VariableDeclarator): ast.VariableDeclarator {
    if (node.id.type === 'Identifier') {
        return {
            normal: {
                identifier: ToIdentifier(node.id),
                value: node.init ? ToExpression(node.init) : undefined,
            },
        } as ast.VariableDeclarator
    } else {
        throw 'binding pattern is not implemented'
    }
}

export function ToWhileStatement(node: types.WhileStatement): ast.WhileStatement {
    return {
        test: ToExpression(node.test),
        body: ToStatement(node.body),
    }
}

/*
export type Expression =
    | types.StringLiteral
    | types.NumericLiteral
    | types.NullLiteral
    | types.BooleanLiteral
    | types.BigIntLiteral

    | ArrayExpression
    | ArrowFunctionExpression
    | AssignmentExpression
    | BinaryExpression
    | CallExpression
    | ConditionalExpression
    | FunctionExpression
    | Identifier
    | LogicalExpression
    | MemberExpression
    | ObjectExpression
    | ParenthesizedExpression
    | UnaryExpression
    | UpdateExpression
*/

  // | TaggedTemplateExpression
  // | TemplateLiteral
  // | Import

  // TODO: add rest of types
  // | OptionalMemberExpression
  // | OptionalCallExpression
  // | TypeCastExpression
  // | TSInstantiationExpression
  // | TSAsExpression
  // | TSTypeAssertion
  // | TSNonNullExpression;

export function ToExpression(node: types.Expression): ast.Expression {
    let result = {} as ast.Expression
    switch (node.type) {
    case 'BigIntLiteral':
        result.literal = { literal: { bigintLiteral: node.value } } as ast.LiteralExpression
        break
    case 'BooleanLiteral':
        result.literal = { literal: { booleanLiteral: node.value } } as ast.LiteralExpression
        break
    case 'NullLiteral':
        result.literal = { literal: { nullLiteral: {} } } as ast.LiteralExpression
        break
    case 'NumericLiteral':
        result.literal = { literal: { numberLiteral: node.value } } as ast.LiteralExpression
        break
    case 'StringLiteral':
        result.literal = { literal: { stringLiteral: node.value } } as ast.LiteralExpression
        break
    case 'ArrayExpression':
        result.array = ToArrayExpression(node) 
        break
    case 'ArrowFunctionExpression':
        result.arrowFunction = ToArrowFunctionExpression(node)
        break
    case 'AssignmentExpression':
        result.assignment = ToAssignmentExpression(node)
        break
    case 'BinaryExpression':
        result.binary = ToBinaryExpression(node)
        break
    case 'CallExpression':
        result.call = ToCallExpression(node)
        break
    case 'ConditionalExpression':
        result.conditional = ToConditionalExpression(node)
        break
    case 'FunctionExpression':
        result.function = ToFunctionExpression(node)
        break
    case 'Identifier':
        result.variable = { name: ToIdentifier(node) } as ast.VariableExpression
        break
    case 'LogicalExpression':
        result.logical = ToLogicalExpression(node)
        break
    case 'MemberExpression':
        result.member = ToMemberExpression(node)
        break
    case 'ObjectExpression':
        result.object = ToObjectExpression(node)
        break
    case 'ParenthesizedExpression':
        result = ToExpression(node.expression)
        break
    case 'UnaryExpression':
        result.unary = ToUnaryExpression(node)
        break
    case 'UpdateExpression':
        result.update = ToUpdateExpression(node)
        break
    default:
        throw `Unsupported expression type: ${node.type}`
    }
    return result
}
/*
export type LVal =
  | Identifier
  | MemberExpression
  | RestElement
  | AssignmentPattern
  | ArrayPattern
  | ObjectPattern
  // | TSParameterProperty
  // | TSAsExpression
  // | TSTypeAssertion
  // | TSNonNullExpression;
*/
/*
export function ToLVal(node: types.LVal): ast.LVal {
    let result = {} as ast.LVal
    switch (node.type) {
    case 'Identifier':
        result.identifier = ToIdentifier(node)
        break
    case 'MemberExpression':
        result.member = ToMemberExpression(node)
        break
    case 'RestElement':
        result = ToLVal(node.argument)
        result.isRest = true
        break
    case 'AssignmentPattern':
        result.assignment = ToAssignmentPattern(node)
        break
    case 'ArrayPattern':
        result.array = ToArrayPattern(node)
        break
    case 'ObjectPattern':
        result.object = ToObjectPattern(node)
        break
    }
    return result
}
*/
export function ToArrayExpression(node: types.ArrayExpression): ast.ArrayExpression {
    return {
        elements: node.elements.map(x => {
            if (!x) throw 'ArrayExpression element is null'
            return (x.type === 'SpreadElement'
                ? { spreadElement: ToExpression(x.argument) }
                : { element: ToExpression(x) }) as ast.ParameterElement
        }) 
    }
}

const AssignmentExpressionOperatorMap = {
    '=': ast.AssignmentExpression_Operator.ASSIGN,
    '*=': ast.AssignmentExpression_Operator.MUL,
    '/=': ast.AssignmentExpression_Operator.DIV,
    '%=': ast.AssignmentExpression_Operator.MOD,
    '+=': ast.AssignmentExpression_Operator.ADD,
    '-=': ast.AssignmentExpression_Operator.SUB,
    '<<=': ast.AssignmentExpression_Operator.LSHIFT,
    '>>=': ast.AssignmentExpression_Operator.RSHIFT,
    '>>>=': ast.AssignmentExpression_Operator.URSHIFT,
    '&=': ast.AssignmentExpression_Operator.BITAND,
    '^=': ast.AssignmentExpression_Operator.BITXOR,
    '|=': ast.AssignmentExpression_Operator.BITOR,
    '**=': ast.AssignmentExpression_Operator.POW,
}

export function ToAssignmentExpression(node: types.AssignmentExpression): ast.AssignmentExpression {
    const left = {} as ast.AssignmentExpression_LValue
    switch (node.left.type) {
        case 'Identifier':
            left.identifier = ToIdentifier(node.left)
            break
        case 'MemberExpression':
            left.member = ToMemberExpression(node.left)
            break
        case 'RestElement':
            throw 'RestElement not supported in AssignmentExpression'
        case 'AssignmentPattern':
            throw 'AssignmentPattern not supported in AssignmentExpression'
        case 'ArrayPattern':
            throw 'ArrayPattern not supported in AssignmentExpression'
        case 'ObjectPattern':
            throw 'ObjectPattern not supported in AssignmentExpression'
    }

    return {
        operator: AssignmentExpressionOperatorMap[node.operator],
        left,
        right: ToExpression(node.right),
    }
}

const BinaryExpressionOperatorMap = {
    "+": ast.BinaryExpression_Operator.ADD,
    "-": ast.BinaryExpression_Operator.SUB,
    "/": ast.BinaryExpression_Operator.DIV,
    "%": ast.BinaryExpression_Operator.MOD,
    "*": ast.BinaryExpression_Operator.MUL,
    "**": ast.BinaryExpression_Operator.POW,
    "&": ast.BinaryExpression_Operator.BITAND,
    "|": ast.BinaryExpression_Operator.BITOR,
    ">>": ast.BinaryExpression_Operator.RSHIFT,
    ">>>": ast.BinaryExpression_Operator.URSHIFT,
    "<<": ast.BinaryExpression_Operator.LSHIFT,
    "^": ast.BinaryExpression_Operator.BITXOR,
    "===": ast.BinaryExpression_Operator.EQ,
    "!==": ast.BinaryExpression_Operator.NEQ,
    ">": ast.BinaryExpression_Operator.GT,
    "<": ast.BinaryExpression_Operator.LT,
    ">=": ast.BinaryExpression_Operator.GTE,
    "<=" : ast.BinaryExpression_Operator.LTE,
}

export function ToBinaryExpression(node: types.BinaryExpression) {
    if (node.left.type === 'PrivateName') {
        throw 'PrivateName is not a valid Tessie grammar'
    }

    return {
        type: node.type,
        operator: BinaryExpressionOperatorMap[node.operator],
        left: ToExpression(node.left),
        right: ToExpression(node.right),
    }
}

export function ToCallExpression(node: types.CallExpression): ast.CallExpression {
    if (node.callee.type === 'V8IntrinsicIdentifier') {
        throw 'V8IntrinsicIdentifier is not a valid Tessie grammar'
    }

    return {
        callee: ToExpression(node.callee),
        arguments: node.arguments.map(x => {
            if (x.type === 'JSXNamespacedName' || x.type === 'ArgumentPlaceholder') {
                throw `${x.type} is not a valid Tessie grammar`
            }
            return (x.type === 'SpreadElement' 
                ? { spreadElement: ToExpression(x.argument) }
                : { element: ToExpression(x) }) as ast.CallExpression_CallElement
        }),
    }
}

export function ToConditionalExpression(node: types.ConditionalExpression): ast.ConditionalExpression {
    return {
        test: ToExpression(node.test),
        consequent: ToExpression(node.consequent),
        alternate: ToExpression(node.alternate),
    }
}

export function ToFunctionExpression(node: types.FunctionExpression): ast.FunctionExpression {
    return {
        identifier: node.id ? ToIdentifier(node.id) : undefined,
        parameters: node.params.map(ToParameterPattern),
        body: { body: node.body.body.map(ToStatement) },
    }
}

const LogicalExpressionOperatorMap = {
    "||": ast.LogicalExpression_Operator.OR,
    "&&": ast.LogicalExpression_Operator.AND,
    "??": ast.LogicalExpression_Operator.COALESCE,
}

export function ToLogicalExpression(node: types.LogicalExpression): ast.LogicalExpression {
    return {
        operator: LogicalExpressionOperatorMap[node.operator],
        left: ToExpression(node.left),
        right: ToExpression(node.right),
    }
}

export function ToMemberExpression(node: types.MemberExpression): ast.MemberExpression {
    if (node.property.type === 'PrivateName') {
        throw 'PrivateName is not a valid Tessie grammar'
    }

    if (node.computed) {
        if (node.property.type === 'Identifier') {
            // obj[x]
            return {
                object: ToExpression(node.object),
                index: { variable: { name: ToIdentifier(node.property) } },
            } as ast.MemberExpression
        } else {
            // obj[someExpression()]
            return {
                object: ToExpression(node.object),
                index: ToExpression(node.property),
            } as ast.MemberExpression
        }
    } else {
        if (node.property.type === 'Identifier') {
            // obj.x
            return {
                object: ToExpression(node.object),
                property: ToIdentifier(node.property),
            } as ast.MemberExpression
        } else {
            // ???
            throw 'Unsupported MemberExpression'
        }
    }
}

export function ToObjectExpression(node: types.ObjectExpression): ast.ObjectExpression {
    const toObjectElement = (x) => { return {
        // TODO: shorthand properties
        method: x.type === 'ObjectMethod' ? ToObjectMethod(x) : undefined,
        property: x.type === 'ObjectProperty' ? ToObjectProperty(x) : undefined,
        spread: x.type === 'SpreadElement' ? ToExpression(x) : undefined,
    } as ast.ObjectExpression_Element}

    return {
        elements: node.properties.map(toObjectElement)
    }
}

export function ToObjectMethod(node: types.ObjectMethod): ast.ObjectExpression_Method {
    if (node.key.type !== 'Identifier') {
        throw 'Unsupported ObjectMethod'
    }

    const method = node.kind === 'method' 
        ? { 
            identifier: { name: node.key.name },
            parameters: node.params.map(ToParameterPattern),
            body: { body: node.body.body.map(ToStatement) },
        } 
        : undefined
/*
    const getter = node.kind === 'get' 
        ? { getter: {
            name: node.key.name,
            body: { body: node.body.body.map(ToStatement) },
        } }
        : undefined

    const setter = node.kind === 'set'
        ? { setter: {
            name: node.key.name,
            parameters: [],
        } }
        : undefined
*/
// TODO

/*
    const key = node.key && {
        keyIdentifier: node.key.type === 'Identifier' ? ToIdentifier(node.key) : undefined,

        keyStringLiteral: node.key.type === 'StringLiteral' ? node.key.value : undefined,

        keyNumericLiteral: node.key.type === 'NumericLiteral' ? node.key.value : undefined,

        keyExpression: node.key.type !== 'Identifier' && node.key.type !== 'StringLiteral' && node.key.type !== 'NumericLiteral' ? ToExpression(node.key) : undefined,
    }
*/
    return {
        method,
    } as ast.ObjectExpression_Method
}

const UnaryExpressionOperatorMap = {
    "void": ast.UnaryExpression_Operator.VOID,
    "!": ast.UnaryExpression_Operator.NOT,
    "+": ast.UnaryExpression_Operator.POS,
    "-": ast.UnaryExpression_Operator.NEG,
    "~": ast.UnaryExpression_Operator.BITNOT,
    "typeof": ast.UnaryExpression_Operator.TYPEOF,
}

export function ToUnaryExpression(node: types.UnaryExpression): ast.UnaryExpression {
    if (node.operator === 'delete' || node.operator === 'throw') {
        throw `${node.operator} is not a valid Tessie operator`
    }

    return {
        operator: UnaryExpressionOperatorMap[node.operator],
        argument: ToExpression(node.argument),
        prefix: node.prefix
    }
}

const UpdateExpressionOperatorMap = {
    "++": ast.UpdateExpression_Operator.INC,
    "--": ast.UpdateExpression_Operator.DEC,
}

export function ToUpdateExpression(node: types.UpdateExpression): ast.UpdateExpression {
    return {
        operator: UpdateExpressionOperatorMap[node.operator],
        argument: ToExpression(node.argument),
        prefix: node.prefix
    }
}

export function ToArrowFunctionExpression(node: types.ArrowFunctionExpression): ast.ArrowFunctionExpression {
    const body = {
        statement: node.body.type === 'BlockStatement' ? { body: node.body.body.map(ToStatement) } : undefined,
        expression: node.body.type !== 'BlockStatement' ? ToExpression(node.body) : undefined,
    }

    return {
        params: node.params.map(ToParameterPattern),
        ...body,
    }
}

export function ToIdentifier(node?: types.Identifier | null | undefined): ast.Identifier {
    if (!node) {
        throw 'Identifier is null'
    }
    return {
        name: node.name,
        option: node.optional ? node.optional : undefined,
    }
}
/*
export function ToParameterPattern(node: types.Identifier | types.Pattern | types.RestElement): ast.Param {
    const result = {} as ast.PatternLike
    if (node.type === 'Identifier') {
        result.identifier = ToIdentifier(node)
    } else if (node.type === 'RestElement') {
        result.restElement = ToRestElement(node)
    } else {
        return ToPatternLike(node)
    }
    return result
}
*/