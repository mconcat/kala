/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "nessie.ast";

/**
 * /////////////////////////////////////////////////////////////////////////////
 * Global types
 */
export enum DeclarationKind {
  UNKNOWN = 0,
  LET = 1,
  CONST = 2,
  UNRECOGNIZED = -1,
}

export function declarationKindFromJSON(object: any): DeclarationKind {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return DeclarationKind.UNKNOWN;
    case 1:
    case "LET":
      return DeclarationKind.LET;
    case 2:
    case "CONST":
      return DeclarationKind.CONST;
    case -1:
    case "UNRECOGNIZED":
    default:
      return DeclarationKind.UNRECOGNIZED;
  }
}

export function declarationKindToJSON(object: DeclarationKind): string {
  switch (object) {
    case DeclarationKind.UNKNOWN:
      return "UNKNOWN";
    case DeclarationKind.LET:
      return "LET";
    case DeclarationKind.CONST:
      return "CONST";
    case DeclarationKind.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum ArithmeticOperator {
  ARITHMETIC_UNKNOWN = 0,
  /** ADD - | "+" */
  ADD = 1,
  /** SUB - | "-" */
  SUB = 2,
  /** DIV - | "/" */
  DIV = 3,
  /** MOD - | "%" */
  MOD = 4,
  /** MUL - | "*" */
  MUL = 5,
  /** POW - | "**" */
  POW = 6,
  /** BITAND - | "&" */
  BITAND = 7,
  /** BITOR - | "|" */
  BITOR = 8,
  /** RSHIFT - | ">>" */
  RSHIFT = 9,
  /** URSHIFT - | ">>>" */
  URSHIFT = 10,
  /** LSHIFT - | "<<" */
  LSHIFT = 11,
  /** BITXOR - | "^" */
  BITXOR = 12,
  UNRECOGNIZED = -1,
}

export function arithmeticOperatorFromJSON(object: any): ArithmeticOperator {
  switch (object) {
    case 0:
    case "ARITHMETIC_UNKNOWN":
      return ArithmeticOperator.ARITHMETIC_UNKNOWN;
    case 1:
    case "ADD":
      return ArithmeticOperator.ADD;
    case 2:
    case "SUB":
      return ArithmeticOperator.SUB;
    case 3:
    case "DIV":
      return ArithmeticOperator.DIV;
    case 4:
    case "MOD":
      return ArithmeticOperator.MOD;
    case 5:
    case "MUL":
      return ArithmeticOperator.MUL;
    case 6:
    case "POW":
      return ArithmeticOperator.POW;
    case 7:
    case "BITAND":
      return ArithmeticOperator.BITAND;
    case 8:
    case "BITOR":
      return ArithmeticOperator.BITOR;
    case 9:
    case "RSHIFT":
      return ArithmeticOperator.RSHIFT;
    case 10:
    case "URSHIFT":
      return ArithmeticOperator.URSHIFT;
    case 11:
    case "LSHIFT":
      return ArithmeticOperator.LSHIFT;
    case 12:
    case "BITXOR":
      return ArithmeticOperator.BITXOR;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ArithmeticOperator.UNRECOGNIZED;
  }
}

export function arithmeticOperatorToJSON(object: ArithmeticOperator): string {
  switch (object) {
    case ArithmeticOperator.ARITHMETIC_UNKNOWN:
      return "ARITHMETIC_UNKNOWN";
    case ArithmeticOperator.ADD:
      return "ADD";
    case ArithmeticOperator.SUB:
      return "SUB";
    case ArithmeticOperator.DIV:
      return "DIV";
    case ArithmeticOperator.MOD:
      return "MOD";
    case ArithmeticOperator.MUL:
      return "MUL";
    case ArithmeticOperator.POW:
      return "POW";
    case ArithmeticOperator.BITAND:
      return "BITAND";
    case ArithmeticOperator.BITOR:
      return "BITOR";
    case ArithmeticOperator.RSHIFT:
      return "RSHIFT";
    case ArithmeticOperator.URSHIFT:
      return "URSHIFT";
    case ArithmeticOperator.LSHIFT:
      return "LSHIFT";
    case ArithmeticOperator.BITXOR:
      return "BITXOR";
    case ArithmeticOperator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface Identifier {
  name: string;
  option?: boolean | undefined;
}

export interface NullLiteral {}

export interface UndefinedLiteral {}

export interface Literal {
  stringLiteral: string | undefined;
  /** must be under 2^53 */
  numberLiteral: number | undefined;
  booleanLiteral: boolean | undefined;
  nullLiteral: NullLiteral | undefined;
  undefinedLiteral: UndefinedLiteral | undefined;
  /** must be stringified big integer */
  bigintLiteral: string | undefined;
}

/**
 * defVar EQUALS assignExpr
 * optional parameter
 */
export interface OptionalPattern {
  identifier: Identifier | undefined;
  expression: Expression | undefined;
}

/** list of patterns with rest or optional */
export interface ParameterPattern {
  pattern: Pattern | undefined;
  optional: OptionalPattern | undefined;
}

/** destructuring array pattern */
export interface ArrayPattern {
  elements: ParameterPattern[];
}

export interface PropName {
  stringLiteral: string | undefined;
  identifier: Identifier | undefined;
  numberLiteral: number | undefined;
}

/** destructuring object pattern */
export interface ObjectPattern {
  elements: ObjectPattern_Element[];
}

export interface ObjectPattern_Property {
  name: PropName | undefined;
  pattern: Pattern | undefined;
}

/**
 * {
 *   id, // keyIdentifier = "id", valuePattern.Identifier = "id"
 *   g = 3, // keyIdentifier = "g", valuePattern.Assignment = 3
 *   "stringkey": 3,
 *   1234: "string",
 * }
 */
export interface ObjectPattern_Element {
  property: ObjectPattern_Property | undefined;
  shorthand: Identifier | undefined;
  optional: OptionalPattern | undefined;
  restPattern: Pattern | undefined;
}

/** destructuring pattern */
export interface BindingPattern {
  array: ArrayPattern | undefined;
  object: ObjectPattern | undefined;
}

export interface Hole {}

/** pattern */
export interface Pattern {
  /** Variable access */
  identifier: Identifier | undefined;
  /** literal value */
  literal: Literal | undefined;
  /** hole */
  hole: Hole | undefined;
  /** destructuring pattern */
  binding: BindingPattern | undefined;
  /** ...pattern */
  isRest: boolean;
}

/** With no label statements, statement and statementitem are equivalent. */
export interface Statement {
  /** Declarations */
  variableDeclaration: VariableDeclaration | undefined;
  functionDeclaration: FunctionDeclaration | undefined;
  /** Block */
  blockStatement: BlockStatement | undefined;
  /** If */
  ifStatement: IfStatement | undefined;
  /** Breakable Statements */
  forStatement: ForStatement | undefined;
  forOfStatement: ForOfStatement | undefined;
  whileStatement: WhileStatement | undefined;
  switchStatement: SwitchStatement | undefined;
  /** Try-catch */
  tryStatement: TryStatement | undefined;
  /** Terminators */
  breakStatement: BreakStatement | undefined;
  continueStatement: ContinueStatement | undefined;
  returnStatement: ReturnStatement | undefined;
  throwStatement: ThrowStatement | undefined;
  /** Expression */
  expressionStatement: ExpressionStatement | undefined;
}

/** binding */
export interface VariableDeclarator {
  normal: VariableDeclarator_NormalDeclarator | undefined;
  binding: VariableDeclarator_BindingDeclarator | undefined;
}

export interface VariableDeclarator_NormalDeclarator {
  identifier: Identifier | undefined;
  /** empty if declared without initialization */
  value?: Expression | undefined;
}

export interface VariableDeclarator_BindingDeclarator {
  pattern: BindingPattern | undefined;
  value: Expression | undefined;
}

export interface VariableDeclaration {
  kind: DeclarationKind;
  declarators: VariableDeclarator[];
}

export interface FunctionDeclaration {
  /** function.identifier should not be empty */
  function: FunctionExpression | undefined;
}

/** { doSomething() } */
export interface BlockStatement {
  body: Statement[];
}

export interface IfStatement {
  test: Expression | undefined;
  consequent: Statement | undefined;
  alternate?: Statement | undefined;
}

/**
 * ForStatement has optional initializer, optional test, optional update.
 * for (let x = 0; x < 5; x++) { doSomething() }
 */
export interface ForStatement {
  kind: DeclarationKind;
  init: VariableDeclarator | undefined;
  test?: Expression | undefined;
  update?: Expression | undefined;
  body: BlockStatement | undefined;
}

/**
 * for (let x of y) { doSomething() }
 * for (const [x, y] of z) { doSomething() }
 */
export interface ForOfStatement {
  kind: DeclarationKind;
  declarator: VariableDeclarator | undefined;
  body: Statement | undefined;
}

export interface WhileStatement {
  test: Expression | undefined;
  body: Statement | undefined;
}

export interface SwitchStatement {
  discriminant: Expression | undefined;
  cases: SwitchStatement_Case[];
}

export interface SwitchStatement_CaseLabel {
  test: Expression | undefined;
  default: SwitchStatement_CaseLabel_Default | undefined;
}

export interface SwitchStatement_CaseLabel_Default {}

export interface SwitchStatement_Case {
  labels: SwitchStatement_CaseLabel[];
  /** CONTRACT: should end with oneof terminators */
  consequent: BlockStatement | undefined;
}

export interface TryStatement {
  block: BlockStatement | undefined;
  handler?: TryStatement_CatchClause | undefined;
  finalizer?: BlockStatement | undefined;
}

export interface TryStatement_CatchClause {
  pattern: Pattern | undefined;
  body: BlockStatement | undefined;
}

/**
 * BreakStatement omits label
 * break
 */
export interface BreakStatement {}

/**
 * ContinuseStatement omits label
 * continue
 */
export interface ContinueStatement {}

export interface ReturnStatement {
  argument?: Expression | undefined;
}

export interface ThrowStatement {
  argument: Expression | undefined;
}

export interface ExpressionStatement {
  expression: Expression | undefined;
}

export interface Expression {
  /** Literal expressions */
  literal: LiteralExpression | undefined;
  array: ArrayExpression | undefined;
  object: ObjectExpression | undefined;
  function: FunctionExpression | undefined;
  arrowFunction: ArrowFunctionExpression | undefined;
  /** Operator expressions */
  binary: BinaryExpression | undefined;
  unary: UnaryExpression | undefined;
  conditional: ConditionalExpression | undefined;
  logical: LogicalExpression | undefined;
  update: UpdateExpression | undefined;
  /** Variable accessing expressions */
  variable: VariableExpression | undefined;
  assignment: AssignmentExpression | undefined;
  member: MemberExpression | undefined;
  /** Function call expression */
  call: CallExpression | undefined;
}

export interface ParameterElement {
  element: Expression | undefined;
  spreadElement: Expression | undefined;
}

export interface LiteralExpression {
  literal: Literal | undefined;
}

export interface ArrayExpression {
  elements: ParameterElement[];
}

export interface ObjectExpression {
  elements: ObjectExpression_Element[];
}

export interface ObjectExpression_Property {
  name: PropName | undefined;
  value: Expression | undefined;
}

export interface ObjectExpression_Method {
  method: FunctionExpression | undefined;
  getter: ObjectExpression_Method_Getter | undefined;
  setter: ObjectExpression_Method_Setter | undefined;
}

export interface ObjectExpression_Method_Getter {
  name: PropName | undefined;
  body: BlockStatement | undefined;
}

export interface ObjectExpression_Method_Setter {
  name: PropName | undefined;
  param: ParameterPattern | undefined;
  body: BlockStatement | undefined;
}

export interface ObjectExpression_Element {
  property: ObjectExpression_Property | undefined;
  shorthand: Identifier | undefined;
  method: ObjectExpression_Method | undefined;
  spread: Expression | undefined;
}

export interface FunctionExpression {
  identifier?: Identifier | undefined;
  parameters: ParameterPattern[];
  body: BlockStatement | undefined;
}

export interface ArrowFunctionExpression {
  params: ParameterPattern[];
  statement: BlockStatement | undefined;
  expression: Expression | undefined;
}

export interface BinaryExpression {
  arithmetic: ArithmeticOperator | undefined;
  comparison: BinaryExpression_ComparisonOperator | undefined;
  left: Expression | undefined;
  right: Expression | undefined;
}

export enum BinaryExpression_ComparisonOperator {
  COMPARISON_UNKNOWN = 0,
  /** EQ - | "===" */
  EQ = 13,
  /** NEQ - | "!==" */
  NEQ = 14,
  /** GT - | ">" */
  GT = 15,
  /** LT - | "<" */
  LT = 16,
  /** GTE - | ">=" */
  GTE = 17,
  /** LTE - | "<=" */
  LTE = 18,
  UNRECOGNIZED = -1,
}

export function binaryExpression_ComparisonOperatorFromJSON(
  object: any
): BinaryExpression_ComparisonOperator {
  switch (object) {
    case 0:
    case "COMPARISON_UNKNOWN":
      return BinaryExpression_ComparisonOperator.COMPARISON_UNKNOWN;
    case 13:
    case "EQ":
      return BinaryExpression_ComparisonOperator.EQ;
    case 14:
    case "NEQ":
      return BinaryExpression_ComparisonOperator.NEQ;
    case 15:
    case "GT":
      return BinaryExpression_ComparisonOperator.GT;
    case 16:
    case "LT":
      return BinaryExpression_ComparisonOperator.LT;
    case 17:
    case "GTE":
      return BinaryExpression_ComparisonOperator.GTE;
    case 18:
    case "LTE":
      return BinaryExpression_ComparisonOperator.LTE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return BinaryExpression_ComparisonOperator.UNRECOGNIZED;
  }
}

export function binaryExpression_ComparisonOperatorToJSON(
  object: BinaryExpression_ComparisonOperator
): string {
  switch (object) {
    case BinaryExpression_ComparisonOperator.COMPARISON_UNKNOWN:
      return "COMPARISON_UNKNOWN";
    case BinaryExpression_ComparisonOperator.EQ:
      return "EQ";
    case BinaryExpression_ComparisonOperator.NEQ:
      return "NEQ";
    case BinaryExpression_ComparisonOperator.GT:
      return "GT";
    case BinaryExpression_ComparisonOperator.LT:
      return "LT";
    case BinaryExpression_ComparisonOperator.GTE:
      return "GTE";
    case BinaryExpression_ComparisonOperator.LTE:
      return "LTE";
    case BinaryExpression_ComparisonOperator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface UnaryExpression {
  operator: UnaryExpression_Operator;
  argument: Expression | undefined;
  prefix: boolean;
}

export enum UnaryExpression_Operator {
  UNKNOWN = 0,
  /** VOID - | "void" */
  VOID = 1,
  /** NOT - | "!" */
  NOT = 2,
  /** POS - | "+" */
  POS = 3,
  /** NEG - | "-" */
  NEG = 4,
  /** BITNOT - | "~" */
  BITNOT = 5,
  /** TYPEOF - | "typeof" */
  TYPEOF = 6,
  UNRECOGNIZED = -1,
}

export function unaryExpression_OperatorFromJSON(
  object: any
): UnaryExpression_Operator {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return UnaryExpression_Operator.UNKNOWN;
    case 1:
    case "VOID":
      return UnaryExpression_Operator.VOID;
    case 2:
    case "NOT":
      return UnaryExpression_Operator.NOT;
    case 3:
    case "POS":
      return UnaryExpression_Operator.POS;
    case 4:
    case "NEG":
      return UnaryExpression_Operator.NEG;
    case 5:
    case "BITNOT":
      return UnaryExpression_Operator.BITNOT;
    case 6:
    case "TYPEOF":
      return UnaryExpression_Operator.TYPEOF;
    case -1:
    case "UNRECOGNIZED":
    default:
      return UnaryExpression_Operator.UNRECOGNIZED;
  }
}

export function unaryExpression_OperatorToJSON(
  object: UnaryExpression_Operator
): string {
  switch (object) {
    case UnaryExpression_Operator.UNKNOWN:
      return "UNKNOWN";
    case UnaryExpression_Operator.VOID:
      return "VOID";
    case UnaryExpression_Operator.NOT:
      return "NOT";
    case UnaryExpression_Operator.POS:
      return "POS";
    case UnaryExpression_Operator.NEG:
      return "NEG";
    case UnaryExpression_Operator.BITNOT:
      return "BITNOT";
    case UnaryExpression_Operator.TYPEOF:
      return "TYPEOF";
    case UnaryExpression_Operator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface ConditionalExpression {
  test: Expression | undefined;
  consequent: Expression | undefined;
  alternate: Expression | undefined;
}

export interface LogicalExpression {
  operator: LogicalExpression_Operator;
  left: Expression | undefined;
  right: Expression | undefined;
}

export enum LogicalExpression_Operator {
  UNKNOWN = 0,
  /** AND - | "&&" */
  AND = 1,
  /** OR - | "||" */
  OR = 2,
  /** COALESCE - | "??" */
  COALESCE = 3,
  UNRECOGNIZED = -1,
}

export function logicalExpression_OperatorFromJSON(
  object: any
): LogicalExpression_Operator {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return LogicalExpression_Operator.UNKNOWN;
    case 1:
    case "AND":
      return LogicalExpression_Operator.AND;
    case 2:
    case "OR":
      return LogicalExpression_Operator.OR;
    case 3:
    case "COALESCE":
      return LogicalExpression_Operator.COALESCE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return LogicalExpression_Operator.UNRECOGNIZED;
  }
}

export function logicalExpression_OperatorToJSON(
  object: LogicalExpression_Operator
): string {
  switch (object) {
    case LogicalExpression_Operator.UNKNOWN:
      return "UNKNOWN";
    case LogicalExpression_Operator.AND:
      return "AND";
    case LogicalExpression_Operator.OR:
      return "OR";
    case LogicalExpression_Operator.COALESCE:
      return "COALESCE";
    case LogicalExpression_Operator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface UpdateExpression {
  operator: UpdateExpression_Operator;
  argument: Expression | undefined;
  prefix: boolean;
}

export enum UpdateExpression_Operator {
  UNKNOWN = 0,
  /** INC - | "++" */
  INC = 1,
  /** DEC - | "--" */
  DEC = 2,
  UNRECOGNIZED = -1,
}

export function updateExpression_OperatorFromJSON(
  object: any
): UpdateExpression_Operator {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return UpdateExpression_Operator.UNKNOWN;
    case 1:
    case "INC":
      return UpdateExpression_Operator.INC;
    case 2:
    case "DEC":
      return UpdateExpression_Operator.DEC;
    case -1:
    case "UNRECOGNIZED":
    default:
      return UpdateExpression_Operator.UNRECOGNIZED;
  }
}

export function updateExpression_OperatorToJSON(
  object: UpdateExpression_Operator
): string {
  switch (object) {
    case UpdateExpression_Operator.UNKNOWN:
      return "UNKNOWN";
    case UpdateExpression_Operator.INC:
      return "INC";
    case UpdateExpression_Operator.DEC:
      return "DEC";
    case UpdateExpression_Operator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface VariableExpression {
  name: Identifier | undefined;
}

export interface AssignmentExpression {
  operator?: ArithmeticOperator | undefined;
  left: AssignmentExpression_LValue | undefined;
  right: Expression | undefined;
}

export interface AssignmentExpression_LValue {
  identifier: Identifier | undefined;
  member: MemberExpression | undefined;
}

export interface MemberExpression {
  object: Expression | undefined;
  index: Expression | undefined;
  /** QuasiQuote propertyQuasi = 4; // TODO */
  property: Identifier | undefined;
}

export interface CallExpression {
  callee: Expression | undefined;
  arguments: ParameterElement[];
}

export interface CallExpression_CallElement {
  element: Expression | undefined;
  spreadElement: Expression | undefined;
}

function createBaseIdentifier(): Identifier {
  return { name: "", option: undefined };
}

export const Identifier = {
  encode(
    message: Identifier,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.option !== undefined) {
      writer.uint32(16).bool(message.option);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Identifier {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseIdentifier();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.option = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Identifier {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      option: isSet(object.option) ? Boolean(object.option) : undefined,
    };
  },

  toJSON(message: Identifier): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.option !== undefined && (obj.option = message.option);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Identifier>, I>>(
    object: I
  ): Identifier {
    const message = createBaseIdentifier();
    message.name = object.name ?? "";
    message.option = object.option ?? undefined;
    return message;
  },
};

function createBaseNullLiteral(): NullLiteral {
  return {};
}

export const NullLiteral = {
  encode(_: NullLiteral, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): NullLiteral {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseNullLiteral();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): NullLiteral {
    return {};
  },

  toJSON(_: NullLiteral): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<NullLiteral>, I>>(_: I): NullLiteral {
    const message = createBaseNullLiteral();
    return message;
  },
};

function createBaseUndefinedLiteral(): UndefinedLiteral {
  return {};
}

export const UndefinedLiteral = {
  encode(
    _: UndefinedLiteral,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UndefinedLiteral {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUndefinedLiteral();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): UndefinedLiteral {
    return {};
  },

  toJSON(_: UndefinedLiteral): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UndefinedLiteral>, I>>(
    _: I
  ): UndefinedLiteral {
    const message = createBaseUndefinedLiteral();
    return message;
  },
};

function createBaseLiteral(): Literal {
  return {
    stringLiteral: undefined,
    numberLiteral: undefined,
    booleanLiteral: undefined,
    nullLiteral: undefined,
    undefinedLiteral: undefined,
    bigintLiteral: undefined,
  };
}

export const Literal = {
  encode(
    message: Literal,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.stringLiteral !== undefined) {
      writer.uint32(10).string(message.stringLiteral);
    }
    if (message.numberLiteral !== undefined) {
      writer.uint32(16).int64(message.numberLiteral);
    }
    if (message.booleanLiteral !== undefined) {
      writer.uint32(24).bool(message.booleanLiteral);
    }
    if (message.nullLiteral !== undefined) {
      NullLiteral.encode(
        message.nullLiteral,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.undefinedLiteral !== undefined) {
      UndefinedLiteral.encode(
        message.undefinedLiteral,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.bigintLiteral !== undefined) {
      writer.uint32(50).string(message.bigintLiteral);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Literal {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLiteral();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.stringLiteral = reader.string();
          break;
        case 2:
          message.numberLiteral = longToNumber(reader.int64() as Long);
          break;
        case 3:
          message.booleanLiteral = reader.bool();
          break;
        case 4:
          message.nullLiteral = NullLiteral.decode(reader, reader.uint32());
          break;
        case 5:
          message.undefinedLiteral = UndefinedLiteral.decode(
            reader,
            reader.uint32()
          );
          break;
        case 6:
          message.bigintLiteral = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Literal {
    return {
      stringLiteral: isSet(object.stringLiteral)
        ? String(object.stringLiteral)
        : undefined,
      numberLiteral: isSet(object.numberLiteral)
        ? Number(object.numberLiteral)
        : undefined,
      booleanLiteral: isSet(object.booleanLiteral)
        ? Boolean(object.booleanLiteral)
        : undefined,
      nullLiteral: isSet(object.nullLiteral)
        ? NullLiteral.fromJSON(object.nullLiteral)
        : undefined,
      undefinedLiteral: isSet(object.undefinedLiteral)
        ? UndefinedLiteral.fromJSON(object.undefinedLiteral)
        : undefined,
      bigintLiteral: isSet(object.bigintLiteral)
        ? String(object.bigintLiteral)
        : undefined,
    };
  },

  toJSON(message: Literal): unknown {
    const obj: any = {};
    message.stringLiteral !== undefined &&
      (obj.stringLiteral = message.stringLiteral);
    message.numberLiteral !== undefined &&
      (obj.numberLiteral = Math.round(message.numberLiteral));
    message.booleanLiteral !== undefined &&
      (obj.booleanLiteral = message.booleanLiteral);
    message.nullLiteral !== undefined &&
      (obj.nullLiteral = message.nullLiteral
        ? NullLiteral.toJSON(message.nullLiteral)
        : undefined);
    message.undefinedLiteral !== undefined &&
      (obj.undefinedLiteral = message.undefinedLiteral
        ? UndefinedLiteral.toJSON(message.undefinedLiteral)
        : undefined);
    message.bigintLiteral !== undefined &&
      (obj.bigintLiteral = message.bigintLiteral);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Literal>, I>>(object: I): Literal {
    const message = createBaseLiteral();
    message.stringLiteral = object.stringLiteral ?? undefined;
    message.numberLiteral = object.numberLiteral ?? undefined;
    message.booleanLiteral = object.booleanLiteral ?? undefined;
    message.nullLiteral =
      object.nullLiteral !== undefined && object.nullLiteral !== null
        ? NullLiteral.fromPartial(object.nullLiteral)
        : undefined;
    message.undefinedLiteral =
      object.undefinedLiteral !== undefined && object.undefinedLiteral !== null
        ? UndefinedLiteral.fromPartial(object.undefinedLiteral)
        : undefined;
    message.bigintLiteral = object.bigintLiteral ?? undefined;
    return message;
  },
};

function createBaseOptionalPattern(): OptionalPattern {
  return { identifier: undefined, expression: undefined };
}

export const OptionalPattern = {
  encode(
    message: OptionalPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    if (message.expression !== undefined) {
      Expression.encode(message.expression, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): OptionalPattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseOptionalPattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.expression = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): OptionalPattern {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      expression: isSet(object.expression)
        ? Expression.fromJSON(object.expression)
        : undefined,
    };
  },

  toJSON(message: OptionalPattern): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.expression !== undefined &&
      (obj.expression = message.expression
        ? Expression.toJSON(message.expression)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<OptionalPattern>, I>>(
    object: I
  ): OptionalPattern {
    const message = createBaseOptionalPattern();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.expression =
      object.expression !== undefined && object.expression !== null
        ? Expression.fromPartial(object.expression)
        : undefined;
    return message;
  },
};

function createBaseParameterPattern(): ParameterPattern {
  return { pattern: undefined, optional: undefined };
}

export const ParameterPattern = {
  encode(
    message: ParameterPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.pattern !== undefined) {
      Pattern.encode(message.pattern, writer.uint32(10).fork()).ldelim();
    }
    if (message.optional !== undefined) {
      OptionalPattern.encode(
        message.optional,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ParameterPattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseParameterPattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.pattern = Pattern.decode(reader, reader.uint32());
          break;
        case 2:
          message.optional = OptionalPattern.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ParameterPattern {
    return {
      pattern: isSet(object.pattern)
        ? Pattern.fromJSON(object.pattern)
        : undefined,
      optional: isSet(object.optional)
        ? OptionalPattern.fromJSON(object.optional)
        : undefined,
    };
  },

  toJSON(message: ParameterPattern): unknown {
    const obj: any = {};
    message.pattern !== undefined &&
      (obj.pattern = message.pattern
        ? Pattern.toJSON(message.pattern)
        : undefined);
    message.optional !== undefined &&
      (obj.optional = message.optional
        ? OptionalPattern.toJSON(message.optional)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ParameterPattern>, I>>(
    object: I
  ): ParameterPattern {
    const message = createBaseParameterPattern();
    message.pattern =
      object.pattern !== undefined && object.pattern !== null
        ? Pattern.fromPartial(object.pattern)
        : undefined;
    message.optional =
      object.optional !== undefined && object.optional !== null
        ? OptionalPattern.fromPartial(object.optional)
        : undefined;
    return message;
  },
};

function createBaseArrayPattern(): ArrayPattern {
  return { elements: [] };
}

export const ArrayPattern = {
  encode(
    message: ArrayPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.elements) {
      ParameterPattern.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ArrayPattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseArrayPattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.elements.push(
            ParameterPattern.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ArrayPattern {
    return {
      elements: Array.isArray(object?.elements)
        ? object.elements.map((e: any) => ParameterPattern.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ArrayPattern): unknown {
    const obj: any = {};
    if (message.elements) {
      obj.elements = message.elements.map((e) =>
        e ? ParameterPattern.toJSON(e) : undefined
      );
    } else {
      obj.elements = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ArrayPattern>, I>>(
    object: I
  ): ArrayPattern {
    const message = createBaseArrayPattern();
    message.elements =
      object.elements?.map((e) => ParameterPattern.fromPartial(e)) || [];
    return message;
  },
};

function createBasePropName(): PropName {
  return {
    stringLiteral: undefined,
    identifier: undefined,
    numberLiteral: undefined,
  };
}

export const PropName = {
  encode(
    message: PropName,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.stringLiteral !== undefined) {
      writer.uint32(10).string(message.stringLiteral);
    }
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(18).fork()).ldelim();
    }
    if (message.numberLiteral !== undefined) {
      writer.uint32(24).uint64(message.numberLiteral);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PropName {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePropName();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.stringLiteral = reader.string();
          break;
        case 2:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 3:
          message.numberLiteral = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PropName {
    return {
      stringLiteral: isSet(object.stringLiteral)
        ? String(object.stringLiteral)
        : undefined,
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      numberLiteral: isSet(object.numberLiteral)
        ? Number(object.numberLiteral)
        : undefined,
    };
  },

  toJSON(message: PropName): unknown {
    const obj: any = {};
    message.stringLiteral !== undefined &&
      (obj.stringLiteral = message.stringLiteral);
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.numberLiteral !== undefined &&
      (obj.numberLiteral = Math.round(message.numberLiteral));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PropName>, I>>(object: I): PropName {
    const message = createBasePropName();
    message.stringLiteral = object.stringLiteral ?? undefined;
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.numberLiteral = object.numberLiteral ?? undefined;
    return message;
  },
};

function createBaseObjectPattern(): ObjectPattern {
  return { elements: [] };
}

export const ObjectPattern = {
  encode(
    message: ObjectPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.elements) {
      ObjectPattern_Element.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ObjectPattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectPattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.elements.push(
            ObjectPattern_Element.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectPattern {
    return {
      elements: Array.isArray(object?.elements)
        ? object.elements.map((e: any) => ObjectPattern_Element.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ObjectPattern): unknown {
    const obj: any = {};
    if (message.elements) {
      obj.elements = message.elements.map((e) =>
        e ? ObjectPattern_Element.toJSON(e) : undefined
      );
    } else {
      obj.elements = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectPattern>, I>>(
    object: I
  ): ObjectPattern {
    const message = createBaseObjectPattern();
    message.elements =
      object.elements?.map((e) => ObjectPattern_Element.fromPartial(e)) || [];
    return message;
  },
};

function createBaseObjectPattern_Property(): ObjectPattern_Property {
  return { name: undefined, pattern: undefined };
}

export const ObjectPattern_Property = {
  encode(
    message: ObjectPattern_Property,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== undefined) {
      PropName.encode(message.name, writer.uint32(10).fork()).ldelim();
    }
    if (message.pattern !== undefined) {
      Pattern.encode(message.pattern, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectPattern_Property {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectPattern_Property();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = PropName.decode(reader, reader.uint32());
          break;
        case 2:
          message.pattern = Pattern.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectPattern_Property {
    return {
      name: isSet(object.name) ? PropName.fromJSON(object.name) : undefined,
      pattern: isSet(object.pattern)
        ? Pattern.fromJSON(object.pattern)
        : undefined,
    };
  },

  toJSON(message: ObjectPattern_Property): unknown {
    const obj: any = {};
    message.name !== undefined &&
      (obj.name = message.name ? PropName.toJSON(message.name) : undefined);
    message.pattern !== undefined &&
      (obj.pattern = message.pattern
        ? Pattern.toJSON(message.pattern)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectPattern_Property>, I>>(
    object: I
  ): ObjectPattern_Property {
    const message = createBaseObjectPattern_Property();
    message.name =
      object.name !== undefined && object.name !== null
        ? PropName.fromPartial(object.name)
        : undefined;
    message.pattern =
      object.pattern !== undefined && object.pattern !== null
        ? Pattern.fromPartial(object.pattern)
        : undefined;
    return message;
  },
};

function createBaseObjectPattern_Element(): ObjectPattern_Element {
  return {
    property: undefined,
    shorthand: undefined,
    optional: undefined,
    restPattern: undefined,
  };
}

export const ObjectPattern_Element = {
  encode(
    message: ObjectPattern_Element,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.property !== undefined) {
      ObjectPattern_Property.encode(
        message.property,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.shorthand !== undefined) {
      Identifier.encode(message.shorthand, writer.uint32(18).fork()).ldelim();
    }
    if (message.optional !== undefined) {
      OptionalPattern.encode(
        message.optional,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.restPattern !== undefined) {
      Pattern.encode(message.restPattern, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectPattern_Element {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectPattern_Element();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.property = ObjectPattern_Property.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.shorthand = Identifier.decode(reader, reader.uint32());
          break;
        case 3:
          message.optional = OptionalPattern.decode(reader, reader.uint32());
          break;
        case 4:
          message.restPattern = Pattern.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectPattern_Element {
    return {
      property: isSet(object.property)
        ? ObjectPattern_Property.fromJSON(object.property)
        : undefined,
      shorthand: isSet(object.shorthand)
        ? Identifier.fromJSON(object.shorthand)
        : undefined,
      optional: isSet(object.optional)
        ? OptionalPattern.fromJSON(object.optional)
        : undefined,
      restPattern: isSet(object.restPattern)
        ? Pattern.fromJSON(object.restPattern)
        : undefined,
    };
  },

  toJSON(message: ObjectPattern_Element): unknown {
    const obj: any = {};
    message.property !== undefined &&
      (obj.property = message.property
        ? ObjectPattern_Property.toJSON(message.property)
        : undefined);
    message.shorthand !== undefined &&
      (obj.shorthand = message.shorthand
        ? Identifier.toJSON(message.shorthand)
        : undefined);
    message.optional !== undefined &&
      (obj.optional = message.optional
        ? OptionalPattern.toJSON(message.optional)
        : undefined);
    message.restPattern !== undefined &&
      (obj.restPattern = message.restPattern
        ? Pattern.toJSON(message.restPattern)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectPattern_Element>, I>>(
    object: I
  ): ObjectPattern_Element {
    const message = createBaseObjectPattern_Element();
    message.property =
      object.property !== undefined && object.property !== null
        ? ObjectPattern_Property.fromPartial(object.property)
        : undefined;
    message.shorthand =
      object.shorthand !== undefined && object.shorthand !== null
        ? Identifier.fromPartial(object.shorthand)
        : undefined;
    message.optional =
      object.optional !== undefined && object.optional !== null
        ? OptionalPattern.fromPartial(object.optional)
        : undefined;
    message.restPattern =
      object.restPattern !== undefined && object.restPattern !== null
        ? Pattern.fromPartial(object.restPattern)
        : undefined;
    return message;
  },
};

function createBaseBindingPattern(): BindingPattern {
  return { array: undefined, object: undefined };
}

export const BindingPattern = {
  encode(
    message: BindingPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.array !== undefined) {
      ArrayPattern.encode(message.array, writer.uint32(10).fork()).ldelim();
    }
    if (message.object !== undefined) {
      ObjectPattern.encode(message.object, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BindingPattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBindingPattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.array = ArrayPattern.decode(reader, reader.uint32());
          break;
        case 2:
          message.object = ObjectPattern.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BindingPattern {
    return {
      array: isSet(object.array)
        ? ArrayPattern.fromJSON(object.array)
        : undefined,
      object: isSet(object.object)
        ? ObjectPattern.fromJSON(object.object)
        : undefined,
    };
  },

  toJSON(message: BindingPattern): unknown {
    const obj: any = {};
    message.array !== undefined &&
      (obj.array = message.array
        ? ArrayPattern.toJSON(message.array)
        : undefined);
    message.object !== undefined &&
      (obj.object = message.object
        ? ObjectPattern.toJSON(message.object)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<BindingPattern>, I>>(
    object: I
  ): BindingPattern {
    const message = createBaseBindingPattern();
    message.array =
      object.array !== undefined && object.array !== null
        ? ArrayPattern.fromPartial(object.array)
        : undefined;
    message.object =
      object.object !== undefined && object.object !== null
        ? ObjectPattern.fromPartial(object.object)
        : undefined;
    return message;
  },
};

function createBaseHole(): Hole {
  return {};
}

export const Hole = {
  encode(_: Hole, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Hole {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHole();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): Hole {
    return {};
  },

  toJSON(_: Hole): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Hole>, I>>(_: I): Hole {
    const message = createBaseHole();
    return message;
  },
};

function createBasePattern(): Pattern {
  return {
    identifier: undefined,
    literal: undefined,
    hole: undefined,
    binding: undefined,
    isRest: false,
  };
}

export const Pattern = {
  encode(
    message: Pattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    if (message.literal !== undefined) {
      Literal.encode(message.literal, writer.uint32(18).fork()).ldelim();
    }
    if (message.hole !== undefined) {
      Hole.encode(message.hole, writer.uint32(26).fork()).ldelim();
    }
    if (message.binding !== undefined) {
      BindingPattern.encode(message.binding, writer.uint32(34).fork()).ldelim();
    }
    if (message.isRest === true) {
      writer.uint32(120).bool(message.isRest);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Pattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.literal = Literal.decode(reader, reader.uint32());
          break;
        case 3:
          message.hole = Hole.decode(reader, reader.uint32());
          break;
        case 4:
          message.binding = BindingPattern.decode(reader, reader.uint32());
          break;
        case 15:
          message.isRest = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Pattern {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      literal: isSet(object.literal)
        ? Literal.fromJSON(object.literal)
        : undefined,
      hole: isSet(object.hole) ? Hole.fromJSON(object.hole) : undefined,
      binding: isSet(object.binding)
        ? BindingPattern.fromJSON(object.binding)
        : undefined,
      isRest: isSet(object.isRest) ? Boolean(object.isRest) : false,
    };
  },

  toJSON(message: Pattern): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.literal !== undefined &&
      (obj.literal = message.literal
        ? Literal.toJSON(message.literal)
        : undefined);
    message.hole !== undefined &&
      (obj.hole = message.hole ? Hole.toJSON(message.hole) : undefined);
    message.binding !== undefined &&
      (obj.binding = message.binding
        ? BindingPattern.toJSON(message.binding)
        : undefined);
    message.isRest !== undefined && (obj.isRest = message.isRest);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Pattern>, I>>(object: I): Pattern {
    const message = createBasePattern();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.literal =
      object.literal !== undefined && object.literal !== null
        ? Literal.fromPartial(object.literal)
        : undefined;
    message.hole =
      object.hole !== undefined && object.hole !== null
        ? Hole.fromPartial(object.hole)
        : undefined;
    message.binding =
      object.binding !== undefined && object.binding !== null
        ? BindingPattern.fromPartial(object.binding)
        : undefined;
    message.isRest = object.isRest ?? false;
    return message;
  },
};

function createBaseStatement(): Statement {
  return {
    variableDeclaration: undefined,
    functionDeclaration: undefined,
    blockStatement: undefined,
    ifStatement: undefined,
    forStatement: undefined,
    forOfStatement: undefined,
    whileStatement: undefined,
    switchStatement: undefined,
    tryStatement: undefined,
    breakStatement: undefined,
    continueStatement: undefined,
    returnStatement: undefined,
    throwStatement: undefined,
    expressionStatement: undefined,
  };
}

export const Statement = {
  encode(
    message: Statement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.variableDeclaration !== undefined) {
      VariableDeclaration.encode(
        message.variableDeclaration,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.functionDeclaration !== undefined) {
      FunctionDeclaration.encode(
        message.functionDeclaration,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.blockStatement !== undefined) {
      BlockStatement.encode(
        message.blockStatement,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.ifStatement !== undefined) {
      IfStatement.encode(
        message.ifStatement,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.forStatement !== undefined) {
      ForStatement.encode(
        message.forStatement,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.forOfStatement !== undefined) {
      ForOfStatement.encode(
        message.forOfStatement,
        writer.uint32(50).fork()
      ).ldelim();
    }
    if (message.whileStatement !== undefined) {
      WhileStatement.encode(
        message.whileStatement,
        writer.uint32(58).fork()
      ).ldelim();
    }
    if (message.switchStatement !== undefined) {
      SwitchStatement.encode(
        message.switchStatement,
        writer.uint32(66).fork()
      ).ldelim();
    }
    if (message.tryStatement !== undefined) {
      TryStatement.encode(
        message.tryStatement,
        writer.uint32(74).fork()
      ).ldelim();
    }
    if (message.breakStatement !== undefined) {
      BreakStatement.encode(
        message.breakStatement,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.continueStatement !== undefined) {
      ContinueStatement.encode(
        message.continueStatement,
        writer.uint32(90).fork()
      ).ldelim();
    }
    if (message.returnStatement !== undefined) {
      ReturnStatement.encode(
        message.returnStatement,
        writer.uint32(98).fork()
      ).ldelim();
    }
    if (message.throwStatement !== undefined) {
      ThrowStatement.encode(
        message.throwStatement,
        writer.uint32(106).fork()
      ).ldelim();
    }
    if (message.expressionStatement !== undefined) {
      ExpressionStatement.encode(
        message.expressionStatement,
        writer.uint32(114).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Statement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.variableDeclaration = VariableDeclaration.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.functionDeclaration = FunctionDeclaration.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.blockStatement = BlockStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.ifStatement = IfStatement.decode(reader, reader.uint32());
          break;
        case 5:
          message.forStatement = ForStatement.decode(reader, reader.uint32());
          break;
        case 6:
          message.forOfStatement = ForOfStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 7:
          message.whileStatement = WhileStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 8:
          message.switchStatement = SwitchStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 9:
          message.tryStatement = TryStatement.decode(reader, reader.uint32());
          break;
        case 10:
          message.breakStatement = BreakStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 11:
          message.continueStatement = ContinueStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 12:
          message.returnStatement = ReturnStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 13:
          message.throwStatement = ThrowStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 14:
          message.expressionStatement = ExpressionStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Statement {
    return {
      variableDeclaration: isSet(object.variableDeclaration)
        ? VariableDeclaration.fromJSON(object.variableDeclaration)
        : undefined,
      functionDeclaration: isSet(object.functionDeclaration)
        ? FunctionDeclaration.fromJSON(object.functionDeclaration)
        : undefined,
      blockStatement: isSet(object.blockStatement)
        ? BlockStatement.fromJSON(object.blockStatement)
        : undefined,
      ifStatement: isSet(object.ifStatement)
        ? IfStatement.fromJSON(object.ifStatement)
        : undefined,
      forStatement: isSet(object.forStatement)
        ? ForStatement.fromJSON(object.forStatement)
        : undefined,
      forOfStatement: isSet(object.forOfStatement)
        ? ForOfStatement.fromJSON(object.forOfStatement)
        : undefined,
      whileStatement: isSet(object.whileStatement)
        ? WhileStatement.fromJSON(object.whileStatement)
        : undefined,
      switchStatement: isSet(object.switchStatement)
        ? SwitchStatement.fromJSON(object.switchStatement)
        : undefined,
      tryStatement: isSet(object.tryStatement)
        ? TryStatement.fromJSON(object.tryStatement)
        : undefined,
      breakStatement: isSet(object.breakStatement)
        ? BreakStatement.fromJSON(object.breakStatement)
        : undefined,
      continueStatement: isSet(object.continueStatement)
        ? ContinueStatement.fromJSON(object.continueStatement)
        : undefined,
      returnStatement: isSet(object.returnStatement)
        ? ReturnStatement.fromJSON(object.returnStatement)
        : undefined,
      throwStatement: isSet(object.throwStatement)
        ? ThrowStatement.fromJSON(object.throwStatement)
        : undefined,
      expressionStatement: isSet(object.expressionStatement)
        ? ExpressionStatement.fromJSON(object.expressionStatement)
        : undefined,
    };
  },

  toJSON(message: Statement): unknown {
    const obj: any = {};
    message.variableDeclaration !== undefined &&
      (obj.variableDeclaration = message.variableDeclaration
        ? VariableDeclaration.toJSON(message.variableDeclaration)
        : undefined);
    message.functionDeclaration !== undefined &&
      (obj.functionDeclaration = message.functionDeclaration
        ? FunctionDeclaration.toJSON(message.functionDeclaration)
        : undefined);
    message.blockStatement !== undefined &&
      (obj.blockStatement = message.blockStatement
        ? BlockStatement.toJSON(message.blockStatement)
        : undefined);
    message.ifStatement !== undefined &&
      (obj.ifStatement = message.ifStatement
        ? IfStatement.toJSON(message.ifStatement)
        : undefined);
    message.forStatement !== undefined &&
      (obj.forStatement = message.forStatement
        ? ForStatement.toJSON(message.forStatement)
        : undefined);
    message.forOfStatement !== undefined &&
      (obj.forOfStatement = message.forOfStatement
        ? ForOfStatement.toJSON(message.forOfStatement)
        : undefined);
    message.whileStatement !== undefined &&
      (obj.whileStatement = message.whileStatement
        ? WhileStatement.toJSON(message.whileStatement)
        : undefined);
    message.switchStatement !== undefined &&
      (obj.switchStatement = message.switchStatement
        ? SwitchStatement.toJSON(message.switchStatement)
        : undefined);
    message.tryStatement !== undefined &&
      (obj.tryStatement = message.tryStatement
        ? TryStatement.toJSON(message.tryStatement)
        : undefined);
    message.breakStatement !== undefined &&
      (obj.breakStatement = message.breakStatement
        ? BreakStatement.toJSON(message.breakStatement)
        : undefined);
    message.continueStatement !== undefined &&
      (obj.continueStatement = message.continueStatement
        ? ContinueStatement.toJSON(message.continueStatement)
        : undefined);
    message.returnStatement !== undefined &&
      (obj.returnStatement = message.returnStatement
        ? ReturnStatement.toJSON(message.returnStatement)
        : undefined);
    message.throwStatement !== undefined &&
      (obj.throwStatement = message.throwStatement
        ? ThrowStatement.toJSON(message.throwStatement)
        : undefined);
    message.expressionStatement !== undefined &&
      (obj.expressionStatement = message.expressionStatement
        ? ExpressionStatement.toJSON(message.expressionStatement)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Statement>, I>>(
    object: I
  ): Statement {
    const message = createBaseStatement();
    message.variableDeclaration =
      object.variableDeclaration !== undefined &&
      object.variableDeclaration !== null
        ? VariableDeclaration.fromPartial(object.variableDeclaration)
        : undefined;
    message.functionDeclaration =
      object.functionDeclaration !== undefined &&
      object.functionDeclaration !== null
        ? FunctionDeclaration.fromPartial(object.functionDeclaration)
        : undefined;
    message.blockStatement =
      object.blockStatement !== undefined && object.blockStatement !== null
        ? BlockStatement.fromPartial(object.blockStatement)
        : undefined;
    message.ifStatement =
      object.ifStatement !== undefined && object.ifStatement !== null
        ? IfStatement.fromPartial(object.ifStatement)
        : undefined;
    message.forStatement =
      object.forStatement !== undefined && object.forStatement !== null
        ? ForStatement.fromPartial(object.forStatement)
        : undefined;
    message.forOfStatement =
      object.forOfStatement !== undefined && object.forOfStatement !== null
        ? ForOfStatement.fromPartial(object.forOfStatement)
        : undefined;
    message.whileStatement =
      object.whileStatement !== undefined && object.whileStatement !== null
        ? WhileStatement.fromPartial(object.whileStatement)
        : undefined;
    message.switchStatement =
      object.switchStatement !== undefined && object.switchStatement !== null
        ? SwitchStatement.fromPartial(object.switchStatement)
        : undefined;
    message.tryStatement =
      object.tryStatement !== undefined && object.tryStatement !== null
        ? TryStatement.fromPartial(object.tryStatement)
        : undefined;
    message.breakStatement =
      object.breakStatement !== undefined && object.breakStatement !== null
        ? BreakStatement.fromPartial(object.breakStatement)
        : undefined;
    message.continueStatement =
      object.continueStatement !== undefined &&
      object.continueStatement !== null
        ? ContinueStatement.fromPartial(object.continueStatement)
        : undefined;
    message.returnStatement =
      object.returnStatement !== undefined && object.returnStatement !== null
        ? ReturnStatement.fromPartial(object.returnStatement)
        : undefined;
    message.throwStatement =
      object.throwStatement !== undefined && object.throwStatement !== null
        ? ThrowStatement.fromPartial(object.throwStatement)
        : undefined;
    message.expressionStatement =
      object.expressionStatement !== undefined &&
      object.expressionStatement !== null
        ? ExpressionStatement.fromPartial(object.expressionStatement)
        : undefined;
    return message;
  },
};

function createBaseVariableDeclarator(): VariableDeclarator {
  return { normal: undefined, binding: undefined };
}

export const VariableDeclarator = {
  encode(
    message: VariableDeclarator,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.normal !== undefined) {
      VariableDeclarator_NormalDeclarator.encode(
        message.normal,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.binding !== undefined) {
      VariableDeclarator_BindingDeclarator.encode(
        message.binding,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): VariableDeclarator {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVariableDeclarator();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.normal = VariableDeclarator_NormalDeclarator.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.binding = VariableDeclarator_BindingDeclarator.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): VariableDeclarator {
    return {
      normal: isSet(object.normal)
        ? VariableDeclarator_NormalDeclarator.fromJSON(object.normal)
        : undefined,
      binding: isSet(object.binding)
        ? VariableDeclarator_BindingDeclarator.fromJSON(object.binding)
        : undefined,
    };
  },

  toJSON(message: VariableDeclarator): unknown {
    const obj: any = {};
    message.normal !== undefined &&
      (obj.normal = message.normal
        ? VariableDeclarator_NormalDeclarator.toJSON(message.normal)
        : undefined);
    message.binding !== undefined &&
      (obj.binding = message.binding
        ? VariableDeclarator_BindingDeclarator.toJSON(message.binding)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<VariableDeclarator>, I>>(
    object: I
  ): VariableDeclarator {
    const message = createBaseVariableDeclarator();
    message.normal =
      object.normal !== undefined && object.normal !== null
        ? VariableDeclarator_NormalDeclarator.fromPartial(object.normal)
        : undefined;
    message.binding =
      object.binding !== undefined && object.binding !== null
        ? VariableDeclarator_BindingDeclarator.fromPartial(object.binding)
        : undefined;
    return message;
  },
};

function createBaseVariableDeclarator_NormalDeclarator(): VariableDeclarator_NormalDeclarator {
  return { identifier: undefined, value: undefined };
}

export const VariableDeclarator_NormalDeclarator = {
  encode(
    message: VariableDeclarator_NormalDeclarator,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    if (message.value !== undefined) {
      Expression.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): VariableDeclarator_NormalDeclarator {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVariableDeclarator_NormalDeclarator();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.value = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): VariableDeclarator_NormalDeclarator {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      value: isSet(object.value)
        ? Expression.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: VariableDeclarator_NormalDeclarator): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.value !== undefined &&
      (obj.value = message.value
        ? Expression.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<VariableDeclarator_NormalDeclarator>, I>
  >(object: I): VariableDeclarator_NormalDeclarator {
    const message = createBaseVariableDeclarator_NormalDeclarator();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.value =
      object.value !== undefined && object.value !== null
        ? Expression.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseVariableDeclarator_BindingDeclarator(): VariableDeclarator_BindingDeclarator {
  return { pattern: undefined, value: undefined };
}

export const VariableDeclarator_BindingDeclarator = {
  encode(
    message: VariableDeclarator_BindingDeclarator,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.pattern !== undefined) {
      BindingPattern.encode(message.pattern, writer.uint32(10).fork()).ldelim();
    }
    if (message.value !== undefined) {
      Expression.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): VariableDeclarator_BindingDeclarator {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVariableDeclarator_BindingDeclarator();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.pattern = BindingPattern.decode(reader, reader.uint32());
          break;
        case 2:
          message.value = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): VariableDeclarator_BindingDeclarator {
    return {
      pattern: isSet(object.pattern)
        ? BindingPattern.fromJSON(object.pattern)
        : undefined,
      value: isSet(object.value)
        ? Expression.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: VariableDeclarator_BindingDeclarator): unknown {
    const obj: any = {};
    message.pattern !== undefined &&
      (obj.pattern = message.pattern
        ? BindingPattern.toJSON(message.pattern)
        : undefined);
    message.value !== undefined &&
      (obj.value = message.value
        ? Expression.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<VariableDeclarator_BindingDeclarator>, I>
  >(object: I): VariableDeclarator_BindingDeclarator {
    const message = createBaseVariableDeclarator_BindingDeclarator();
    message.pattern =
      object.pattern !== undefined && object.pattern !== null
        ? BindingPattern.fromPartial(object.pattern)
        : undefined;
    message.value =
      object.value !== undefined && object.value !== null
        ? Expression.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseVariableDeclaration(): VariableDeclaration {
  return { kind: 0, declarators: [] };
}

export const VariableDeclaration = {
  encode(
    message: VariableDeclaration,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.kind !== 0) {
      writer.uint32(8).int32(message.kind);
    }
    for (const v of message.declarators) {
      VariableDeclarator.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): VariableDeclaration {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVariableDeclaration();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.kind = reader.int32() as any;
          break;
        case 2:
          message.declarators.push(
            VariableDeclarator.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): VariableDeclaration {
    return {
      kind: isSet(object.kind) ? declarationKindFromJSON(object.kind) : 0,
      declarators: Array.isArray(object?.declarators)
        ? object.declarators.map((e: any) => VariableDeclarator.fromJSON(e))
        : [],
    };
  },

  toJSON(message: VariableDeclaration): unknown {
    const obj: any = {};
    message.kind !== undefined &&
      (obj.kind = declarationKindToJSON(message.kind));
    if (message.declarators) {
      obj.declarators = message.declarators.map((e) =>
        e ? VariableDeclarator.toJSON(e) : undefined
      );
    } else {
      obj.declarators = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<VariableDeclaration>, I>>(
    object: I
  ): VariableDeclaration {
    const message = createBaseVariableDeclaration();
    message.kind = object.kind ?? 0;
    message.declarators =
      object.declarators?.map((e) => VariableDeclarator.fromPartial(e)) || [];
    return message;
  },
};

function createBaseFunctionDeclaration(): FunctionDeclaration {
  return { function: undefined };
}

export const FunctionDeclaration = {
  encode(
    message: FunctionDeclaration,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.function !== undefined) {
      FunctionExpression.encode(
        message.function,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FunctionDeclaration {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFunctionDeclaration();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.function = FunctionExpression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FunctionDeclaration {
    return {
      function: isSet(object.function)
        ? FunctionExpression.fromJSON(object.function)
        : undefined,
    };
  },

  toJSON(message: FunctionDeclaration): unknown {
    const obj: any = {};
    message.function !== undefined &&
      (obj.function = message.function
        ? FunctionExpression.toJSON(message.function)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FunctionDeclaration>, I>>(
    object: I
  ): FunctionDeclaration {
    const message = createBaseFunctionDeclaration();
    message.function =
      object.function !== undefined && object.function !== null
        ? FunctionExpression.fromPartial(object.function)
        : undefined;
    return message;
  },
};

function createBaseBlockStatement(): BlockStatement {
  return { body: [] };
}

export const BlockStatement = {
  encode(
    message: BlockStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.body) {
      Statement.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BlockStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBlockStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.body.push(Statement.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BlockStatement {
    return {
      body: Array.isArray(object?.body)
        ? object.body.map((e: any) => Statement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: BlockStatement): unknown {
    const obj: any = {};
    if (message.body) {
      obj.body = message.body.map((e) => (e ? Statement.toJSON(e) : undefined));
    } else {
      obj.body = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<BlockStatement>, I>>(
    object: I
  ): BlockStatement {
    const message = createBaseBlockStatement();
    message.body = object.body?.map((e) => Statement.fromPartial(e)) || [];
    return message;
  },
};

function createBaseIfStatement(): IfStatement {
  return { test: undefined, consequent: undefined, alternate: undefined };
}

export const IfStatement = {
  encode(
    message: IfStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.test !== undefined) {
      Expression.encode(message.test, writer.uint32(10).fork()).ldelim();
    }
    if (message.consequent !== undefined) {
      Statement.encode(message.consequent, writer.uint32(18).fork()).ldelim();
    }
    if (message.alternate !== undefined) {
      Statement.encode(message.alternate, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): IfStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseIfStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.consequent = Statement.decode(reader, reader.uint32());
          break;
        case 3:
          message.alternate = Statement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IfStatement {
    return {
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      consequent: isSet(object.consequent)
        ? Statement.fromJSON(object.consequent)
        : undefined,
      alternate: isSet(object.alternate)
        ? Statement.fromJSON(object.alternate)
        : undefined,
    };
  },

  toJSON(message: IfStatement): unknown {
    const obj: any = {};
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    message.consequent !== undefined &&
      (obj.consequent = message.consequent
        ? Statement.toJSON(message.consequent)
        : undefined);
    message.alternate !== undefined &&
      (obj.alternate = message.alternate
        ? Statement.toJSON(message.alternate)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<IfStatement>, I>>(
    object: I
  ): IfStatement {
    const message = createBaseIfStatement();
    message.test =
      object.test !== undefined && object.test !== null
        ? Expression.fromPartial(object.test)
        : undefined;
    message.consequent =
      object.consequent !== undefined && object.consequent !== null
        ? Statement.fromPartial(object.consequent)
        : undefined;
    message.alternate =
      object.alternate !== undefined && object.alternate !== null
        ? Statement.fromPartial(object.alternate)
        : undefined;
    return message;
  },
};

function createBaseForStatement(): ForStatement {
  return {
    kind: 0,
    init: undefined,
    test: undefined,
    update: undefined,
    body: undefined,
  };
}

export const ForStatement = {
  encode(
    message: ForStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.kind !== 0) {
      writer.uint32(8).int32(message.kind);
    }
    if (message.init !== undefined) {
      VariableDeclarator.encode(
        message.init,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.test !== undefined) {
      Expression.encode(message.test, writer.uint32(26).fork()).ldelim();
    }
    if (message.update !== undefined) {
      Expression.encode(message.update, writer.uint32(34).fork()).ldelim();
    }
    if (message.body !== undefined) {
      BlockStatement.encode(message.body, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ForStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseForStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.kind = reader.int32() as any;
          break;
        case 2:
          message.init = VariableDeclarator.decode(reader, reader.uint32());
          break;
        case 3:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 4:
          message.update = Expression.decode(reader, reader.uint32());
          break;
        case 5:
          message.body = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ForStatement {
    return {
      kind: isSet(object.kind) ? declarationKindFromJSON(object.kind) : 0,
      init: isSet(object.init)
        ? VariableDeclarator.fromJSON(object.init)
        : undefined,
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      update: isSet(object.update)
        ? Expression.fromJSON(object.update)
        : undefined,
      body: isSet(object.body)
        ? BlockStatement.fromJSON(object.body)
        : undefined,
    };
  },

  toJSON(message: ForStatement): unknown {
    const obj: any = {};
    message.kind !== undefined &&
      (obj.kind = declarationKindToJSON(message.kind));
    message.init !== undefined &&
      (obj.init = message.init
        ? VariableDeclarator.toJSON(message.init)
        : undefined);
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    message.update !== undefined &&
      (obj.update = message.update
        ? Expression.toJSON(message.update)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body
        ? BlockStatement.toJSON(message.body)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ForStatement>, I>>(
    object: I
  ): ForStatement {
    const message = createBaseForStatement();
    message.kind = object.kind ?? 0;
    message.init =
      object.init !== undefined && object.init !== null
        ? VariableDeclarator.fromPartial(object.init)
        : undefined;
    message.test =
      object.test !== undefined && object.test !== null
        ? Expression.fromPartial(object.test)
        : undefined;
    message.update =
      object.update !== undefined && object.update !== null
        ? Expression.fromPartial(object.update)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? BlockStatement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseForOfStatement(): ForOfStatement {
  return { kind: 0, declarator: undefined, body: undefined };
}

export const ForOfStatement = {
  encode(
    message: ForOfStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.kind !== 0) {
      writer.uint32(8).int32(message.kind);
    }
    if (message.declarator !== undefined) {
      VariableDeclarator.encode(
        message.declarator,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.body !== undefined) {
      Statement.encode(message.body, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ForOfStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseForOfStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.kind = reader.int32() as any;
          break;
        case 2:
          message.declarator = VariableDeclarator.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.body = Statement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ForOfStatement {
    return {
      kind: isSet(object.kind) ? declarationKindFromJSON(object.kind) : 0,
      declarator: isSet(object.declarator)
        ? VariableDeclarator.fromJSON(object.declarator)
        : undefined,
      body: isSet(object.body) ? Statement.fromJSON(object.body) : undefined,
    };
  },

  toJSON(message: ForOfStatement): unknown {
    const obj: any = {};
    message.kind !== undefined &&
      (obj.kind = declarationKindToJSON(message.kind));
    message.declarator !== undefined &&
      (obj.declarator = message.declarator
        ? VariableDeclarator.toJSON(message.declarator)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body ? Statement.toJSON(message.body) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ForOfStatement>, I>>(
    object: I
  ): ForOfStatement {
    const message = createBaseForOfStatement();
    message.kind = object.kind ?? 0;
    message.declarator =
      object.declarator !== undefined && object.declarator !== null
        ? VariableDeclarator.fromPartial(object.declarator)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? Statement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseWhileStatement(): WhileStatement {
  return { test: undefined, body: undefined };
}

export const WhileStatement = {
  encode(
    message: WhileStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.test !== undefined) {
      Expression.encode(message.test, writer.uint32(10).fork()).ldelim();
    }
    if (message.body !== undefined) {
      Statement.encode(message.body, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): WhileStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseWhileStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.body = Statement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): WhileStatement {
    return {
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      body: isSet(object.body) ? Statement.fromJSON(object.body) : undefined,
    };
  },

  toJSON(message: WhileStatement): unknown {
    const obj: any = {};
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    message.body !== undefined &&
      (obj.body = message.body ? Statement.toJSON(message.body) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<WhileStatement>, I>>(
    object: I
  ): WhileStatement {
    const message = createBaseWhileStatement();
    message.test =
      object.test !== undefined && object.test !== null
        ? Expression.fromPartial(object.test)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? Statement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseSwitchStatement(): SwitchStatement {
  return { discriminant: undefined, cases: [] };
}

export const SwitchStatement = {
  encode(
    message: SwitchStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.discriminant !== undefined) {
      Expression.encode(
        message.discriminant,
        writer.uint32(10).fork()
      ).ldelim();
    }
    for (const v of message.cases) {
      SwitchStatement_Case.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SwitchStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSwitchStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.discriminant = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.cases.push(
            SwitchStatement_Case.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SwitchStatement {
    return {
      discriminant: isSet(object.discriminant)
        ? Expression.fromJSON(object.discriminant)
        : undefined,
      cases: Array.isArray(object?.cases)
        ? object.cases.map((e: any) => SwitchStatement_Case.fromJSON(e))
        : [],
    };
  },

  toJSON(message: SwitchStatement): unknown {
    const obj: any = {};
    message.discriminant !== undefined &&
      (obj.discriminant = message.discriminant
        ? Expression.toJSON(message.discriminant)
        : undefined);
    if (message.cases) {
      obj.cases = message.cases.map((e) =>
        e ? SwitchStatement_Case.toJSON(e) : undefined
      );
    } else {
      obj.cases = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SwitchStatement>, I>>(
    object: I
  ): SwitchStatement {
    const message = createBaseSwitchStatement();
    message.discriminant =
      object.discriminant !== undefined && object.discriminant !== null
        ? Expression.fromPartial(object.discriminant)
        : undefined;
    message.cases =
      object.cases?.map((e) => SwitchStatement_Case.fromPartial(e)) || [];
    return message;
  },
};

function createBaseSwitchStatement_CaseLabel(): SwitchStatement_CaseLabel {
  return { test: undefined, default: undefined };
}

export const SwitchStatement_CaseLabel = {
  encode(
    message: SwitchStatement_CaseLabel,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.test !== undefined) {
      Expression.encode(message.test, writer.uint32(10).fork()).ldelim();
    }
    if (message.default !== undefined) {
      SwitchStatement_CaseLabel_Default.encode(
        message.default,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): SwitchStatement_CaseLabel {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSwitchStatement_CaseLabel();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.default = SwitchStatement_CaseLabel_Default.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SwitchStatement_CaseLabel {
    return {
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      default: isSet(object.default)
        ? SwitchStatement_CaseLabel_Default.fromJSON(object.default)
        : undefined,
    };
  },

  toJSON(message: SwitchStatement_CaseLabel): unknown {
    const obj: any = {};
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    message.default !== undefined &&
      (obj.default = message.default
        ? SwitchStatement_CaseLabel_Default.toJSON(message.default)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SwitchStatement_CaseLabel>, I>>(
    object: I
  ): SwitchStatement_CaseLabel {
    const message = createBaseSwitchStatement_CaseLabel();
    message.test =
      object.test !== undefined && object.test !== null
        ? Expression.fromPartial(object.test)
        : undefined;
    message.default =
      object.default !== undefined && object.default !== null
        ? SwitchStatement_CaseLabel_Default.fromPartial(object.default)
        : undefined;
    return message;
  },
};

function createBaseSwitchStatement_CaseLabel_Default(): SwitchStatement_CaseLabel_Default {
  return {};
}

export const SwitchStatement_CaseLabel_Default = {
  encode(
    _: SwitchStatement_CaseLabel_Default,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): SwitchStatement_CaseLabel_Default {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSwitchStatement_CaseLabel_Default();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): SwitchStatement_CaseLabel_Default {
    return {};
  },

  toJSON(_: SwitchStatement_CaseLabel_Default): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<SwitchStatement_CaseLabel_Default>, I>
  >(_: I): SwitchStatement_CaseLabel_Default {
    const message = createBaseSwitchStatement_CaseLabel_Default();
    return message;
  },
};

function createBaseSwitchStatement_Case(): SwitchStatement_Case {
  return { labels: [], consequent: undefined };
}

export const SwitchStatement_Case = {
  encode(
    message: SwitchStatement_Case,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.labels) {
      SwitchStatement_CaseLabel.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.consequent !== undefined) {
      BlockStatement.encode(
        message.consequent,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): SwitchStatement_Case {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSwitchStatement_Case();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.labels.push(
            SwitchStatement_CaseLabel.decode(reader, reader.uint32())
          );
          break;
        case 2:
          message.consequent = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SwitchStatement_Case {
    return {
      labels: Array.isArray(object?.labels)
        ? object.labels.map((e: any) => SwitchStatement_CaseLabel.fromJSON(e))
        : [],
      consequent: isSet(object.consequent)
        ? BlockStatement.fromJSON(object.consequent)
        : undefined,
    };
  },

  toJSON(message: SwitchStatement_Case): unknown {
    const obj: any = {};
    if (message.labels) {
      obj.labels = message.labels.map((e) =>
        e ? SwitchStatement_CaseLabel.toJSON(e) : undefined
      );
    } else {
      obj.labels = [];
    }
    message.consequent !== undefined &&
      (obj.consequent = message.consequent
        ? BlockStatement.toJSON(message.consequent)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SwitchStatement_Case>, I>>(
    object: I
  ): SwitchStatement_Case {
    const message = createBaseSwitchStatement_Case();
    message.labels =
      object.labels?.map((e) => SwitchStatement_CaseLabel.fromPartial(e)) || [];
    message.consequent =
      object.consequent !== undefined && object.consequent !== null
        ? BlockStatement.fromPartial(object.consequent)
        : undefined;
    return message;
  },
};

function createBaseTryStatement(): TryStatement {
  return { block: undefined, handler: undefined, finalizer: undefined };
}

export const TryStatement = {
  encode(
    message: TryStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.block !== undefined) {
      BlockStatement.encode(message.block, writer.uint32(10).fork()).ldelim();
    }
    if (message.handler !== undefined) {
      TryStatement_CatchClause.encode(
        message.handler,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.finalizer !== undefined) {
      BlockStatement.encode(
        message.finalizer,
        writer.uint32(26).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TryStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTryStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.block = BlockStatement.decode(reader, reader.uint32());
          break;
        case 2:
          message.handler = TryStatement_CatchClause.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.finalizer = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): TryStatement {
    return {
      block: isSet(object.block)
        ? BlockStatement.fromJSON(object.block)
        : undefined,
      handler: isSet(object.handler)
        ? TryStatement_CatchClause.fromJSON(object.handler)
        : undefined,
      finalizer: isSet(object.finalizer)
        ? BlockStatement.fromJSON(object.finalizer)
        : undefined,
    };
  },

  toJSON(message: TryStatement): unknown {
    const obj: any = {};
    message.block !== undefined &&
      (obj.block = message.block
        ? BlockStatement.toJSON(message.block)
        : undefined);
    message.handler !== undefined &&
      (obj.handler = message.handler
        ? TryStatement_CatchClause.toJSON(message.handler)
        : undefined);
    message.finalizer !== undefined &&
      (obj.finalizer = message.finalizer
        ? BlockStatement.toJSON(message.finalizer)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<TryStatement>, I>>(
    object: I
  ): TryStatement {
    const message = createBaseTryStatement();
    message.block =
      object.block !== undefined && object.block !== null
        ? BlockStatement.fromPartial(object.block)
        : undefined;
    message.handler =
      object.handler !== undefined && object.handler !== null
        ? TryStatement_CatchClause.fromPartial(object.handler)
        : undefined;
    message.finalizer =
      object.finalizer !== undefined && object.finalizer !== null
        ? BlockStatement.fromPartial(object.finalizer)
        : undefined;
    return message;
  },
};

function createBaseTryStatement_CatchClause(): TryStatement_CatchClause {
  return { pattern: undefined, body: undefined };
}

export const TryStatement_CatchClause = {
  encode(
    message: TryStatement_CatchClause,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.pattern !== undefined) {
      Pattern.encode(message.pattern, writer.uint32(10).fork()).ldelim();
    }
    if (message.body !== undefined) {
      BlockStatement.encode(message.body, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): TryStatement_CatchClause {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTryStatement_CatchClause();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.pattern = Pattern.decode(reader, reader.uint32());
          break;
        case 2:
          message.body = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): TryStatement_CatchClause {
    return {
      pattern: isSet(object.pattern)
        ? Pattern.fromJSON(object.pattern)
        : undefined,
      body: isSet(object.body)
        ? BlockStatement.fromJSON(object.body)
        : undefined,
    };
  },

  toJSON(message: TryStatement_CatchClause): unknown {
    const obj: any = {};
    message.pattern !== undefined &&
      (obj.pattern = message.pattern
        ? Pattern.toJSON(message.pattern)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body
        ? BlockStatement.toJSON(message.body)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<TryStatement_CatchClause>, I>>(
    object: I
  ): TryStatement_CatchClause {
    const message = createBaseTryStatement_CatchClause();
    message.pattern =
      object.pattern !== undefined && object.pattern !== null
        ? Pattern.fromPartial(object.pattern)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? BlockStatement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseBreakStatement(): BreakStatement {
  return {};
}

export const BreakStatement = {
  encode(
    _: BreakStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BreakStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBreakStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): BreakStatement {
    return {};
  },

  toJSON(_: BreakStatement): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<BreakStatement>, I>>(
    _: I
  ): BreakStatement {
    const message = createBaseBreakStatement();
    return message;
  },
};

function createBaseContinueStatement(): ContinueStatement {
  return {};
}

export const ContinueStatement = {
  encode(
    _: ContinueStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ContinueStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseContinueStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): ContinueStatement {
    return {};
  },

  toJSON(_: ContinueStatement): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ContinueStatement>, I>>(
    _: I
  ): ContinueStatement {
    const message = createBaseContinueStatement();
    return message;
  },
};

function createBaseReturnStatement(): ReturnStatement {
  return { argument: undefined };
}

export const ReturnStatement = {
  encode(
    message: ReturnStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.argument !== undefined) {
      Expression.encode(message.argument, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ReturnStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseReturnStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.argument = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ReturnStatement {
    return {
      argument: isSet(object.argument)
        ? Expression.fromJSON(object.argument)
        : undefined,
    };
  },

  toJSON(message: ReturnStatement): unknown {
    const obj: any = {};
    message.argument !== undefined &&
      (obj.argument = message.argument
        ? Expression.toJSON(message.argument)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ReturnStatement>, I>>(
    object: I
  ): ReturnStatement {
    const message = createBaseReturnStatement();
    message.argument =
      object.argument !== undefined && object.argument !== null
        ? Expression.fromPartial(object.argument)
        : undefined;
    return message;
  },
};

function createBaseThrowStatement(): ThrowStatement {
  return { argument: undefined };
}

export const ThrowStatement = {
  encode(
    message: ThrowStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.argument !== undefined) {
      Expression.encode(message.argument, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ThrowStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseThrowStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.argument = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ThrowStatement {
    return {
      argument: isSet(object.argument)
        ? Expression.fromJSON(object.argument)
        : undefined,
    };
  },

  toJSON(message: ThrowStatement): unknown {
    const obj: any = {};
    message.argument !== undefined &&
      (obj.argument = message.argument
        ? Expression.toJSON(message.argument)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ThrowStatement>, I>>(
    object: I
  ): ThrowStatement {
    const message = createBaseThrowStatement();
    message.argument =
      object.argument !== undefined && object.argument !== null
        ? Expression.fromPartial(object.argument)
        : undefined;
    return message;
  },
};

function createBaseExpressionStatement(): ExpressionStatement {
  return { expression: undefined };
}

export const ExpressionStatement = {
  encode(
    message: ExpressionStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.expression !== undefined) {
      Expression.encode(message.expression, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ExpressionStatement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExpressionStatement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.expression = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ExpressionStatement {
    return {
      expression: isSet(object.expression)
        ? Expression.fromJSON(object.expression)
        : undefined,
    };
  },

  toJSON(message: ExpressionStatement): unknown {
    const obj: any = {};
    message.expression !== undefined &&
      (obj.expression = message.expression
        ? Expression.toJSON(message.expression)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ExpressionStatement>, I>>(
    object: I
  ): ExpressionStatement {
    const message = createBaseExpressionStatement();
    message.expression =
      object.expression !== undefined && object.expression !== null
        ? Expression.fromPartial(object.expression)
        : undefined;
    return message;
  },
};

function createBaseExpression(): Expression {
  return {
    literal: undefined,
    array: undefined,
    object: undefined,
    function: undefined,
    arrowFunction: undefined,
    binary: undefined,
    unary: undefined,
    conditional: undefined,
    logical: undefined,
    update: undefined,
    variable: undefined,
    assignment: undefined,
    member: undefined,
    call: undefined,
  };
}

export const Expression = {
  encode(
    message: Expression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.literal !== undefined) {
      LiteralExpression.encode(
        message.literal,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.array !== undefined) {
      ArrayExpression.encode(message.array, writer.uint32(18).fork()).ldelim();
    }
    if (message.object !== undefined) {
      ObjectExpression.encode(
        message.object,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.function !== undefined) {
      FunctionExpression.encode(
        message.function,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.arrowFunction !== undefined) {
      ArrowFunctionExpression.encode(
        message.arrowFunction,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.binary !== undefined) {
      BinaryExpression.encode(
        message.binary,
        writer.uint32(50).fork()
      ).ldelim();
    }
    if (message.unary !== undefined) {
      UnaryExpression.encode(message.unary, writer.uint32(58).fork()).ldelim();
    }
    if (message.conditional !== undefined) {
      ConditionalExpression.encode(
        message.conditional,
        writer.uint32(66).fork()
      ).ldelim();
    }
    if (message.logical !== undefined) {
      LogicalExpression.encode(
        message.logical,
        writer.uint32(74).fork()
      ).ldelim();
    }
    if (message.update !== undefined) {
      UpdateExpression.encode(
        message.update,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.variable !== undefined) {
      VariableExpression.encode(
        message.variable,
        writer.uint32(90).fork()
      ).ldelim();
    }
    if (message.assignment !== undefined) {
      AssignmentExpression.encode(
        message.assignment,
        writer.uint32(98).fork()
      ).ldelim();
    }
    if (message.member !== undefined) {
      MemberExpression.encode(
        message.member,
        writer.uint32(106).fork()
      ).ldelim();
    }
    if (message.call !== undefined) {
      CallExpression.encode(message.call, writer.uint32(114).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Expression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.literal = LiteralExpression.decode(reader, reader.uint32());
          break;
        case 2:
          message.array = ArrayExpression.decode(reader, reader.uint32());
          break;
        case 3:
          message.object = ObjectExpression.decode(reader, reader.uint32());
          break;
        case 4:
          message.function = FunctionExpression.decode(reader, reader.uint32());
          break;
        case 5:
          message.arrowFunction = ArrowFunctionExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 6:
          message.binary = BinaryExpression.decode(reader, reader.uint32());
          break;
        case 7:
          message.unary = UnaryExpression.decode(reader, reader.uint32());
          break;
        case 8:
          message.conditional = ConditionalExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 9:
          message.logical = LogicalExpression.decode(reader, reader.uint32());
          break;
        case 10:
          message.update = UpdateExpression.decode(reader, reader.uint32());
          break;
        case 11:
          message.variable = VariableExpression.decode(reader, reader.uint32());
          break;
        case 12:
          message.assignment = AssignmentExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 13:
          message.member = MemberExpression.decode(reader, reader.uint32());
          break;
        case 14:
          message.call = CallExpression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Expression {
    return {
      literal: isSet(object.literal)
        ? LiteralExpression.fromJSON(object.literal)
        : undefined,
      array: isSet(object.array)
        ? ArrayExpression.fromJSON(object.array)
        : undefined,
      object: isSet(object.object)
        ? ObjectExpression.fromJSON(object.object)
        : undefined,
      function: isSet(object.function)
        ? FunctionExpression.fromJSON(object.function)
        : undefined,
      arrowFunction: isSet(object.arrowFunction)
        ? ArrowFunctionExpression.fromJSON(object.arrowFunction)
        : undefined,
      binary: isSet(object.binary)
        ? BinaryExpression.fromJSON(object.binary)
        : undefined,
      unary: isSet(object.unary)
        ? UnaryExpression.fromJSON(object.unary)
        : undefined,
      conditional: isSet(object.conditional)
        ? ConditionalExpression.fromJSON(object.conditional)
        : undefined,
      logical: isSet(object.logical)
        ? LogicalExpression.fromJSON(object.logical)
        : undefined,
      update: isSet(object.update)
        ? UpdateExpression.fromJSON(object.update)
        : undefined,
      variable: isSet(object.variable)
        ? VariableExpression.fromJSON(object.variable)
        : undefined,
      assignment: isSet(object.assignment)
        ? AssignmentExpression.fromJSON(object.assignment)
        : undefined,
      member: isSet(object.member)
        ? MemberExpression.fromJSON(object.member)
        : undefined,
      call: isSet(object.call)
        ? CallExpression.fromJSON(object.call)
        : undefined,
    };
  },

  toJSON(message: Expression): unknown {
    const obj: any = {};
    message.literal !== undefined &&
      (obj.literal = message.literal
        ? LiteralExpression.toJSON(message.literal)
        : undefined);
    message.array !== undefined &&
      (obj.array = message.array
        ? ArrayExpression.toJSON(message.array)
        : undefined);
    message.object !== undefined &&
      (obj.object = message.object
        ? ObjectExpression.toJSON(message.object)
        : undefined);
    message.function !== undefined &&
      (obj.function = message.function
        ? FunctionExpression.toJSON(message.function)
        : undefined);
    message.arrowFunction !== undefined &&
      (obj.arrowFunction = message.arrowFunction
        ? ArrowFunctionExpression.toJSON(message.arrowFunction)
        : undefined);
    message.binary !== undefined &&
      (obj.binary = message.binary
        ? BinaryExpression.toJSON(message.binary)
        : undefined);
    message.unary !== undefined &&
      (obj.unary = message.unary
        ? UnaryExpression.toJSON(message.unary)
        : undefined);
    message.conditional !== undefined &&
      (obj.conditional = message.conditional
        ? ConditionalExpression.toJSON(message.conditional)
        : undefined);
    message.logical !== undefined &&
      (obj.logical = message.logical
        ? LogicalExpression.toJSON(message.logical)
        : undefined);
    message.update !== undefined &&
      (obj.update = message.update
        ? UpdateExpression.toJSON(message.update)
        : undefined);
    message.variable !== undefined &&
      (obj.variable = message.variable
        ? VariableExpression.toJSON(message.variable)
        : undefined);
    message.assignment !== undefined &&
      (obj.assignment = message.assignment
        ? AssignmentExpression.toJSON(message.assignment)
        : undefined);
    message.member !== undefined &&
      (obj.member = message.member
        ? MemberExpression.toJSON(message.member)
        : undefined);
    message.call !== undefined &&
      (obj.call = message.call
        ? CallExpression.toJSON(message.call)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Expression>, I>>(
    object: I
  ): Expression {
    const message = createBaseExpression();
    message.literal =
      object.literal !== undefined && object.literal !== null
        ? LiteralExpression.fromPartial(object.literal)
        : undefined;
    message.array =
      object.array !== undefined && object.array !== null
        ? ArrayExpression.fromPartial(object.array)
        : undefined;
    message.object =
      object.object !== undefined && object.object !== null
        ? ObjectExpression.fromPartial(object.object)
        : undefined;
    message.function =
      object.function !== undefined && object.function !== null
        ? FunctionExpression.fromPartial(object.function)
        : undefined;
    message.arrowFunction =
      object.arrowFunction !== undefined && object.arrowFunction !== null
        ? ArrowFunctionExpression.fromPartial(object.arrowFunction)
        : undefined;
    message.binary =
      object.binary !== undefined && object.binary !== null
        ? BinaryExpression.fromPartial(object.binary)
        : undefined;
    message.unary =
      object.unary !== undefined && object.unary !== null
        ? UnaryExpression.fromPartial(object.unary)
        : undefined;
    message.conditional =
      object.conditional !== undefined && object.conditional !== null
        ? ConditionalExpression.fromPartial(object.conditional)
        : undefined;
    message.logical =
      object.logical !== undefined && object.logical !== null
        ? LogicalExpression.fromPartial(object.logical)
        : undefined;
    message.update =
      object.update !== undefined && object.update !== null
        ? UpdateExpression.fromPartial(object.update)
        : undefined;
    message.variable =
      object.variable !== undefined && object.variable !== null
        ? VariableExpression.fromPartial(object.variable)
        : undefined;
    message.assignment =
      object.assignment !== undefined && object.assignment !== null
        ? AssignmentExpression.fromPartial(object.assignment)
        : undefined;
    message.member =
      object.member !== undefined && object.member !== null
        ? MemberExpression.fromPartial(object.member)
        : undefined;
    message.call =
      object.call !== undefined && object.call !== null
        ? CallExpression.fromPartial(object.call)
        : undefined;
    return message;
  },
};

function createBaseParameterElement(): ParameterElement {
  return { element: undefined, spreadElement: undefined };
}

export const ParameterElement = {
  encode(
    message: ParameterElement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.element !== undefined) {
      Expression.encode(message.element, writer.uint32(10).fork()).ldelim();
    }
    if (message.spreadElement !== undefined) {
      Expression.encode(
        message.spreadElement,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ParameterElement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseParameterElement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.element = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.spreadElement = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ParameterElement {
    return {
      element: isSet(object.element)
        ? Expression.fromJSON(object.element)
        : undefined,
      spreadElement: isSet(object.spreadElement)
        ? Expression.fromJSON(object.spreadElement)
        : undefined,
    };
  },

  toJSON(message: ParameterElement): unknown {
    const obj: any = {};
    message.element !== undefined &&
      (obj.element = message.element
        ? Expression.toJSON(message.element)
        : undefined);
    message.spreadElement !== undefined &&
      (obj.spreadElement = message.spreadElement
        ? Expression.toJSON(message.spreadElement)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ParameterElement>, I>>(
    object: I
  ): ParameterElement {
    const message = createBaseParameterElement();
    message.element =
      object.element !== undefined && object.element !== null
        ? Expression.fromPartial(object.element)
        : undefined;
    message.spreadElement =
      object.spreadElement !== undefined && object.spreadElement !== null
        ? Expression.fromPartial(object.spreadElement)
        : undefined;
    return message;
  },
};

function createBaseLiteralExpression(): LiteralExpression {
  return { literal: undefined };
}

export const LiteralExpression = {
  encode(
    message: LiteralExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.literal !== undefined) {
      Literal.encode(message.literal, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LiteralExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLiteralExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.literal = Literal.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LiteralExpression {
    return {
      literal: isSet(object.literal)
        ? Literal.fromJSON(object.literal)
        : undefined,
    };
  },

  toJSON(message: LiteralExpression): unknown {
    const obj: any = {};
    message.literal !== undefined &&
      (obj.literal = message.literal
        ? Literal.toJSON(message.literal)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LiteralExpression>, I>>(
    object: I
  ): LiteralExpression {
    const message = createBaseLiteralExpression();
    message.literal =
      object.literal !== undefined && object.literal !== null
        ? Literal.fromPartial(object.literal)
        : undefined;
    return message;
  },
};

function createBaseArrayExpression(): ArrayExpression {
  return { elements: [] };
}

export const ArrayExpression = {
  encode(
    message: ArrayExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.elements) {
      ParameterElement.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ArrayExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseArrayExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.elements.push(
            ParameterElement.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ArrayExpression {
    return {
      elements: Array.isArray(object?.elements)
        ? object.elements.map((e: any) => ParameterElement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ArrayExpression): unknown {
    const obj: any = {};
    if (message.elements) {
      obj.elements = message.elements.map((e) =>
        e ? ParameterElement.toJSON(e) : undefined
      );
    } else {
      obj.elements = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ArrayExpression>, I>>(
    object: I
  ): ArrayExpression {
    const message = createBaseArrayExpression();
    message.elements =
      object.elements?.map((e) => ParameterElement.fromPartial(e)) || [];
    return message;
  },
};

function createBaseObjectExpression(): ObjectExpression {
  return { elements: [] };
}

export const ObjectExpression = {
  encode(
    message: ObjectExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.elements) {
      ObjectExpression_Element.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ObjectExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.elements.push(
            ObjectExpression_Element.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectExpression {
    return {
      elements: Array.isArray(object?.elements)
        ? object.elements.map((e: any) => ObjectExpression_Element.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ObjectExpression): unknown {
    const obj: any = {};
    if (message.elements) {
      obj.elements = message.elements.map((e) =>
        e ? ObjectExpression_Element.toJSON(e) : undefined
      );
    } else {
      obj.elements = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression>, I>>(
    object: I
  ): ObjectExpression {
    const message = createBaseObjectExpression();
    message.elements =
      object.elements?.map((e) => ObjectExpression_Element.fromPartial(e)) ||
      [];
    return message;
  },
};

function createBaseObjectExpression_Property(): ObjectExpression_Property {
  return { name: undefined, value: undefined };
}

export const ObjectExpression_Property = {
  encode(
    message: ObjectExpression_Property,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== undefined) {
      PropName.encode(message.name, writer.uint32(10).fork()).ldelim();
    }
    if (message.value !== undefined) {
      Expression.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectExpression_Property {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectExpression_Property();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = PropName.decode(reader, reader.uint32());
          break;
        case 2:
          message.value = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectExpression_Property {
    return {
      name: isSet(object.name) ? PropName.fromJSON(object.name) : undefined,
      value: isSet(object.value)
        ? Expression.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: ObjectExpression_Property): unknown {
    const obj: any = {};
    message.name !== undefined &&
      (obj.name = message.name ? PropName.toJSON(message.name) : undefined);
    message.value !== undefined &&
      (obj.value = message.value
        ? Expression.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression_Property>, I>>(
    object: I
  ): ObjectExpression_Property {
    const message = createBaseObjectExpression_Property();
    message.name =
      object.name !== undefined && object.name !== null
        ? PropName.fromPartial(object.name)
        : undefined;
    message.value =
      object.value !== undefined && object.value !== null
        ? Expression.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseObjectExpression_Method(): ObjectExpression_Method {
  return { method: undefined, getter: undefined, setter: undefined };
}

export const ObjectExpression_Method = {
  encode(
    message: ObjectExpression_Method,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.method !== undefined) {
      FunctionExpression.encode(
        message.method,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.getter !== undefined) {
      ObjectExpression_Method_Getter.encode(
        message.getter,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.setter !== undefined) {
      ObjectExpression_Method_Setter.encode(
        message.setter,
        writer.uint32(26).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectExpression_Method {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectExpression_Method();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.method = FunctionExpression.decode(reader, reader.uint32());
          break;
        case 2:
          message.getter = ObjectExpression_Method_Getter.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.setter = ObjectExpression_Method_Setter.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectExpression_Method {
    return {
      method: isSet(object.method)
        ? FunctionExpression.fromJSON(object.method)
        : undefined,
      getter: isSet(object.getter)
        ? ObjectExpression_Method_Getter.fromJSON(object.getter)
        : undefined,
      setter: isSet(object.setter)
        ? ObjectExpression_Method_Setter.fromJSON(object.setter)
        : undefined,
    };
  },

  toJSON(message: ObjectExpression_Method): unknown {
    const obj: any = {};
    message.method !== undefined &&
      (obj.method = message.method
        ? FunctionExpression.toJSON(message.method)
        : undefined);
    message.getter !== undefined &&
      (obj.getter = message.getter
        ? ObjectExpression_Method_Getter.toJSON(message.getter)
        : undefined);
    message.setter !== undefined &&
      (obj.setter = message.setter
        ? ObjectExpression_Method_Setter.toJSON(message.setter)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression_Method>, I>>(
    object: I
  ): ObjectExpression_Method {
    const message = createBaseObjectExpression_Method();
    message.method =
      object.method !== undefined && object.method !== null
        ? FunctionExpression.fromPartial(object.method)
        : undefined;
    message.getter =
      object.getter !== undefined && object.getter !== null
        ? ObjectExpression_Method_Getter.fromPartial(object.getter)
        : undefined;
    message.setter =
      object.setter !== undefined && object.setter !== null
        ? ObjectExpression_Method_Setter.fromPartial(object.setter)
        : undefined;
    return message;
  },
};

function createBaseObjectExpression_Method_Getter(): ObjectExpression_Method_Getter {
  return { name: undefined, body: undefined };
}

export const ObjectExpression_Method_Getter = {
  encode(
    message: ObjectExpression_Method_Getter,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== undefined) {
      PropName.encode(message.name, writer.uint32(10).fork()).ldelim();
    }
    if (message.body !== undefined) {
      BlockStatement.encode(message.body, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectExpression_Method_Getter {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectExpression_Method_Getter();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = PropName.decode(reader, reader.uint32());
          break;
        case 2:
          message.body = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectExpression_Method_Getter {
    return {
      name: isSet(object.name) ? PropName.fromJSON(object.name) : undefined,
      body: isSet(object.body)
        ? BlockStatement.fromJSON(object.body)
        : undefined,
    };
  },

  toJSON(message: ObjectExpression_Method_Getter): unknown {
    const obj: any = {};
    message.name !== undefined &&
      (obj.name = message.name ? PropName.toJSON(message.name) : undefined);
    message.body !== undefined &&
      (obj.body = message.body
        ? BlockStatement.toJSON(message.body)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression_Method_Getter>, I>>(
    object: I
  ): ObjectExpression_Method_Getter {
    const message = createBaseObjectExpression_Method_Getter();
    message.name =
      object.name !== undefined && object.name !== null
        ? PropName.fromPartial(object.name)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? BlockStatement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseObjectExpression_Method_Setter(): ObjectExpression_Method_Setter {
  return { name: undefined, param: undefined, body: undefined };
}

export const ObjectExpression_Method_Setter = {
  encode(
    message: ObjectExpression_Method_Setter,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== undefined) {
      PropName.encode(message.name, writer.uint32(10).fork()).ldelim();
    }
    if (message.param !== undefined) {
      ParameterPattern.encode(message.param, writer.uint32(18).fork()).ldelim();
    }
    if (message.body !== undefined) {
      BlockStatement.encode(message.body, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectExpression_Method_Setter {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectExpression_Method_Setter();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = PropName.decode(reader, reader.uint32());
          break;
        case 2:
          message.param = ParameterPattern.decode(reader, reader.uint32());
          break;
        case 3:
          message.body = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectExpression_Method_Setter {
    return {
      name: isSet(object.name) ? PropName.fromJSON(object.name) : undefined,
      param: isSet(object.param)
        ? ParameterPattern.fromJSON(object.param)
        : undefined,
      body: isSet(object.body)
        ? BlockStatement.fromJSON(object.body)
        : undefined,
    };
  },

  toJSON(message: ObjectExpression_Method_Setter): unknown {
    const obj: any = {};
    message.name !== undefined &&
      (obj.name = message.name ? PropName.toJSON(message.name) : undefined);
    message.param !== undefined &&
      (obj.param = message.param
        ? ParameterPattern.toJSON(message.param)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body
        ? BlockStatement.toJSON(message.body)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression_Method_Setter>, I>>(
    object: I
  ): ObjectExpression_Method_Setter {
    const message = createBaseObjectExpression_Method_Setter();
    message.name =
      object.name !== undefined && object.name !== null
        ? PropName.fromPartial(object.name)
        : undefined;
    message.param =
      object.param !== undefined && object.param !== null
        ? ParameterPattern.fromPartial(object.param)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? BlockStatement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseObjectExpression_Element(): ObjectExpression_Element {
  return {
    property: undefined,
    shorthand: undefined,
    method: undefined,
    spread: undefined,
  };
}

export const ObjectExpression_Element = {
  encode(
    message: ObjectExpression_Element,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.property !== undefined) {
      ObjectExpression_Property.encode(
        message.property,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.shorthand !== undefined) {
      Identifier.encode(message.shorthand, writer.uint32(18).fork()).ldelim();
    }
    if (message.method !== undefined) {
      ObjectExpression_Method.encode(
        message.method,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.spread !== undefined) {
      Expression.encode(message.spread, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ObjectExpression_Element {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectExpression_Element();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.property = ObjectExpression_Property.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.shorthand = Identifier.decode(reader, reader.uint32());
          break;
        case 3:
          message.method = ObjectExpression_Method.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.spread = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectExpression_Element {
    return {
      property: isSet(object.property)
        ? ObjectExpression_Property.fromJSON(object.property)
        : undefined,
      shorthand: isSet(object.shorthand)
        ? Identifier.fromJSON(object.shorthand)
        : undefined,
      method: isSet(object.method)
        ? ObjectExpression_Method.fromJSON(object.method)
        : undefined,
      spread: isSet(object.spread)
        ? Expression.fromJSON(object.spread)
        : undefined,
    };
  },

  toJSON(message: ObjectExpression_Element): unknown {
    const obj: any = {};
    message.property !== undefined &&
      (obj.property = message.property
        ? ObjectExpression_Property.toJSON(message.property)
        : undefined);
    message.shorthand !== undefined &&
      (obj.shorthand = message.shorthand
        ? Identifier.toJSON(message.shorthand)
        : undefined);
    message.method !== undefined &&
      (obj.method = message.method
        ? ObjectExpression_Method.toJSON(message.method)
        : undefined);
    message.spread !== undefined &&
      (obj.spread = message.spread
        ? Expression.toJSON(message.spread)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression_Element>, I>>(
    object: I
  ): ObjectExpression_Element {
    const message = createBaseObjectExpression_Element();
    message.property =
      object.property !== undefined && object.property !== null
        ? ObjectExpression_Property.fromPartial(object.property)
        : undefined;
    message.shorthand =
      object.shorthand !== undefined && object.shorthand !== null
        ? Identifier.fromPartial(object.shorthand)
        : undefined;
    message.method =
      object.method !== undefined && object.method !== null
        ? ObjectExpression_Method.fromPartial(object.method)
        : undefined;
    message.spread =
      object.spread !== undefined && object.spread !== null
        ? Expression.fromPartial(object.spread)
        : undefined;
    return message;
  },
};

function createBaseFunctionExpression(): FunctionExpression {
  return { identifier: undefined, parameters: [], body: undefined };
}

export const FunctionExpression = {
  encode(
    message: FunctionExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.parameters) {
      ParameterPattern.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.body !== undefined) {
      BlockStatement.encode(message.body, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FunctionExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFunctionExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.parameters.push(
            ParameterPattern.decode(reader, reader.uint32())
          );
          break;
        case 3:
          message.body = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FunctionExpression {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      parameters: Array.isArray(object?.parameters)
        ? object.parameters.map((e: any) => ParameterPattern.fromJSON(e))
        : [],
      body: isSet(object.body)
        ? BlockStatement.fromJSON(object.body)
        : undefined,
    };
  },

  toJSON(message: FunctionExpression): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    if (message.parameters) {
      obj.parameters = message.parameters.map((e) =>
        e ? ParameterPattern.toJSON(e) : undefined
      );
    } else {
      obj.parameters = [];
    }
    message.body !== undefined &&
      (obj.body = message.body
        ? BlockStatement.toJSON(message.body)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FunctionExpression>, I>>(
    object: I
  ): FunctionExpression {
    const message = createBaseFunctionExpression();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.parameters =
      object.parameters?.map((e) => ParameterPattern.fromPartial(e)) || [];
    message.body =
      object.body !== undefined && object.body !== null
        ? BlockStatement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseArrowFunctionExpression(): ArrowFunctionExpression {
  return { params: [], statement: undefined, expression: undefined };
}

export const ArrowFunctionExpression = {
  encode(
    message: ArrowFunctionExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.params) {
      ParameterPattern.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.statement !== undefined) {
      BlockStatement.encode(
        message.statement,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.expression !== undefined) {
      Expression.encode(message.expression, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ArrowFunctionExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseArrowFunctionExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.params.push(ParameterPattern.decode(reader, reader.uint32()));
          break;
        case 2:
          message.statement = BlockStatement.decode(reader, reader.uint32());
          break;
        case 3:
          message.expression = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ArrowFunctionExpression {
    return {
      params: Array.isArray(object?.params)
        ? object.params.map((e: any) => ParameterPattern.fromJSON(e))
        : [],
      statement: isSet(object.statement)
        ? BlockStatement.fromJSON(object.statement)
        : undefined,
      expression: isSet(object.expression)
        ? Expression.fromJSON(object.expression)
        : undefined,
    };
  },

  toJSON(message: ArrowFunctionExpression): unknown {
    const obj: any = {};
    if (message.params) {
      obj.params = message.params.map((e) =>
        e ? ParameterPattern.toJSON(e) : undefined
      );
    } else {
      obj.params = [];
    }
    message.statement !== undefined &&
      (obj.statement = message.statement
        ? BlockStatement.toJSON(message.statement)
        : undefined);
    message.expression !== undefined &&
      (obj.expression = message.expression
        ? Expression.toJSON(message.expression)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ArrowFunctionExpression>, I>>(
    object: I
  ): ArrowFunctionExpression {
    const message = createBaseArrowFunctionExpression();
    message.params =
      object.params?.map((e) => ParameterPattern.fromPartial(e)) || [];
    message.statement =
      object.statement !== undefined && object.statement !== null
        ? BlockStatement.fromPartial(object.statement)
        : undefined;
    message.expression =
      object.expression !== undefined && object.expression !== null
        ? Expression.fromPartial(object.expression)
        : undefined;
    return message;
  },
};

function createBaseBinaryExpression(): BinaryExpression {
  return {
    arithmetic: undefined,
    comparison: undefined,
    left: undefined,
    right: undefined,
  };
}

export const BinaryExpression = {
  encode(
    message: BinaryExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.arithmetic !== undefined) {
      writer.uint32(8).int32(message.arithmetic);
    }
    if (message.comparison !== undefined) {
      writer.uint32(16).int32(message.comparison);
    }
    if (message.left !== undefined) {
      Expression.encode(message.left, writer.uint32(26).fork()).ldelim();
    }
    if (message.right !== undefined) {
      Expression.encode(message.right, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BinaryExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBinaryExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.arithmetic = reader.int32() as any;
          break;
        case 2:
          message.comparison = reader.int32() as any;
          break;
        case 3:
          message.left = Expression.decode(reader, reader.uint32());
          break;
        case 4:
          message.right = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BinaryExpression {
    return {
      arithmetic: isSet(object.arithmetic)
        ? arithmeticOperatorFromJSON(object.arithmetic)
        : undefined,
      comparison: isSet(object.comparison)
        ? binaryExpression_ComparisonOperatorFromJSON(object.comparison)
        : undefined,
      left: isSet(object.left) ? Expression.fromJSON(object.left) : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
    };
  },

  toJSON(message: BinaryExpression): unknown {
    const obj: any = {};
    message.arithmetic !== undefined &&
      (obj.arithmetic =
        message.arithmetic !== undefined
          ? arithmeticOperatorToJSON(message.arithmetic)
          : undefined);
    message.comparison !== undefined &&
      (obj.comparison =
        message.comparison !== undefined
          ? binaryExpression_ComparisonOperatorToJSON(message.comparison)
          : undefined);
    message.left !== undefined &&
      (obj.left = message.left ? Expression.toJSON(message.left) : undefined);
    message.right !== undefined &&
      (obj.right = message.right
        ? Expression.toJSON(message.right)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<BinaryExpression>, I>>(
    object: I
  ): BinaryExpression {
    const message = createBaseBinaryExpression();
    message.arithmetic = object.arithmetic ?? undefined;
    message.comparison = object.comparison ?? undefined;
    message.left =
      object.left !== undefined && object.left !== null
        ? Expression.fromPartial(object.left)
        : undefined;
    message.right =
      object.right !== undefined && object.right !== null
        ? Expression.fromPartial(object.right)
        : undefined;
    return message;
  },
};

function createBaseUnaryExpression(): UnaryExpression {
  return { operator: 0, argument: undefined, prefix: false };
}

export const UnaryExpression = {
  encode(
    message: UnaryExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.operator !== 0) {
      writer.uint32(8).int32(message.operator);
    }
    if (message.argument !== undefined) {
      Expression.encode(message.argument, writer.uint32(18).fork()).ldelim();
    }
    if (message.prefix === true) {
      writer.uint32(24).bool(message.prefix);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UnaryExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUnaryExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.operator = reader.int32() as any;
          break;
        case 2:
          message.argument = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.prefix = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): UnaryExpression {
    return {
      operator: isSet(object.operator)
        ? unaryExpression_OperatorFromJSON(object.operator)
        : 0,
      argument: isSet(object.argument)
        ? Expression.fromJSON(object.argument)
        : undefined,
      prefix: isSet(object.prefix) ? Boolean(object.prefix) : false,
    };
  },

  toJSON(message: UnaryExpression): unknown {
    const obj: any = {};
    message.operator !== undefined &&
      (obj.operator = unaryExpression_OperatorToJSON(message.operator));
    message.argument !== undefined &&
      (obj.argument = message.argument
        ? Expression.toJSON(message.argument)
        : undefined);
    message.prefix !== undefined && (obj.prefix = message.prefix);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UnaryExpression>, I>>(
    object: I
  ): UnaryExpression {
    const message = createBaseUnaryExpression();
    message.operator = object.operator ?? 0;
    message.argument =
      object.argument !== undefined && object.argument !== null
        ? Expression.fromPartial(object.argument)
        : undefined;
    message.prefix = object.prefix ?? false;
    return message;
  },
};

function createBaseConditionalExpression(): ConditionalExpression {
  return { test: undefined, consequent: undefined, alternate: undefined };
}

export const ConditionalExpression = {
  encode(
    message: ConditionalExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.test !== undefined) {
      Expression.encode(message.test, writer.uint32(10).fork()).ldelim();
    }
    if (message.consequent !== undefined) {
      Expression.encode(message.consequent, writer.uint32(18).fork()).ldelim();
    }
    if (message.alternate !== undefined) {
      Expression.encode(message.alternate, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ConditionalExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseConditionalExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.consequent = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.alternate = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ConditionalExpression {
    return {
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      consequent: isSet(object.consequent)
        ? Expression.fromJSON(object.consequent)
        : undefined,
      alternate: isSet(object.alternate)
        ? Expression.fromJSON(object.alternate)
        : undefined,
    };
  },

  toJSON(message: ConditionalExpression): unknown {
    const obj: any = {};
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    message.consequent !== undefined &&
      (obj.consequent = message.consequent
        ? Expression.toJSON(message.consequent)
        : undefined);
    message.alternate !== undefined &&
      (obj.alternate = message.alternate
        ? Expression.toJSON(message.alternate)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ConditionalExpression>, I>>(
    object: I
  ): ConditionalExpression {
    const message = createBaseConditionalExpression();
    message.test =
      object.test !== undefined && object.test !== null
        ? Expression.fromPartial(object.test)
        : undefined;
    message.consequent =
      object.consequent !== undefined && object.consequent !== null
        ? Expression.fromPartial(object.consequent)
        : undefined;
    message.alternate =
      object.alternate !== undefined && object.alternate !== null
        ? Expression.fromPartial(object.alternate)
        : undefined;
    return message;
  },
};

function createBaseLogicalExpression(): LogicalExpression {
  return { operator: 0, left: undefined, right: undefined };
}

export const LogicalExpression = {
  encode(
    message: LogicalExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.operator !== 0) {
      writer.uint32(8).int32(message.operator);
    }
    if (message.left !== undefined) {
      Expression.encode(message.left, writer.uint32(18).fork()).ldelim();
    }
    if (message.right !== undefined) {
      Expression.encode(message.right, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogicalExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogicalExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.operator = reader.int32() as any;
          break;
        case 2:
          message.left = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.right = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LogicalExpression {
    return {
      operator: isSet(object.operator)
        ? logicalExpression_OperatorFromJSON(object.operator)
        : 0,
      left: isSet(object.left) ? Expression.fromJSON(object.left) : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
    };
  },

  toJSON(message: LogicalExpression): unknown {
    const obj: any = {};
    message.operator !== undefined &&
      (obj.operator = logicalExpression_OperatorToJSON(message.operator));
    message.left !== undefined &&
      (obj.left = message.left ? Expression.toJSON(message.left) : undefined);
    message.right !== undefined &&
      (obj.right = message.right
        ? Expression.toJSON(message.right)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogicalExpression>, I>>(
    object: I
  ): LogicalExpression {
    const message = createBaseLogicalExpression();
    message.operator = object.operator ?? 0;
    message.left =
      object.left !== undefined && object.left !== null
        ? Expression.fromPartial(object.left)
        : undefined;
    message.right =
      object.right !== undefined && object.right !== null
        ? Expression.fromPartial(object.right)
        : undefined;
    return message;
  },
};

function createBaseUpdateExpression(): UpdateExpression {
  return { operator: 0, argument: undefined, prefix: false };
}

export const UpdateExpression = {
  encode(
    message: UpdateExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.operator !== 0) {
      writer.uint32(8).int32(message.operator);
    }
    if (message.argument !== undefined) {
      Expression.encode(message.argument, writer.uint32(18).fork()).ldelim();
    }
    if (message.prefix === true) {
      writer.uint32(24).bool(message.prefix);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.operator = reader.int32() as any;
          break;
        case 2:
          message.argument = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.prefix = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): UpdateExpression {
    return {
      operator: isSet(object.operator)
        ? updateExpression_OperatorFromJSON(object.operator)
        : 0,
      argument: isSet(object.argument)
        ? Expression.fromJSON(object.argument)
        : undefined,
      prefix: isSet(object.prefix) ? Boolean(object.prefix) : false,
    };
  },

  toJSON(message: UpdateExpression): unknown {
    const obj: any = {};
    message.operator !== undefined &&
      (obj.operator = updateExpression_OperatorToJSON(message.operator));
    message.argument !== undefined &&
      (obj.argument = message.argument
        ? Expression.toJSON(message.argument)
        : undefined);
    message.prefix !== undefined && (obj.prefix = message.prefix);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UpdateExpression>, I>>(
    object: I
  ): UpdateExpression {
    const message = createBaseUpdateExpression();
    message.operator = object.operator ?? 0;
    message.argument =
      object.argument !== undefined && object.argument !== null
        ? Expression.fromPartial(object.argument)
        : undefined;
    message.prefix = object.prefix ?? false;
    return message;
  },
};

function createBaseVariableExpression(): VariableExpression {
  return { name: undefined };
}

export const VariableExpression = {
  encode(
    message: VariableExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== undefined) {
      Identifier.encode(message.name, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): VariableExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVariableExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = Identifier.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): VariableExpression {
    return {
      name: isSet(object.name) ? Identifier.fromJSON(object.name) : undefined,
    };
  },

  toJSON(message: VariableExpression): unknown {
    const obj: any = {};
    message.name !== undefined &&
      (obj.name = message.name ? Identifier.toJSON(message.name) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<VariableExpression>, I>>(
    object: I
  ): VariableExpression {
    const message = createBaseVariableExpression();
    message.name =
      object.name !== undefined && object.name !== null
        ? Identifier.fromPartial(object.name)
        : undefined;
    return message;
  },
};

function createBaseAssignmentExpression(): AssignmentExpression {
  return { operator: undefined, left: undefined, right: undefined };
}

export const AssignmentExpression = {
  encode(
    message: AssignmentExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.operator !== undefined) {
      writer.uint32(8).int32(message.operator);
    }
    if (message.left !== undefined) {
      AssignmentExpression_LValue.encode(
        message.left,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.right !== undefined) {
      Expression.encode(message.right, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): AssignmentExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAssignmentExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.operator = reader.int32() as any;
          break;
        case 2:
          message.left = AssignmentExpression_LValue.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.right = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AssignmentExpression {
    return {
      operator: isSet(object.operator)
        ? arithmeticOperatorFromJSON(object.operator)
        : undefined,
      left: isSet(object.left)
        ? AssignmentExpression_LValue.fromJSON(object.left)
        : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
    };
  },

  toJSON(message: AssignmentExpression): unknown {
    const obj: any = {};
    message.operator !== undefined &&
      (obj.operator =
        message.operator !== undefined
          ? arithmeticOperatorToJSON(message.operator)
          : undefined);
    message.left !== undefined &&
      (obj.left = message.left
        ? AssignmentExpression_LValue.toJSON(message.left)
        : undefined);
    message.right !== undefined &&
      (obj.right = message.right
        ? Expression.toJSON(message.right)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AssignmentExpression>, I>>(
    object: I
  ): AssignmentExpression {
    const message = createBaseAssignmentExpression();
    message.operator = object.operator ?? undefined;
    message.left =
      object.left !== undefined && object.left !== null
        ? AssignmentExpression_LValue.fromPartial(object.left)
        : undefined;
    message.right =
      object.right !== undefined && object.right !== null
        ? Expression.fromPartial(object.right)
        : undefined;
    return message;
  },
};

function createBaseAssignmentExpression_LValue(): AssignmentExpression_LValue {
  return { identifier: undefined, member: undefined };
}

export const AssignmentExpression_LValue = {
  encode(
    message: AssignmentExpression_LValue,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    if (message.member !== undefined) {
      MemberExpression.encode(
        message.member,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): AssignmentExpression_LValue {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAssignmentExpression_LValue();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.member = MemberExpression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AssignmentExpression_LValue {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      member: isSet(object.member)
        ? MemberExpression.fromJSON(object.member)
        : undefined,
    };
  },

  toJSON(message: AssignmentExpression_LValue): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.member !== undefined &&
      (obj.member = message.member
        ? MemberExpression.toJSON(message.member)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AssignmentExpression_LValue>, I>>(
    object: I
  ): AssignmentExpression_LValue {
    const message = createBaseAssignmentExpression_LValue();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.member =
      object.member !== undefined && object.member !== null
        ? MemberExpression.fromPartial(object.member)
        : undefined;
    return message;
  },
};

function createBaseMemberExpression(): MemberExpression {
  return { object: undefined, index: undefined, property: undefined };
}

export const MemberExpression = {
  encode(
    message: MemberExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.object !== undefined) {
      Expression.encode(message.object, writer.uint32(10).fork()).ldelim();
    }
    if (message.index !== undefined) {
      Expression.encode(message.index, writer.uint32(18).fork()).ldelim();
    }
    if (message.property !== undefined) {
      Identifier.encode(message.property, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): MemberExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMemberExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.object = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.index = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.property = Identifier.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MemberExpression {
    return {
      object: isSet(object.object)
        ? Expression.fromJSON(object.object)
        : undefined,
      index: isSet(object.index)
        ? Expression.fromJSON(object.index)
        : undefined,
      property: isSet(object.property)
        ? Identifier.fromJSON(object.property)
        : undefined,
    };
  },

  toJSON(message: MemberExpression): unknown {
    const obj: any = {};
    message.object !== undefined &&
      (obj.object = message.object
        ? Expression.toJSON(message.object)
        : undefined);
    message.index !== undefined &&
      (obj.index = message.index
        ? Expression.toJSON(message.index)
        : undefined);
    message.property !== undefined &&
      (obj.property = message.property
        ? Identifier.toJSON(message.property)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<MemberExpression>, I>>(
    object: I
  ): MemberExpression {
    const message = createBaseMemberExpression();
    message.object =
      object.object !== undefined && object.object !== null
        ? Expression.fromPartial(object.object)
        : undefined;
    message.index =
      object.index !== undefined && object.index !== null
        ? Expression.fromPartial(object.index)
        : undefined;
    message.property =
      object.property !== undefined && object.property !== null
        ? Identifier.fromPartial(object.property)
        : undefined;
    return message;
  },
};

function createBaseCallExpression(): CallExpression {
  return { callee: undefined, arguments: [] };
}

export const CallExpression = {
  encode(
    message: CallExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.callee !== undefined) {
      Expression.encode(message.callee, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.arguments) {
      ParameterElement.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CallExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCallExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.callee = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.arguments.push(
            ParameterElement.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CallExpression {
    return {
      callee: isSet(object.callee)
        ? Expression.fromJSON(object.callee)
        : undefined,
      arguments: Array.isArray(object?.arguments)
        ? object.arguments.map((e: any) => ParameterElement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: CallExpression): unknown {
    const obj: any = {};
    message.callee !== undefined &&
      (obj.callee = message.callee
        ? Expression.toJSON(message.callee)
        : undefined);
    if (message.arguments) {
      obj.arguments = message.arguments.map((e) =>
        e ? ParameterElement.toJSON(e) : undefined
      );
    } else {
      obj.arguments = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CallExpression>, I>>(
    object: I
  ): CallExpression {
    const message = createBaseCallExpression();
    message.callee =
      object.callee !== undefined && object.callee !== null
        ? Expression.fromPartial(object.callee)
        : undefined;
    message.arguments =
      object.arguments?.map((e) => ParameterElement.fromPartial(e)) || [];
    return message;
  },
};

function createBaseCallExpression_CallElement(): CallExpression_CallElement {
  return { element: undefined, spreadElement: undefined };
}

export const CallExpression_CallElement = {
  encode(
    message: CallExpression_CallElement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.element !== undefined) {
      Expression.encode(message.element, writer.uint32(10).fork()).ldelim();
    }
    if (message.spreadElement !== undefined) {
      Expression.encode(
        message.spreadElement,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): CallExpression_CallElement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCallExpression_CallElement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.element = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.spreadElement = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CallExpression_CallElement {
    return {
      element: isSet(object.element)
        ? Expression.fromJSON(object.element)
        : undefined,
      spreadElement: isSet(object.spreadElement)
        ? Expression.fromJSON(object.spreadElement)
        : undefined,
    };
  },

  toJSON(message: CallExpression_CallElement): unknown {
    const obj: any = {};
    message.element !== undefined &&
      (obj.element = message.element
        ? Expression.toJSON(message.element)
        : undefined);
    message.spreadElement !== undefined &&
      (obj.spreadElement = message.spreadElement
        ? Expression.toJSON(message.spreadElement)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CallExpression_CallElement>, I>>(
    object: I
  ): CallExpression_CallElement {
    const message = createBaseCallExpression_CallElement();
    message.element =
      object.element !== undefined && object.element !== null
        ? Expression.fromPartial(object.element)
        : undefined;
    message.spreadElement =
      object.spreadElement !== undefined && object.spreadElement !== null
        ? Expression.fromPartial(object.spreadElement)
        : undefined;
    return message;
  },
};

declare var self: any | undefined;
declare var window: any | undefined;
declare var global: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

type Builtin =
  | Date
  | Function
  | Uint8Array
  | string
  | number
  | boolean
  | undefined;

export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin
  ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & Record<
        Exclude<keyof I, KeysOfUnion<P>>,
        never
      >;

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
