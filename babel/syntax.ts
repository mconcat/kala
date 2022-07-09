import { parse } from '@babel/parser';
import * as types from '@babel/types';
import traverse from '@babel/traverse';
import generate from '@babel/generator';
import * as ast from './gen/ast';
import * as proto from './proto';

export const checker = {
        RegExpLiteral(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have RegExpLiteral.'
            )
        },
        DecimalLiteral(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have DecimalLiteral.'
            )
        },
        Function(path) {
            if (path.node.generator) {
                throw path.buildCodeFrameError(
                    'Tessie code cannot have generator function'
                )
            }
            if (path.node.async) {
                throw path.buildCodeFrameError(
                    'Tessie code cannot have async function'
                )
            }
        },
        DebuggerStatement(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have DebuggerStatement.'
            )
        },
        WithStatement(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have WithStatement.'
            )
        },
        LabeledStatement(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have LabeledStatement.'
            )
        },
        BreakStatement(path) {
            if (path.node.label) {
                throw path.buildCodeFrameError(
                    'Tessie code cannot have labeled BreakStatement.'
                )
            }
        },
        ContinueStatement(path) {
            if (path.node.label) {
                throw path.buildCodeFrameError(
                    'Tessie code cannot have labeled ContinueStatement.'
                )
            }
        },
        DoWhileStatement(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have DoWhileStatement.'
            )
        },
        ForInStatement(path) { // TODO: check
            throw path.buildCodeFrameError(
                'Tessie code cannot have ForInStatement.'
            )
        },
        ForOfStatement(path) { // TODO: checl
            throw path.buildCodeFrameError(
                'Tessie code cannot have ForOfStatement.'
            )
        },
        VariableDeclaration(path) {
            if (path.node.kind === 'var') {
                throw path.buildCodeFrameError(
                    'Tessie code cannot have var VariableDeclaration.'
                )
            }
        },
        VariableDeclarator(path) {
            if (path.node.init === null) {
                throw path.buildCodeFrameError(
                    'Tessie code cannot have non-initialized VariableDeclarator.'
                )
            }
        },
        Super(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have super keyword.'
            )
        },
        ThisExpression(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have this keyword.'
            )
        }, 
        YieldExpression(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have yield keyword.'
            )
        }, 
        AwaitExpression(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have await keyword.'
            )
        }, 
        BinaryExpression(path) {
            const prohibited = {
                '==': true,
                '!=': true,
                'in': true,
                'instanceof': true,
                '|>': true,
            }
            if (path.node.operator in prohibited) {
                throw path.buildCodeFrameError(
                    `Tessie code cannot have operator ${path.node.operator}.`
                )
            }
        },
        NewExpression(path) {
            console.log(path.node)
            throw path.buildCodeFrameError(
                'Tessie code cannot have keyword new'
            )
        },
        Class(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have class.'
            )
        },
        ClassDeclaration(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have ClassDeclaration.'
            )
        },
        ClassExpression(path) {
            throw path.buildCodeFrameError(
                'Tessie code cannot have ClassExpression.'
            )
        }, 
    }

type Identifier = types.Identifier

type Literal =
    | types.StringLiteral
    | types.NumericLiteral 
    | types.NullLiteral
    | types.BooleanLiteral
    | types.TemplateLiteral
    | types.BigIntLiteral

// check checks if the input javascript code follows the 
// jessie grammar constraints.
export function check(code: string): ast.Statement[] {
    const root = parse(code, {
        sourceType: 'script',

        plugins: [
            "typescript",
        ],
    })

    const statements = root.program.body.map(node => 
        proto.ToStatement(node))

    return statements
}

export function protofy(code: string): Uint8Array[] {
    const root = parse(code, {
        sourceType: 'script',

        plugins: [
            "typescript",
        ],
    })

    const statements = root.program.body.map(node => 
        ast.Statement.encode(proto.ToStatement(node)).finish())

    return statements
} 


