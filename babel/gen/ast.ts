/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "nessie.ast";

export interface NullLiteral {}

export interface Literal {
  stringLiteral: string | undefined;
  /** must be under 2^53 */
  numberLiteral: number | undefined;
  booleanLiteral: boolean | undefined;
  nullLiteral: NullLiteral | undefined;
  /** must be stringified big integer */
  bigintLiteral: string | undefined;
}

export interface Statement {
  blockStatement: BlockStatement | undefined;
  breakStatement: BreakStatement | undefined;
  continueStatement: ContinueStatement | undefined;
  expressionStatement: Expression | undefined;
  forStatement: ForStatement | undefined;
  forOfStatement: ForOfStatement | undefined;
  functionDeclaration: FunctionDeclaration | undefined;
  ifStatement: IfStatement | undefined;
  returnStatement: ReturnStatement | undefined;
  switchStatement: SwitchStatement | undefined;
  throwStatement: ThrowStatement | undefined;
  tryStatement: TryStatement | undefined;
  variableDeclaration: VariableDeclaration | undefined;
  whileStatement: WhileStatement | undefined;
}

/** Whenever BlockStatement is used as an element, it is expanded as a list of statements. */
export interface BlockStatement {
  body: Statement[];
}

/** BreakStatement omits label */
export interface BreakStatement {}

/** ContinuseStatement omits label */
export interface ContinueStatement {}

export interface ForStatement {
  initDeclaration: VariableDeclaration | undefined;
  initExpression: Expression | undefined;
  test?: Expression | undefined;
  update?: Expression | undefined;
  body: Statement | undefined;
}

export interface ForOfStatement {
  leftDeclaration: VariableDeclaration | undefined;
  leftLval: LVal | undefined;
  right: Expression | undefined;
  body: Statement | undefined;
}

export interface AssignmentPattern {
  leftIdentifier: Identifier | undefined;
  leftObject: ObjectPattern | undefined;
  leftArray: ArrayPattern | undefined;
  leftMember: MemberExpression | undefined;
  right: Expression | undefined;
}

export interface ArrayPattern {
  elements: PatternLike[];
}

export interface ObjectPattern {
  properties: ObjectProperty[];
}

export interface ObjectProperty {
  keyIdentifier: Identifier | undefined;
  keyStringLiteral: string | undefined;
  keyNumericLiteral: number | undefined;
  keyBigintLiteral: string | undefined;
  keyExpression: Expression | undefined;
  valueExpression: Expression | undefined;
  valuePattern: PatternLike | undefined;
  computed: boolean;
  shorthand: boolean;
  rest?: LVal | undefined;
}

export interface PatternLike {
  identifier: Identifier | undefined;
  assignment: AssignmentPattern | undefined;
  array: ArrayPattern | undefined;
  object: ObjectPattern | undefined;
  restElement: LVal | undefined;
}

export interface LVal {
  identifier: Identifier | undefined;
  member: MemberExpression | undefined;
  assignment: AssignmentPattern | undefined;
  array: ArrayPattern | undefined;
  object: ObjectPattern | undefined;
  isRest?: boolean | undefined;
}

export interface FunctionDeclaration {
  id: Identifier | undefined;
  params: PatternLike[];
  rest?: LVal | undefined;
  body: Statement[];
}

export interface IfStatement {
  test: Expression | undefined;
  consequent: Statement | undefined;
  alternate?: Statement | undefined;
}

export interface ReturnStatement {
  argument?: Expression | undefined;
}

export interface SwitchStatement {
  discriminant: Expression | undefined;
  cases: SwitchCase[];
}

export interface SwitchCase {
  test?: Expression | undefined;
  consequent: Statement[];
}

export interface ThrowStatement {
  argument: Expression | undefined;
}

export interface TryStatement {
  block: Statement | undefined;
  handler?: CatchClause | undefined;
  finalizer?: BlockStatement | undefined;
}

export interface CatchClause {
  paramIdentifier: Identifier | undefined;
  paramArray: ArrayPattern | undefined;
  paramObject: ObjectPattern | undefined;
  body: BlockStatement | undefined;
}

export interface VariableDeclaration {
  kind: VariableDeclaration_Kind;
  declarators: VariableDeclarator[];
}

export enum VariableDeclaration_Kind {
  UNKNOWN = 0,
  LET = 1,
  CONST = 2,
  UNRECOGNIZED = -1,
}

export function variableDeclaration_KindFromJSON(
  object: any
): VariableDeclaration_Kind {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return VariableDeclaration_Kind.UNKNOWN;
    case 1:
    case "LET":
      return VariableDeclaration_Kind.LET;
    case 2:
    case "CONST":
      return VariableDeclaration_Kind.CONST;
    case -1:
    case "UNRECOGNIZED":
    default:
      return VariableDeclaration_Kind.UNRECOGNIZED;
  }
}

export function variableDeclaration_KindToJSON(
  object: VariableDeclaration_Kind
): string {
  switch (object) {
    case VariableDeclaration_Kind.UNKNOWN:
      return "UNKNOWN";
    case VariableDeclaration_Kind.LET:
      return "LET";
    case VariableDeclaration_Kind.CONST:
      return "CONST";
    case VariableDeclaration_Kind.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface VariableDeclarator {
  id: LVal | undefined;
  init?: Expression | undefined;
}

export interface WhileStatement {
  test: Expression | undefined;
  body: Statement | undefined;
}

export interface Expression {
  literal: Literal | undefined;
  array: ArrayExpression | undefined;
  assignment: AssignmentExpression | undefined;
  binary: BinaryExpression | undefined;
  call: CallExpression | undefined;
  conditional: ConditionalExpression | undefined;
  function: FunctionExpression | undefined;
  identifier: Identifier | undefined;
  logical: LogicalExpression | undefined;
  member: MemberExpression | undefined;
  object: ObjectExpression | undefined;
  unary: UnaryExpression | undefined;
  update: UpdateExpression | undefined;
  arrowFunction: ArrowFunctionExpression | undefined;
}

export interface MaybeSpreadExpression {
  literal: Literal | undefined;
  array: ArrayExpression | undefined;
  assignment: AssignmentExpression | undefined;
  binary: BinaryExpression | undefined;
  call: CallExpression | undefined;
  conditional: ConditionalExpression | undefined;
  function: FunctionExpression | undefined;
  identifier: Identifier | undefined;
  logical: LogicalExpression | undefined;
  member: MemberExpression | undefined;
  object: ObjectExpression | undefined;
  unary: UnaryExpression | undefined;
  update: UpdateExpression | undefined;
  arrowFunction: ArrowFunctionExpression | undefined;
  isSpread?: boolean | undefined;
}

export interface ArrayExpression {
  elements: MaybeSpreadExpression[];
}

export interface AssignmentExpression {
  operator: AssignmentExpression_Operator;
  left: LVal | undefined;
  right: Expression | undefined;
}

export enum AssignmentExpression_Operator {
  UNKNOWN = 0,
  /** ASSIGN - = */
  ASSIGN = 1,
  /** MUL - = */
  MUL = 2,
  /** DIV - /= */
  DIV = 3,
  /** MOD - %= */
  MOD = 4,
  /** ADD - += */
  ADD = 5,
  /** SUB - -= */
  SUB = 6,
  /** LSHIFT - <<= */
  LSHIFT = 7,
  /** RSHIFT - >>= */
  RSHIFT = 8,
  /** ZRSHIFT - >>>= */
  ZRSHIFT = 9,
  /** BITAND - &= */
  BITAND = 10,
  /** BITXOR - ^= */
  BITXOR = 11,
  /** BITOR - |= */
  BITOR = 12,
  /** POW - *= */
  POW = 13,
  UNRECOGNIZED = -1,
}

export function assignmentExpression_OperatorFromJSON(
  object: any
): AssignmentExpression_Operator {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return AssignmentExpression_Operator.UNKNOWN;
    case 1:
    case "ASSIGN":
      return AssignmentExpression_Operator.ASSIGN;
    case 2:
    case "MUL":
      return AssignmentExpression_Operator.MUL;
    case 3:
    case "DIV":
      return AssignmentExpression_Operator.DIV;
    case 4:
    case "MOD":
      return AssignmentExpression_Operator.MOD;
    case 5:
    case "ADD":
      return AssignmentExpression_Operator.ADD;
    case 6:
    case "SUB":
      return AssignmentExpression_Operator.SUB;
    case 7:
    case "LSHIFT":
      return AssignmentExpression_Operator.LSHIFT;
    case 8:
    case "RSHIFT":
      return AssignmentExpression_Operator.RSHIFT;
    case 9:
    case "ZRSHIFT":
      return AssignmentExpression_Operator.ZRSHIFT;
    case 10:
    case "BITAND":
      return AssignmentExpression_Operator.BITAND;
    case 11:
    case "BITXOR":
      return AssignmentExpression_Operator.BITXOR;
    case 12:
    case "BITOR":
      return AssignmentExpression_Operator.BITOR;
    case 13:
    case "POW":
      return AssignmentExpression_Operator.POW;
    case -1:
    case "UNRECOGNIZED":
    default:
      return AssignmentExpression_Operator.UNRECOGNIZED;
  }
}

export function assignmentExpression_OperatorToJSON(
  object: AssignmentExpression_Operator
): string {
  switch (object) {
    case AssignmentExpression_Operator.UNKNOWN:
      return "UNKNOWN";
    case AssignmentExpression_Operator.ASSIGN:
      return "ASSIGN";
    case AssignmentExpression_Operator.MUL:
      return "MUL";
    case AssignmentExpression_Operator.DIV:
      return "DIV";
    case AssignmentExpression_Operator.MOD:
      return "MOD";
    case AssignmentExpression_Operator.ADD:
      return "ADD";
    case AssignmentExpression_Operator.SUB:
      return "SUB";
    case AssignmentExpression_Operator.LSHIFT:
      return "LSHIFT";
    case AssignmentExpression_Operator.RSHIFT:
      return "RSHIFT";
    case AssignmentExpression_Operator.ZRSHIFT:
      return "ZRSHIFT";
    case AssignmentExpression_Operator.BITAND:
      return "BITAND";
    case AssignmentExpression_Operator.BITXOR:
      return "BITXOR";
    case AssignmentExpression_Operator.BITOR:
      return "BITOR";
    case AssignmentExpression_Operator.POW:
      return "POW";
    case AssignmentExpression_Operator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface BinaryExpression {
  operator: BinaryExpression_Operator;
  left: Expression | undefined;
  right: Expression | undefined;
}

export enum BinaryExpression_Operator {
  UNKNOWN = 0,
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

export function binaryExpression_OperatorFromJSON(
  object: any
): BinaryExpression_Operator {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return BinaryExpression_Operator.UNKNOWN;
    case 1:
    case "ADD":
      return BinaryExpression_Operator.ADD;
    case 2:
    case "SUB":
      return BinaryExpression_Operator.SUB;
    case 3:
    case "DIV":
      return BinaryExpression_Operator.DIV;
    case 4:
    case "MOD":
      return BinaryExpression_Operator.MOD;
    case 5:
    case "MUL":
      return BinaryExpression_Operator.MUL;
    case 6:
    case "POW":
      return BinaryExpression_Operator.POW;
    case 7:
    case "BITAND":
      return BinaryExpression_Operator.BITAND;
    case 8:
    case "BITOR":
      return BinaryExpression_Operator.BITOR;
    case 9:
    case "RSHIFT":
      return BinaryExpression_Operator.RSHIFT;
    case 10:
    case "URSHIFT":
      return BinaryExpression_Operator.URSHIFT;
    case 11:
    case "LSHIFT":
      return BinaryExpression_Operator.LSHIFT;
    case 12:
    case "BITXOR":
      return BinaryExpression_Operator.BITXOR;
    case 13:
    case "EQ":
      return BinaryExpression_Operator.EQ;
    case 14:
    case "NEQ":
      return BinaryExpression_Operator.NEQ;
    case 15:
    case "GT":
      return BinaryExpression_Operator.GT;
    case 16:
    case "LT":
      return BinaryExpression_Operator.LT;
    case 17:
    case "GTE":
      return BinaryExpression_Operator.GTE;
    case 18:
    case "LTE":
      return BinaryExpression_Operator.LTE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return BinaryExpression_Operator.UNRECOGNIZED;
  }
}

export function binaryExpression_OperatorToJSON(
  object: BinaryExpression_Operator
): string {
  switch (object) {
    case BinaryExpression_Operator.UNKNOWN:
      return "UNKNOWN";
    case BinaryExpression_Operator.ADD:
      return "ADD";
    case BinaryExpression_Operator.SUB:
      return "SUB";
    case BinaryExpression_Operator.DIV:
      return "DIV";
    case BinaryExpression_Operator.MOD:
      return "MOD";
    case BinaryExpression_Operator.MUL:
      return "MUL";
    case BinaryExpression_Operator.POW:
      return "POW";
    case BinaryExpression_Operator.BITAND:
      return "BITAND";
    case BinaryExpression_Operator.BITOR:
      return "BITOR";
    case BinaryExpression_Operator.RSHIFT:
      return "RSHIFT";
    case BinaryExpression_Operator.URSHIFT:
      return "URSHIFT";
    case BinaryExpression_Operator.LSHIFT:
      return "LSHIFT";
    case BinaryExpression_Operator.BITXOR:
      return "BITXOR";
    case BinaryExpression_Operator.EQ:
      return "EQ";
    case BinaryExpression_Operator.NEQ:
      return "NEQ";
    case BinaryExpression_Operator.GT:
      return "GT";
    case BinaryExpression_Operator.LT:
      return "LT";
    case BinaryExpression_Operator.GTE:
      return "GTE";
    case BinaryExpression_Operator.LTE:
      return "LTE";
    case BinaryExpression_Operator.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface CallExpression {
  callee: Expression | undefined;
  arguments: MaybeSpreadExpression[];
  option?: boolean | undefined;
}

export interface ConditionalExpression {
  test: Expression | undefined;
  consequent: Expression | undefined;
  alternate: Expression | undefined;
}

export interface FunctionExpression {
  id?: Identifier | undefined;
  params: PatternLike[];
  body: Statement[];
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

export interface MemberExpression {
  object: Expression | undefined;
  propertyExpression: Expression | undefined;
  propertyIdentifier: Identifier | undefined;
  computed: boolean;
  option?: boolean | undefined;
}

export interface ObjectExpression {
  properties: ObjectElement[];
}

export interface ObjectElement {
  method: ObjectMethod | undefined;
  property: ObjectProperty | undefined;
  spread: Expression | undefined;
}

export interface ObjectMethod {
  kind: ObjectMethod_Kind;
  keyExpression: Expression | undefined;
  keyIdentifier: Identifier | undefined;
  keyStringLiteral: string | undefined;
  keyNumericLiteral: number | undefined;
  params: PatternLike[];
  body: Statement[];
  computed: boolean;
}

export enum ObjectMethod_Kind {
  UNKNOWN = 0,
  METHOD = 1,
  GET = 2,
  SET = 3,
  UNRECOGNIZED = -1,
}

export function objectMethod_KindFromJSON(object: any): ObjectMethod_Kind {
  switch (object) {
    case 0:
    case "UNKNOWN":
      return ObjectMethod_Kind.UNKNOWN;
    case 1:
    case "METHOD":
      return ObjectMethod_Kind.METHOD;
    case 2:
    case "GET":
      return ObjectMethod_Kind.GET;
    case 3:
    case "SET":
      return ObjectMethod_Kind.SET;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ObjectMethod_Kind.UNRECOGNIZED;
  }
}

export function objectMethod_KindToJSON(object: ObjectMethod_Kind): string {
  switch (object) {
    case ObjectMethod_Kind.UNKNOWN:
      return "UNKNOWN";
    case ObjectMethod_Kind.METHOD:
      return "METHOD";
    case ObjectMethod_Kind.GET:
      return "GET";
    case ObjectMethod_Kind.SET:
      return "SET";
    case ObjectMethod_Kind.UNRECOGNIZED:
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

export interface ArrowFunctionExpression {
  params: PatternLike[];
  statement: Statement[];
  expression?: Expression | undefined;
}

export interface Identifier {
  name: string;
  option?: boolean | undefined;
}

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

function createBaseLiteral(): Literal {
  return {
    stringLiteral: undefined,
    numberLiteral: undefined,
    booleanLiteral: undefined,
    nullLiteral: undefined,
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
    if (message.bigintLiteral !== undefined) {
      writer.uint32(42).string(message.bigintLiteral);
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
    message.bigintLiteral = object.bigintLiteral ?? undefined;
    return message;
  },
};

function createBaseStatement(): Statement {
  return {
    blockStatement: undefined,
    breakStatement: undefined,
    continueStatement: undefined,
    expressionStatement: undefined,
    forStatement: undefined,
    forOfStatement: undefined,
    functionDeclaration: undefined,
    ifStatement: undefined,
    returnStatement: undefined,
    switchStatement: undefined,
    throwStatement: undefined,
    tryStatement: undefined,
    variableDeclaration: undefined,
    whileStatement: undefined,
  };
}

export const Statement = {
  encode(
    message: Statement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.blockStatement !== undefined) {
      BlockStatement.encode(
        message.blockStatement,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.breakStatement !== undefined) {
      BreakStatement.encode(
        message.breakStatement,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.continueStatement !== undefined) {
      ContinueStatement.encode(
        message.continueStatement,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.expressionStatement !== undefined) {
      Expression.encode(
        message.expressionStatement,
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
    if (message.functionDeclaration !== undefined) {
      FunctionDeclaration.encode(
        message.functionDeclaration,
        writer.uint32(58).fork()
      ).ldelim();
    }
    if (message.ifStatement !== undefined) {
      IfStatement.encode(
        message.ifStatement,
        writer.uint32(66).fork()
      ).ldelim();
    }
    if (message.returnStatement !== undefined) {
      ReturnStatement.encode(
        message.returnStatement,
        writer.uint32(74).fork()
      ).ldelim();
    }
    if (message.switchStatement !== undefined) {
      SwitchStatement.encode(
        message.switchStatement,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.throwStatement !== undefined) {
      ThrowStatement.encode(
        message.throwStatement,
        writer.uint32(90).fork()
      ).ldelim();
    }
    if (message.tryStatement !== undefined) {
      TryStatement.encode(
        message.tryStatement,
        writer.uint32(98).fork()
      ).ldelim();
    }
    if (message.variableDeclaration !== undefined) {
      VariableDeclaration.encode(
        message.variableDeclaration,
        writer.uint32(106).fork()
      ).ldelim();
    }
    if (message.whileStatement !== undefined) {
      WhileStatement.encode(
        message.whileStatement,
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
          message.blockStatement = BlockStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.breakStatement = BreakStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.continueStatement = ContinueStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.expressionStatement = Expression.decode(
            reader,
            reader.uint32()
          );
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
          message.functionDeclaration = FunctionDeclaration.decode(
            reader,
            reader.uint32()
          );
          break;
        case 8:
          message.ifStatement = IfStatement.decode(reader, reader.uint32());
          break;
        case 9:
          message.returnStatement = ReturnStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 10:
          message.switchStatement = SwitchStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 11:
          message.throwStatement = ThrowStatement.decode(
            reader,
            reader.uint32()
          );
          break;
        case 12:
          message.tryStatement = TryStatement.decode(reader, reader.uint32());
          break;
        case 13:
          message.variableDeclaration = VariableDeclaration.decode(
            reader,
            reader.uint32()
          );
          break;
        case 14:
          message.whileStatement = WhileStatement.decode(
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
      blockStatement: isSet(object.blockStatement)
        ? BlockStatement.fromJSON(object.blockStatement)
        : undefined,
      breakStatement: isSet(object.breakStatement)
        ? BreakStatement.fromJSON(object.breakStatement)
        : undefined,
      continueStatement: isSet(object.continueStatement)
        ? ContinueStatement.fromJSON(object.continueStatement)
        : undefined,
      expressionStatement: isSet(object.expressionStatement)
        ? Expression.fromJSON(object.expressionStatement)
        : undefined,
      forStatement: isSet(object.forStatement)
        ? ForStatement.fromJSON(object.forStatement)
        : undefined,
      forOfStatement: isSet(object.forOfStatement)
        ? ForOfStatement.fromJSON(object.forOfStatement)
        : undefined,
      functionDeclaration: isSet(object.functionDeclaration)
        ? FunctionDeclaration.fromJSON(object.functionDeclaration)
        : undefined,
      ifStatement: isSet(object.ifStatement)
        ? IfStatement.fromJSON(object.ifStatement)
        : undefined,
      returnStatement: isSet(object.returnStatement)
        ? ReturnStatement.fromJSON(object.returnStatement)
        : undefined,
      switchStatement: isSet(object.switchStatement)
        ? SwitchStatement.fromJSON(object.switchStatement)
        : undefined,
      throwStatement: isSet(object.throwStatement)
        ? ThrowStatement.fromJSON(object.throwStatement)
        : undefined,
      tryStatement: isSet(object.tryStatement)
        ? TryStatement.fromJSON(object.tryStatement)
        : undefined,
      variableDeclaration: isSet(object.variableDeclaration)
        ? VariableDeclaration.fromJSON(object.variableDeclaration)
        : undefined,
      whileStatement: isSet(object.whileStatement)
        ? WhileStatement.fromJSON(object.whileStatement)
        : undefined,
    };
  },

  toJSON(message: Statement): unknown {
    const obj: any = {};
    message.blockStatement !== undefined &&
      (obj.blockStatement = message.blockStatement
        ? BlockStatement.toJSON(message.blockStatement)
        : undefined);
    message.breakStatement !== undefined &&
      (obj.breakStatement = message.breakStatement
        ? BreakStatement.toJSON(message.breakStatement)
        : undefined);
    message.continueStatement !== undefined &&
      (obj.continueStatement = message.continueStatement
        ? ContinueStatement.toJSON(message.continueStatement)
        : undefined);
    message.expressionStatement !== undefined &&
      (obj.expressionStatement = message.expressionStatement
        ? Expression.toJSON(message.expressionStatement)
        : undefined);
    message.forStatement !== undefined &&
      (obj.forStatement = message.forStatement
        ? ForStatement.toJSON(message.forStatement)
        : undefined);
    message.forOfStatement !== undefined &&
      (obj.forOfStatement = message.forOfStatement
        ? ForOfStatement.toJSON(message.forOfStatement)
        : undefined);
    message.functionDeclaration !== undefined &&
      (obj.functionDeclaration = message.functionDeclaration
        ? FunctionDeclaration.toJSON(message.functionDeclaration)
        : undefined);
    message.ifStatement !== undefined &&
      (obj.ifStatement = message.ifStatement
        ? IfStatement.toJSON(message.ifStatement)
        : undefined);
    message.returnStatement !== undefined &&
      (obj.returnStatement = message.returnStatement
        ? ReturnStatement.toJSON(message.returnStatement)
        : undefined);
    message.switchStatement !== undefined &&
      (obj.switchStatement = message.switchStatement
        ? SwitchStatement.toJSON(message.switchStatement)
        : undefined);
    message.throwStatement !== undefined &&
      (obj.throwStatement = message.throwStatement
        ? ThrowStatement.toJSON(message.throwStatement)
        : undefined);
    message.tryStatement !== undefined &&
      (obj.tryStatement = message.tryStatement
        ? TryStatement.toJSON(message.tryStatement)
        : undefined);
    message.variableDeclaration !== undefined &&
      (obj.variableDeclaration = message.variableDeclaration
        ? VariableDeclaration.toJSON(message.variableDeclaration)
        : undefined);
    message.whileStatement !== undefined &&
      (obj.whileStatement = message.whileStatement
        ? WhileStatement.toJSON(message.whileStatement)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Statement>, I>>(
    object: I
  ): Statement {
    const message = createBaseStatement();
    message.blockStatement =
      object.blockStatement !== undefined && object.blockStatement !== null
        ? BlockStatement.fromPartial(object.blockStatement)
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
    message.expressionStatement =
      object.expressionStatement !== undefined &&
      object.expressionStatement !== null
        ? Expression.fromPartial(object.expressionStatement)
        : undefined;
    message.forStatement =
      object.forStatement !== undefined && object.forStatement !== null
        ? ForStatement.fromPartial(object.forStatement)
        : undefined;
    message.forOfStatement =
      object.forOfStatement !== undefined && object.forOfStatement !== null
        ? ForOfStatement.fromPartial(object.forOfStatement)
        : undefined;
    message.functionDeclaration =
      object.functionDeclaration !== undefined &&
      object.functionDeclaration !== null
        ? FunctionDeclaration.fromPartial(object.functionDeclaration)
        : undefined;
    message.ifStatement =
      object.ifStatement !== undefined && object.ifStatement !== null
        ? IfStatement.fromPartial(object.ifStatement)
        : undefined;
    message.returnStatement =
      object.returnStatement !== undefined && object.returnStatement !== null
        ? ReturnStatement.fromPartial(object.returnStatement)
        : undefined;
    message.switchStatement =
      object.switchStatement !== undefined && object.switchStatement !== null
        ? SwitchStatement.fromPartial(object.switchStatement)
        : undefined;
    message.throwStatement =
      object.throwStatement !== undefined && object.throwStatement !== null
        ? ThrowStatement.fromPartial(object.throwStatement)
        : undefined;
    message.tryStatement =
      object.tryStatement !== undefined && object.tryStatement !== null
        ? TryStatement.fromPartial(object.tryStatement)
        : undefined;
    message.variableDeclaration =
      object.variableDeclaration !== undefined &&
      object.variableDeclaration !== null
        ? VariableDeclaration.fromPartial(object.variableDeclaration)
        : undefined;
    message.whileStatement =
      object.whileStatement !== undefined && object.whileStatement !== null
        ? WhileStatement.fromPartial(object.whileStatement)
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

function createBaseForStatement(): ForStatement {
  return {
    initDeclaration: undefined,
    initExpression: undefined,
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
    if (message.initDeclaration !== undefined) {
      VariableDeclaration.encode(
        message.initDeclaration,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.initExpression !== undefined) {
      Expression.encode(
        message.initExpression,
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
      Statement.encode(message.body, writer.uint32(42).fork()).ldelim();
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
          message.initDeclaration = VariableDeclaration.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.initExpression = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 4:
          message.update = Expression.decode(reader, reader.uint32());
          break;
        case 5:
          message.body = Statement.decode(reader, reader.uint32());
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
      initDeclaration: isSet(object.initDeclaration)
        ? VariableDeclaration.fromJSON(object.initDeclaration)
        : undefined,
      initExpression: isSet(object.initExpression)
        ? Expression.fromJSON(object.initExpression)
        : undefined,
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      update: isSet(object.update)
        ? Expression.fromJSON(object.update)
        : undefined,
      body: isSet(object.body) ? Statement.fromJSON(object.body) : undefined,
    };
  },

  toJSON(message: ForStatement): unknown {
    const obj: any = {};
    message.initDeclaration !== undefined &&
      (obj.initDeclaration = message.initDeclaration
        ? VariableDeclaration.toJSON(message.initDeclaration)
        : undefined);
    message.initExpression !== undefined &&
      (obj.initExpression = message.initExpression
        ? Expression.toJSON(message.initExpression)
        : undefined);
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    message.update !== undefined &&
      (obj.update = message.update
        ? Expression.toJSON(message.update)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body ? Statement.toJSON(message.body) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ForStatement>, I>>(
    object: I
  ): ForStatement {
    const message = createBaseForStatement();
    message.initDeclaration =
      object.initDeclaration !== undefined && object.initDeclaration !== null
        ? VariableDeclaration.fromPartial(object.initDeclaration)
        : undefined;
    message.initExpression =
      object.initExpression !== undefined && object.initExpression !== null
        ? Expression.fromPartial(object.initExpression)
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
        ? Statement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseForOfStatement(): ForOfStatement {
  return {
    leftDeclaration: undefined,
    leftLval: undefined,
    right: undefined,
    body: undefined,
  };
}

export const ForOfStatement = {
  encode(
    message: ForOfStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.leftDeclaration !== undefined) {
      VariableDeclaration.encode(
        message.leftDeclaration,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.leftLval !== undefined) {
      LVal.encode(message.leftLval, writer.uint32(18).fork()).ldelim();
    }
    if (message.right !== undefined) {
      Expression.encode(message.right, writer.uint32(26).fork()).ldelim();
    }
    if (message.body !== undefined) {
      Statement.encode(message.body, writer.uint32(34).fork()).ldelim();
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
          message.leftDeclaration = VariableDeclaration.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.leftLval = LVal.decode(reader, reader.uint32());
          break;
        case 3:
          message.right = Expression.decode(reader, reader.uint32());
          break;
        case 4:
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
      leftDeclaration: isSet(object.leftDeclaration)
        ? VariableDeclaration.fromJSON(object.leftDeclaration)
        : undefined,
      leftLval: isSet(object.leftLval)
        ? LVal.fromJSON(object.leftLval)
        : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
      body: isSet(object.body) ? Statement.fromJSON(object.body) : undefined,
    };
  },

  toJSON(message: ForOfStatement): unknown {
    const obj: any = {};
    message.leftDeclaration !== undefined &&
      (obj.leftDeclaration = message.leftDeclaration
        ? VariableDeclaration.toJSON(message.leftDeclaration)
        : undefined);
    message.leftLval !== undefined &&
      (obj.leftLval = message.leftLval
        ? LVal.toJSON(message.leftLval)
        : undefined);
    message.right !== undefined &&
      (obj.right = message.right
        ? Expression.toJSON(message.right)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body ? Statement.toJSON(message.body) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ForOfStatement>, I>>(
    object: I
  ): ForOfStatement {
    const message = createBaseForOfStatement();
    message.leftDeclaration =
      object.leftDeclaration !== undefined && object.leftDeclaration !== null
        ? VariableDeclaration.fromPartial(object.leftDeclaration)
        : undefined;
    message.leftLval =
      object.leftLval !== undefined && object.leftLval !== null
        ? LVal.fromPartial(object.leftLval)
        : undefined;
    message.right =
      object.right !== undefined && object.right !== null
        ? Expression.fromPartial(object.right)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? Statement.fromPartial(object.body)
        : undefined;
    return message;
  },
};

function createBaseAssignmentPattern(): AssignmentPattern {
  return {
    leftIdentifier: undefined,
    leftObject: undefined,
    leftArray: undefined,
    leftMember: undefined,
    right: undefined,
  };
}

export const AssignmentPattern = {
  encode(
    message: AssignmentPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.leftIdentifier !== undefined) {
      Identifier.encode(
        message.leftIdentifier,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.leftObject !== undefined) {
      ObjectPattern.encode(
        message.leftObject,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.leftArray !== undefined) {
      ArrayPattern.encode(message.leftArray, writer.uint32(26).fork()).ldelim();
    }
    if (message.leftMember !== undefined) {
      MemberExpression.encode(
        message.leftMember,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.right !== undefined) {
      Expression.encode(message.right, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AssignmentPattern {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAssignmentPattern();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.leftIdentifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.leftObject = ObjectPattern.decode(reader, reader.uint32());
          break;
        case 3:
          message.leftArray = ArrayPattern.decode(reader, reader.uint32());
          break;
        case 4:
          message.leftMember = MemberExpression.decode(reader, reader.uint32());
          break;
        case 5:
          message.right = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AssignmentPattern {
    return {
      leftIdentifier: isSet(object.leftIdentifier)
        ? Identifier.fromJSON(object.leftIdentifier)
        : undefined,
      leftObject: isSet(object.leftObject)
        ? ObjectPattern.fromJSON(object.leftObject)
        : undefined,
      leftArray: isSet(object.leftArray)
        ? ArrayPattern.fromJSON(object.leftArray)
        : undefined,
      leftMember: isSet(object.leftMember)
        ? MemberExpression.fromJSON(object.leftMember)
        : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
    };
  },

  toJSON(message: AssignmentPattern): unknown {
    const obj: any = {};
    message.leftIdentifier !== undefined &&
      (obj.leftIdentifier = message.leftIdentifier
        ? Identifier.toJSON(message.leftIdentifier)
        : undefined);
    message.leftObject !== undefined &&
      (obj.leftObject = message.leftObject
        ? ObjectPattern.toJSON(message.leftObject)
        : undefined);
    message.leftArray !== undefined &&
      (obj.leftArray = message.leftArray
        ? ArrayPattern.toJSON(message.leftArray)
        : undefined);
    message.leftMember !== undefined &&
      (obj.leftMember = message.leftMember
        ? MemberExpression.toJSON(message.leftMember)
        : undefined);
    message.right !== undefined &&
      (obj.right = message.right
        ? Expression.toJSON(message.right)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AssignmentPattern>, I>>(
    object: I
  ): AssignmentPattern {
    const message = createBaseAssignmentPattern();
    message.leftIdentifier =
      object.leftIdentifier !== undefined && object.leftIdentifier !== null
        ? Identifier.fromPartial(object.leftIdentifier)
        : undefined;
    message.leftObject =
      object.leftObject !== undefined && object.leftObject !== null
        ? ObjectPattern.fromPartial(object.leftObject)
        : undefined;
    message.leftArray =
      object.leftArray !== undefined && object.leftArray !== null
        ? ArrayPattern.fromPartial(object.leftArray)
        : undefined;
    message.leftMember =
      object.leftMember !== undefined && object.leftMember !== null
        ? MemberExpression.fromPartial(object.leftMember)
        : undefined;
    message.right =
      object.right !== undefined && object.right !== null
        ? Expression.fromPartial(object.right)
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
      PatternLike.encode(v!, writer.uint32(10).fork()).ldelim();
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
          message.elements.push(PatternLike.decode(reader, reader.uint32()));
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
        ? object.elements.map((e: any) => PatternLike.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ArrayPattern): unknown {
    const obj: any = {};
    if (message.elements) {
      obj.elements = message.elements.map((e) =>
        e ? PatternLike.toJSON(e) : undefined
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
      object.elements?.map((e) => PatternLike.fromPartial(e)) || [];
    return message;
  },
};

function createBaseObjectPattern(): ObjectPattern {
  return { properties: [] };
}

export const ObjectPattern = {
  encode(
    message: ObjectPattern,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.properties) {
      ObjectProperty.encode(v!, writer.uint32(10).fork()).ldelim();
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
          message.properties.push(
            ObjectProperty.decode(reader, reader.uint32())
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
      properties: Array.isArray(object?.properties)
        ? object.properties.map((e: any) => ObjectProperty.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ObjectPattern): unknown {
    const obj: any = {};
    if (message.properties) {
      obj.properties = message.properties.map((e) =>
        e ? ObjectProperty.toJSON(e) : undefined
      );
    } else {
      obj.properties = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectPattern>, I>>(
    object: I
  ): ObjectPattern {
    const message = createBaseObjectPattern();
    message.properties =
      object.properties?.map((e) => ObjectProperty.fromPartial(e)) || [];
    return message;
  },
};

function createBaseObjectProperty(): ObjectProperty {
  return {
    keyIdentifier: undefined,
    keyStringLiteral: undefined,
    keyNumericLiteral: undefined,
    keyBigintLiteral: undefined,
    keyExpression: undefined,
    valueExpression: undefined,
    valuePattern: undefined,
    computed: false,
    shorthand: false,
    rest: undefined,
  };
}

export const ObjectProperty = {
  encode(
    message: ObjectProperty,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.keyIdentifier !== undefined) {
      Identifier.encode(
        message.keyIdentifier,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.keyStringLiteral !== undefined) {
      writer.uint32(18).string(message.keyStringLiteral);
    }
    if (message.keyNumericLiteral !== undefined) {
      writer.uint32(24).uint64(message.keyNumericLiteral);
    }
    if (message.keyBigintLiteral !== undefined) {
      writer.uint32(34).string(message.keyBigintLiteral);
    }
    if (message.keyExpression !== undefined) {
      Expression.encode(
        message.keyExpression,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.valueExpression !== undefined) {
      Expression.encode(
        message.valueExpression,
        writer.uint32(50).fork()
      ).ldelim();
    }
    if (message.valuePattern !== undefined) {
      PatternLike.encode(
        message.valuePattern,
        writer.uint32(58).fork()
      ).ldelim();
    }
    if (message.computed === true) {
      writer.uint32(64).bool(message.computed);
    }
    if (message.shorthand === true) {
      writer.uint32(72).bool(message.shorthand);
    }
    if (message.rest !== undefined) {
      LVal.encode(message.rest, writer.uint32(82).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ObjectProperty {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectProperty();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.keyIdentifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.keyStringLiteral = reader.string();
          break;
        case 3:
          message.keyNumericLiteral = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.keyBigintLiteral = reader.string();
          break;
        case 5:
          message.keyExpression = Expression.decode(reader, reader.uint32());
          break;
        case 6:
          message.valueExpression = Expression.decode(reader, reader.uint32());
          break;
        case 7:
          message.valuePattern = PatternLike.decode(reader, reader.uint32());
          break;
        case 8:
          message.computed = reader.bool();
          break;
        case 9:
          message.shorthand = reader.bool();
          break;
        case 10:
          message.rest = LVal.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectProperty {
    return {
      keyIdentifier: isSet(object.keyIdentifier)
        ? Identifier.fromJSON(object.keyIdentifier)
        : undefined,
      keyStringLiteral: isSet(object.keyStringLiteral)
        ? String(object.keyStringLiteral)
        : undefined,
      keyNumericLiteral: isSet(object.keyNumericLiteral)
        ? Number(object.keyNumericLiteral)
        : undefined,
      keyBigintLiteral: isSet(object.keyBigintLiteral)
        ? String(object.keyBigintLiteral)
        : undefined,
      keyExpression: isSet(object.keyExpression)
        ? Expression.fromJSON(object.keyExpression)
        : undefined,
      valueExpression: isSet(object.valueExpression)
        ? Expression.fromJSON(object.valueExpression)
        : undefined,
      valuePattern: isSet(object.valuePattern)
        ? PatternLike.fromJSON(object.valuePattern)
        : undefined,
      computed: isSet(object.computed) ? Boolean(object.computed) : false,
      shorthand: isSet(object.shorthand) ? Boolean(object.shorthand) : false,
      rest: isSet(object.rest) ? LVal.fromJSON(object.rest) : undefined,
    };
  },

  toJSON(message: ObjectProperty): unknown {
    const obj: any = {};
    message.keyIdentifier !== undefined &&
      (obj.keyIdentifier = message.keyIdentifier
        ? Identifier.toJSON(message.keyIdentifier)
        : undefined);
    message.keyStringLiteral !== undefined &&
      (obj.keyStringLiteral = message.keyStringLiteral);
    message.keyNumericLiteral !== undefined &&
      (obj.keyNumericLiteral = Math.round(message.keyNumericLiteral));
    message.keyBigintLiteral !== undefined &&
      (obj.keyBigintLiteral = message.keyBigintLiteral);
    message.keyExpression !== undefined &&
      (obj.keyExpression = message.keyExpression
        ? Expression.toJSON(message.keyExpression)
        : undefined);
    message.valueExpression !== undefined &&
      (obj.valueExpression = message.valueExpression
        ? Expression.toJSON(message.valueExpression)
        : undefined);
    message.valuePattern !== undefined &&
      (obj.valuePattern = message.valuePattern
        ? PatternLike.toJSON(message.valuePattern)
        : undefined);
    message.computed !== undefined && (obj.computed = message.computed);
    message.shorthand !== undefined && (obj.shorthand = message.shorthand);
    message.rest !== undefined &&
      (obj.rest = message.rest ? LVal.toJSON(message.rest) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectProperty>, I>>(
    object: I
  ): ObjectProperty {
    const message = createBaseObjectProperty();
    message.keyIdentifier =
      object.keyIdentifier !== undefined && object.keyIdentifier !== null
        ? Identifier.fromPartial(object.keyIdentifier)
        : undefined;
    message.keyStringLiteral = object.keyStringLiteral ?? undefined;
    message.keyNumericLiteral = object.keyNumericLiteral ?? undefined;
    message.keyBigintLiteral = object.keyBigintLiteral ?? undefined;
    message.keyExpression =
      object.keyExpression !== undefined && object.keyExpression !== null
        ? Expression.fromPartial(object.keyExpression)
        : undefined;
    message.valueExpression =
      object.valueExpression !== undefined && object.valueExpression !== null
        ? Expression.fromPartial(object.valueExpression)
        : undefined;
    message.valuePattern =
      object.valuePattern !== undefined && object.valuePattern !== null
        ? PatternLike.fromPartial(object.valuePattern)
        : undefined;
    message.computed = object.computed ?? false;
    message.shorthand = object.shorthand ?? false;
    message.rest =
      object.rest !== undefined && object.rest !== null
        ? LVal.fromPartial(object.rest)
        : undefined;
    return message;
  },
};

function createBasePatternLike(): PatternLike {
  return {
    identifier: undefined,
    assignment: undefined,
    array: undefined,
    object: undefined,
    restElement: undefined,
  };
}

export const PatternLike = {
  encode(
    message: PatternLike,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    if (message.assignment !== undefined) {
      AssignmentPattern.encode(
        message.assignment,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.array !== undefined) {
      ArrayPattern.encode(message.array, writer.uint32(26).fork()).ldelim();
    }
    if (message.object !== undefined) {
      ObjectPattern.encode(message.object, writer.uint32(34).fork()).ldelim();
    }
    if (message.restElement !== undefined) {
      LVal.encode(message.restElement, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PatternLike {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePatternLike();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.assignment = AssignmentPattern.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.array = ArrayPattern.decode(reader, reader.uint32());
          break;
        case 4:
          message.object = ObjectPattern.decode(reader, reader.uint32());
          break;
        case 5:
          message.restElement = LVal.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PatternLike {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      assignment: isSet(object.assignment)
        ? AssignmentPattern.fromJSON(object.assignment)
        : undefined,
      array: isSet(object.array)
        ? ArrayPattern.fromJSON(object.array)
        : undefined,
      object: isSet(object.object)
        ? ObjectPattern.fromJSON(object.object)
        : undefined,
      restElement: isSet(object.restElement)
        ? LVal.fromJSON(object.restElement)
        : undefined,
    };
  },

  toJSON(message: PatternLike): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.assignment !== undefined &&
      (obj.assignment = message.assignment
        ? AssignmentPattern.toJSON(message.assignment)
        : undefined);
    message.array !== undefined &&
      (obj.array = message.array
        ? ArrayPattern.toJSON(message.array)
        : undefined);
    message.object !== undefined &&
      (obj.object = message.object
        ? ObjectPattern.toJSON(message.object)
        : undefined);
    message.restElement !== undefined &&
      (obj.restElement = message.restElement
        ? LVal.toJSON(message.restElement)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PatternLike>, I>>(
    object: I
  ): PatternLike {
    const message = createBasePatternLike();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.assignment =
      object.assignment !== undefined && object.assignment !== null
        ? AssignmentPattern.fromPartial(object.assignment)
        : undefined;
    message.array =
      object.array !== undefined && object.array !== null
        ? ArrayPattern.fromPartial(object.array)
        : undefined;
    message.object =
      object.object !== undefined && object.object !== null
        ? ObjectPattern.fromPartial(object.object)
        : undefined;
    message.restElement =
      object.restElement !== undefined && object.restElement !== null
        ? LVal.fromPartial(object.restElement)
        : undefined;
    return message;
  },
};

function createBaseLVal(): LVal {
  return {
    identifier: undefined,
    member: undefined,
    assignment: undefined,
    array: undefined,
    object: undefined,
    isRest: undefined,
  };
}

export const LVal = {
  encode(message: LVal, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(10).fork()).ldelim();
    }
    if (message.member !== undefined) {
      MemberExpression.encode(
        message.member,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.assignment !== undefined) {
      AssignmentPattern.encode(
        message.assignment,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.array !== undefined) {
      ArrayPattern.encode(message.array, writer.uint32(34).fork()).ldelim();
    }
    if (message.object !== undefined) {
      ObjectPattern.encode(message.object, writer.uint32(42).fork()).ldelim();
    }
    if (message.isRest !== undefined) {
      writer.uint32(48).bool(message.isRest);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LVal {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLVal();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.member = MemberExpression.decode(reader, reader.uint32());
          break;
        case 3:
          message.assignment = AssignmentPattern.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.array = ArrayPattern.decode(reader, reader.uint32());
          break;
        case 5:
          message.object = ObjectPattern.decode(reader, reader.uint32());
          break;
        case 6:
          message.isRest = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LVal {
    return {
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      member: isSet(object.member)
        ? MemberExpression.fromJSON(object.member)
        : undefined,
      assignment: isSet(object.assignment)
        ? AssignmentPattern.fromJSON(object.assignment)
        : undefined,
      array: isSet(object.array)
        ? ArrayPattern.fromJSON(object.array)
        : undefined,
      object: isSet(object.object)
        ? ObjectPattern.fromJSON(object.object)
        : undefined,
      isRest: isSet(object.isRest) ? Boolean(object.isRest) : undefined,
    };
  },

  toJSON(message: LVal): unknown {
    const obj: any = {};
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.member !== undefined &&
      (obj.member = message.member
        ? MemberExpression.toJSON(message.member)
        : undefined);
    message.assignment !== undefined &&
      (obj.assignment = message.assignment
        ? AssignmentPattern.toJSON(message.assignment)
        : undefined);
    message.array !== undefined &&
      (obj.array = message.array
        ? ArrayPattern.toJSON(message.array)
        : undefined);
    message.object !== undefined &&
      (obj.object = message.object
        ? ObjectPattern.toJSON(message.object)
        : undefined);
    message.isRest !== undefined && (obj.isRest = message.isRest);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LVal>, I>>(object: I): LVal {
    const message = createBaseLVal();
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.member =
      object.member !== undefined && object.member !== null
        ? MemberExpression.fromPartial(object.member)
        : undefined;
    message.assignment =
      object.assignment !== undefined && object.assignment !== null
        ? AssignmentPattern.fromPartial(object.assignment)
        : undefined;
    message.array =
      object.array !== undefined && object.array !== null
        ? ArrayPattern.fromPartial(object.array)
        : undefined;
    message.object =
      object.object !== undefined && object.object !== null
        ? ObjectPattern.fromPartial(object.object)
        : undefined;
    message.isRest = object.isRest ?? undefined;
    return message;
  },
};

function createBaseFunctionDeclaration(): FunctionDeclaration {
  return { id: undefined, params: [], rest: undefined, body: [] };
}

export const FunctionDeclaration = {
  encode(
    message: FunctionDeclaration,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== undefined) {
      Identifier.encode(message.id, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.params) {
      PatternLike.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.rest !== undefined) {
      LVal.encode(message.rest, writer.uint32(26).fork()).ldelim();
    }
    for (const v of message.body) {
      Statement.encode(v!, writer.uint32(34).fork()).ldelim();
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
          message.id = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.params.push(PatternLike.decode(reader, reader.uint32()));
          break;
        case 3:
          message.rest = LVal.decode(reader, reader.uint32());
          break;
        case 4:
          message.body.push(Statement.decode(reader, reader.uint32()));
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
      id: isSet(object.id) ? Identifier.fromJSON(object.id) : undefined,
      params: Array.isArray(object?.params)
        ? object.params.map((e: any) => PatternLike.fromJSON(e))
        : [],
      rest: isSet(object.rest) ? LVal.fromJSON(object.rest) : undefined,
      body: Array.isArray(object?.body)
        ? object.body.map((e: any) => Statement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: FunctionDeclaration): unknown {
    const obj: any = {};
    message.id !== undefined &&
      (obj.id = message.id ? Identifier.toJSON(message.id) : undefined);
    if (message.params) {
      obj.params = message.params.map((e) =>
        e ? PatternLike.toJSON(e) : undefined
      );
    } else {
      obj.params = [];
    }
    message.rest !== undefined &&
      (obj.rest = message.rest ? LVal.toJSON(message.rest) : undefined);
    if (message.body) {
      obj.body = message.body.map((e) => (e ? Statement.toJSON(e) : undefined));
    } else {
      obj.body = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FunctionDeclaration>, I>>(
    object: I
  ): FunctionDeclaration {
    const message = createBaseFunctionDeclaration();
    message.id =
      object.id !== undefined && object.id !== null
        ? Identifier.fromPartial(object.id)
        : undefined;
    message.params =
      object.params?.map((e) => PatternLike.fromPartial(e)) || [];
    message.rest =
      object.rest !== undefined && object.rest !== null
        ? LVal.fromPartial(object.rest)
        : undefined;
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
      SwitchCase.encode(v!, writer.uint32(18).fork()).ldelim();
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
          message.cases.push(SwitchCase.decode(reader, reader.uint32()));
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
        ? object.cases.map((e: any) => SwitchCase.fromJSON(e))
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
        e ? SwitchCase.toJSON(e) : undefined
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
    message.cases = object.cases?.map((e) => SwitchCase.fromPartial(e)) || [];
    return message;
  },
};

function createBaseSwitchCase(): SwitchCase {
  return { test: undefined, consequent: [] };
}

export const SwitchCase = {
  encode(
    message: SwitchCase,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.test !== undefined) {
      Expression.encode(message.test, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.consequent) {
      Statement.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SwitchCase {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSwitchCase();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.test = Expression.decode(reader, reader.uint32());
          break;
        case 2:
          message.consequent.push(Statement.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SwitchCase {
    return {
      test: isSet(object.test) ? Expression.fromJSON(object.test) : undefined,
      consequent: Array.isArray(object?.consequent)
        ? object.consequent.map((e: any) => Statement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: SwitchCase): unknown {
    const obj: any = {};
    message.test !== undefined &&
      (obj.test = message.test ? Expression.toJSON(message.test) : undefined);
    if (message.consequent) {
      obj.consequent = message.consequent.map((e) =>
        e ? Statement.toJSON(e) : undefined
      );
    } else {
      obj.consequent = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SwitchCase>, I>>(
    object: I
  ): SwitchCase {
    const message = createBaseSwitchCase();
    message.test =
      object.test !== undefined && object.test !== null
        ? Expression.fromPartial(object.test)
        : undefined;
    message.consequent =
      object.consequent?.map((e) => Statement.fromPartial(e)) || [];
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

function createBaseTryStatement(): TryStatement {
  return { block: undefined, handler: undefined, finalizer: undefined };
}

export const TryStatement = {
  encode(
    message: TryStatement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.block !== undefined) {
      Statement.encode(message.block, writer.uint32(10).fork()).ldelim();
    }
    if (message.handler !== undefined) {
      CatchClause.encode(message.handler, writer.uint32(18).fork()).ldelim();
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
          message.block = Statement.decode(reader, reader.uint32());
          break;
        case 2:
          message.handler = CatchClause.decode(reader, reader.uint32());
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
      block: isSet(object.block) ? Statement.fromJSON(object.block) : undefined,
      handler: isSet(object.handler)
        ? CatchClause.fromJSON(object.handler)
        : undefined,
      finalizer: isSet(object.finalizer)
        ? BlockStatement.fromJSON(object.finalizer)
        : undefined,
    };
  },

  toJSON(message: TryStatement): unknown {
    const obj: any = {};
    message.block !== undefined &&
      (obj.block = message.block ? Statement.toJSON(message.block) : undefined);
    message.handler !== undefined &&
      (obj.handler = message.handler
        ? CatchClause.toJSON(message.handler)
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
        ? Statement.fromPartial(object.block)
        : undefined;
    message.handler =
      object.handler !== undefined && object.handler !== null
        ? CatchClause.fromPartial(object.handler)
        : undefined;
    message.finalizer =
      object.finalizer !== undefined && object.finalizer !== null
        ? BlockStatement.fromPartial(object.finalizer)
        : undefined;
    return message;
  },
};

function createBaseCatchClause(): CatchClause {
  return {
    paramIdentifier: undefined,
    paramArray: undefined,
    paramObject: undefined,
    body: undefined,
  };
}

export const CatchClause = {
  encode(
    message: CatchClause,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.paramIdentifier !== undefined) {
      Identifier.encode(
        message.paramIdentifier,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.paramArray !== undefined) {
      ArrayPattern.encode(
        message.paramArray,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.paramObject !== undefined) {
      ObjectPattern.encode(
        message.paramObject,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.body !== undefined) {
      BlockStatement.encode(message.body, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CatchClause {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCatchClause();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.paramIdentifier = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.paramArray = ArrayPattern.decode(reader, reader.uint32());
          break;
        case 3:
          message.paramObject = ObjectPattern.decode(reader, reader.uint32());
          break;
        case 4:
          message.body = BlockStatement.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CatchClause {
    return {
      paramIdentifier: isSet(object.paramIdentifier)
        ? Identifier.fromJSON(object.paramIdentifier)
        : undefined,
      paramArray: isSet(object.paramArray)
        ? ArrayPattern.fromJSON(object.paramArray)
        : undefined,
      paramObject: isSet(object.paramObject)
        ? ObjectPattern.fromJSON(object.paramObject)
        : undefined,
      body: isSet(object.body)
        ? BlockStatement.fromJSON(object.body)
        : undefined,
    };
  },

  toJSON(message: CatchClause): unknown {
    const obj: any = {};
    message.paramIdentifier !== undefined &&
      (obj.paramIdentifier = message.paramIdentifier
        ? Identifier.toJSON(message.paramIdentifier)
        : undefined);
    message.paramArray !== undefined &&
      (obj.paramArray = message.paramArray
        ? ArrayPattern.toJSON(message.paramArray)
        : undefined);
    message.paramObject !== undefined &&
      (obj.paramObject = message.paramObject
        ? ObjectPattern.toJSON(message.paramObject)
        : undefined);
    message.body !== undefined &&
      (obj.body = message.body
        ? BlockStatement.toJSON(message.body)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CatchClause>, I>>(
    object: I
  ): CatchClause {
    const message = createBaseCatchClause();
    message.paramIdentifier =
      object.paramIdentifier !== undefined && object.paramIdentifier !== null
        ? Identifier.fromPartial(object.paramIdentifier)
        : undefined;
    message.paramArray =
      object.paramArray !== undefined && object.paramArray !== null
        ? ArrayPattern.fromPartial(object.paramArray)
        : undefined;
    message.paramObject =
      object.paramObject !== undefined && object.paramObject !== null
        ? ObjectPattern.fromPartial(object.paramObject)
        : undefined;
    message.body =
      object.body !== undefined && object.body !== null
        ? BlockStatement.fromPartial(object.body)
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
      kind: isSet(object.kind)
        ? variableDeclaration_KindFromJSON(object.kind)
        : 0,
      declarators: Array.isArray(object?.declarators)
        ? object.declarators.map((e: any) => VariableDeclarator.fromJSON(e))
        : [],
    };
  },

  toJSON(message: VariableDeclaration): unknown {
    const obj: any = {};
    message.kind !== undefined &&
      (obj.kind = variableDeclaration_KindToJSON(message.kind));
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

function createBaseVariableDeclarator(): VariableDeclarator {
  return { id: undefined, init: undefined };
}

export const VariableDeclarator = {
  encode(
    message: VariableDeclarator,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== undefined) {
      LVal.encode(message.id, writer.uint32(10).fork()).ldelim();
    }
    if (message.init !== undefined) {
      Expression.encode(message.init, writer.uint32(18).fork()).ldelim();
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
          message.id = LVal.decode(reader, reader.uint32());
          break;
        case 2:
          message.init = Expression.decode(reader, reader.uint32());
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
      id: isSet(object.id) ? LVal.fromJSON(object.id) : undefined,
      init: isSet(object.init) ? Expression.fromJSON(object.init) : undefined,
    };
  },

  toJSON(message: VariableDeclarator): unknown {
    const obj: any = {};
    message.id !== undefined &&
      (obj.id = message.id ? LVal.toJSON(message.id) : undefined);
    message.init !== undefined &&
      (obj.init = message.init ? Expression.toJSON(message.init) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<VariableDeclarator>, I>>(
    object: I
  ): VariableDeclarator {
    const message = createBaseVariableDeclarator();
    message.id =
      object.id !== undefined && object.id !== null
        ? LVal.fromPartial(object.id)
        : undefined;
    message.init =
      object.init !== undefined && object.init !== null
        ? Expression.fromPartial(object.init)
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

function createBaseExpression(): Expression {
  return {
    literal: undefined,
    array: undefined,
    assignment: undefined,
    binary: undefined,
    call: undefined,
    conditional: undefined,
    function: undefined,
    identifier: undefined,
    logical: undefined,
    member: undefined,
    object: undefined,
    unary: undefined,
    update: undefined,
    arrowFunction: undefined,
  };
}

export const Expression = {
  encode(
    message: Expression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.literal !== undefined) {
      Literal.encode(message.literal, writer.uint32(10).fork()).ldelim();
    }
    if (message.array !== undefined) {
      ArrayExpression.encode(message.array, writer.uint32(18).fork()).ldelim();
    }
    if (message.assignment !== undefined) {
      AssignmentExpression.encode(
        message.assignment,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.binary !== undefined) {
      BinaryExpression.encode(
        message.binary,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.call !== undefined) {
      CallExpression.encode(message.call, writer.uint32(42).fork()).ldelim();
    }
    if (message.conditional !== undefined) {
      ConditionalExpression.encode(
        message.conditional,
        writer.uint32(50).fork()
      ).ldelim();
    }
    if (message.function !== undefined) {
      FunctionExpression.encode(
        message.function,
        writer.uint32(58).fork()
      ).ldelim();
    }
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(66).fork()).ldelim();
    }
    if (message.logical !== undefined) {
      LogicalExpression.encode(
        message.logical,
        writer.uint32(74).fork()
      ).ldelim();
    }
    if (message.member !== undefined) {
      MemberExpression.encode(
        message.member,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.object !== undefined) {
      ObjectExpression.encode(
        message.object,
        writer.uint32(90).fork()
      ).ldelim();
    }
    if (message.unary !== undefined) {
      UnaryExpression.encode(message.unary, writer.uint32(98).fork()).ldelim();
    }
    if (message.update !== undefined) {
      UpdateExpression.encode(
        message.update,
        writer.uint32(106).fork()
      ).ldelim();
    }
    if (message.arrowFunction !== undefined) {
      ArrowFunctionExpression.encode(
        message.arrowFunction,
        writer.uint32(114).fork()
      ).ldelim();
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
          message.literal = Literal.decode(reader, reader.uint32());
          break;
        case 2:
          message.array = ArrayExpression.decode(reader, reader.uint32());
          break;
        case 3:
          message.assignment = AssignmentExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.binary = BinaryExpression.decode(reader, reader.uint32());
          break;
        case 5:
          message.call = CallExpression.decode(reader, reader.uint32());
          break;
        case 6:
          message.conditional = ConditionalExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 7:
          message.function = FunctionExpression.decode(reader, reader.uint32());
          break;
        case 8:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 9:
          message.logical = LogicalExpression.decode(reader, reader.uint32());
          break;
        case 10:
          message.member = MemberExpression.decode(reader, reader.uint32());
          break;
        case 11:
          message.object = ObjectExpression.decode(reader, reader.uint32());
          break;
        case 12:
          message.unary = UnaryExpression.decode(reader, reader.uint32());
          break;
        case 13:
          message.update = UpdateExpression.decode(reader, reader.uint32());
          break;
        case 14:
          message.arrowFunction = ArrowFunctionExpression.decode(
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

  fromJSON(object: any): Expression {
    return {
      literal: isSet(object.literal)
        ? Literal.fromJSON(object.literal)
        : undefined,
      array: isSet(object.array)
        ? ArrayExpression.fromJSON(object.array)
        : undefined,
      assignment: isSet(object.assignment)
        ? AssignmentExpression.fromJSON(object.assignment)
        : undefined,
      binary: isSet(object.binary)
        ? BinaryExpression.fromJSON(object.binary)
        : undefined,
      call: isSet(object.call)
        ? CallExpression.fromJSON(object.call)
        : undefined,
      conditional: isSet(object.conditional)
        ? ConditionalExpression.fromJSON(object.conditional)
        : undefined,
      function: isSet(object.function)
        ? FunctionExpression.fromJSON(object.function)
        : undefined,
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      logical: isSet(object.logical)
        ? LogicalExpression.fromJSON(object.logical)
        : undefined,
      member: isSet(object.member)
        ? MemberExpression.fromJSON(object.member)
        : undefined,
      object: isSet(object.object)
        ? ObjectExpression.fromJSON(object.object)
        : undefined,
      unary: isSet(object.unary)
        ? UnaryExpression.fromJSON(object.unary)
        : undefined,
      update: isSet(object.update)
        ? UpdateExpression.fromJSON(object.update)
        : undefined,
      arrowFunction: isSet(object.arrowFunction)
        ? ArrowFunctionExpression.fromJSON(object.arrowFunction)
        : undefined,
    };
  },

  toJSON(message: Expression): unknown {
    const obj: any = {};
    message.literal !== undefined &&
      (obj.literal = message.literal
        ? Literal.toJSON(message.literal)
        : undefined);
    message.array !== undefined &&
      (obj.array = message.array
        ? ArrayExpression.toJSON(message.array)
        : undefined);
    message.assignment !== undefined &&
      (obj.assignment = message.assignment
        ? AssignmentExpression.toJSON(message.assignment)
        : undefined);
    message.binary !== undefined &&
      (obj.binary = message.binary
        ? BinaryExpression.toJSON(message.binary)
        : undefined);
    message.call !== undefined &&
      (obj.call = message.call
        ? CallExpression.toJSON(message.call)
        : undefined);
    message.conditional !== undefined &&
      (obj.conditional = message.conditional
        ? ConditionalExpression.toJSON(message.conditional)
        : undefined);
    message.function !== undefined &&
      (obj.function = message.function
        ? FunctionExpression.toJSON(message.function)
        : undefined);
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.logical !== undefined &&
      (obj.logical = message.logical
        ? LogicalExpression.toJSON(message.logical)
        : undefined);
    message.member !== undefined &&
      (obj.member = message.member
        ? MemberExpression.toJSON(message.member)
        : undefined);
    message.object !== undefined &&
      (obj.object = message.object
        ? ObjectExpression.toJSON(message.object)
        : undefined);
    message.unary !== undefined &&
      (obj.unary = message.unary
        ? UnaryExpression.toJSON(message.unary)
        : undefined);
    message.update !== undefined &&
      (obj.update = message.update
        ? UpdateExpression.toJSON(message.update)
        : undefined);
    message.arrowFunction !== undefined &&
      (obj.arrowFunction = message.arrowFunction
        ? ArrowFunctionExpression.toJSON(message.arrowFunction)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Expression>, I>>(
    object: I
  ): Expression {
    const message = createBaseExpression();
    message.literal =
      object.literal !== undefined && object.literal !== null
        ? Literal.fromPartial(object.literal)
        : undefined;
    message.array =
      object.array !== undefined && object.array !== null
        ? ArrayExpression.fromPartial(object.array)
        : undefined;
    message.assignment =
      object.assignment !== undefined && object.assignment !== null
        ? AssignmentExpression.fromPartial(object.assignment)
        : undefined;
    message.binary =
      object.binary !== undefined && object.binary !== null
        ? BinaryExpression.fromPartial(object.binary)
        : undefined;
    message.call =
      object.call !== undefined && object.call !== null
        ? CallExpression.fromPartial(object.call)
        : undefined;
    message.conditional =
      object.conditional !== undefined && object.conditional !== null
        ? ConditionalExpression.fromPartial(object.conditional)
        : undefined;
    message.function =
      object.function !== undefined && object.function !== null
        ? FunctionExpression.fromPartial(object.function)
        : undefined;
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.logical =
      object.logical !== undefined && object.logical !== null
        ? LogicalExpression.fromPartial(object.logical)
        : undefined;
    message.member =
      object.member !== undefined && object.member !== null
        ? MemberExpression.fromPartial(object.member)
        : undefined;
    message.object =
      object.object !== undefined && object.object !== null
        ? ObjectExpression.fromPartial(object.object)
        : undefined;
    message.unary =
      object.unary !== undefined && object.unary !== null
        ? UnaryExpression.fromPartial(object.unary)
        : undefined;
    message.update =
      object.update !== undefined && object.update !== null
        ? UpdateExpression.fromPartial(object.update)
        : undefined;
    message.arrowFunction =
      object.arrowFunction !== undefined && object.arrowFunction !== null
        ? ArrowFunctionExpression.fromPartial(object.arrowFunction)
        : undefined;
    return message;
  },
};

function createBaseMaybeSpreadExpression(): MaybeSpreadExpression {
  return {
    literal: undefined,
    array: undefined,
    assignment: undefined,
    binary: undefined,
    call: undefined,
    conditional: undefined,
    function: undefined,
    identifier: undefined,
    logical: undefined,
    member: undefined,
    object: undefined,
    unary: undefined,
    update: undefined,
    arrowFunction: undefined,
    isSpread: undefined,
  };
}

export const MaybeSpreadExpression = {
  encode(
    message: MaybeSpreadExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.literal !== undefined) {
      Literal.encode(message.literal, writer.uint32(10).fork()).ldelim();
    }
    if (message.array !== undefined) {
      ArrayExpression.encode(message.array, writer.uint32(18).fork()).ldelim();
    }
    if (message.assignment !== undefined) {
      AssignmentExpression.encode(
        message.assignment,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.binary !== undefined) {
      BinaryExpression.encode(
        message.binary,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.call !== undefined) {
      CallExpression.encode(message.call, writer.uint32(42).fork()).ldelim();
    }
    if (message.conditional !== undefined) {
      ConditionalExpression.encode(
        message.conditional,
        writer.uint32(50).fork()
      ).ldelim();
    }
    if (message.function !== undefined) {
      FunctionExpression.encode(
        message.function,
        writer.uint32(58).fork()
      ).ldelim();
    }
    if (message.identifier !== undefined) {
      Identifier.encode(message.identifier, writer.uint32(66).fork()).ldelim();
    }
    if (message.logical !== undefined) {
      LogicalExpression.encode(
        message.logical,
        writer.uint32(74).fork()
      ).ldelim();
    }
    if (message.member !== undefined) {
      MemberExpression.encode(
        message.member,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.object !== undefined) {
      ObjectExpression.encode(
        message.object,
        writer.uint32(90).fork()
      ).ldelim();
    }
    if (message.unary !== undefined) {
      UnaryExpression.encode(message.unary, writer.uint32(98).fork()).ldelim();
    }
    if (message.update !== undefined) {
      UpdateExpression.encode(
        message.update,
        writer.uint32(106).fork()
      ).ldelim();
    }
    if (message.arrowFunction !== undefined) {
      ArrowFunctionExpression.encode(
        message.arrowFunction,
        writer.uint32(114).fork()
      ).ldelim();
    }
    if (message.isSpread !== undefined) {
      writer.uint32(120).bool(message.isSpread);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): MaybeSpreadExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMaybeSpreadExpression();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.literal = Literal.decode(reader, reader.uint32());
          break;
        case 2:
          message.array = ArrayExpression.decode(reader, reader.uint32());
          break;
        case 3:
          message.assignment = AssignmentExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.binary = BinaryExpression.decode(reader, reader.uint32());
          break;
        case 5:
          message.call = CallExpression.decode(reader, reader.uint32());
          break;
        case 6:
          message.conditional = ConditionalExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 7:
          message.function = FunctionExpression.decode(reader, reader.uint32());
          break;
        case 8:
          message.identifier = Identifier.decode(reader, reader.uint32());
          break;
        case 9:
          message.logical = LogicalExpression.decode(reader, reader.uint32());
          break;
        case 10:
          message.member = MemberExpression.decode(reader, reader.uint32());
          break;
        case 11:
          message.object = ObjectExpression.decode(reader, reader.uint32());
          break;
        case 12:
          message.unary = UnaryExpression.decode(reader, reader.uint32());
          break;
        case 13:
          message.update = UpdateExpression.decode(reader, reader.uint32());
          break;
        case 14:
          message.arrowFunction = ArrowFunctionExpression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 15:
          message.isSpread = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MaybeSpreadExpression {
    return {
      literal: isSet(object.literal)
        ? Literal.fromJSON(object.literal)
        : undefined,
      array: isSet(object.array)
        ? ArrayExpression.fromJSON(object.array)
        : undefined,
      assignment: isSet(object.assignment)
        ? AssignmentExpression.fromJSON(object.assignment)
        : undefined,
      binary: isSet(object.binary)
        ? BinaryExpression.fromJSON(object.binary)
        : undefined,
      call: isSet(object.call)
        ? CallExpression.fromJSON(object.call)
        : undefined,
      conditional: isSet(object.conditional)
        ? ConditionalExpression.fromJSON(object.conditional)
        : undefined,
      function: isSet(object.function)
        ? FunctionExpression.fromJSON(object.function)
        : undefined,
      identifier: isSet(object.identifier)
        ? Identifier.fromJSON(object.identifier)
        : undefined,
      logical: isSet(object.logical)
        ? LogicalExpression.fromJSON(object.logical)
        : undefined,
      member: isSet(object.member)
        ? MemberExpression.fromJSON(object.member)
        : undefined,
      object: isSet(object.object)
        ? ObjectExpression.fromJSON(object.object)
        : undefined,
      unary: isSet(object.unary)
        ? UnaryExpression.fromJSON(object.unary)
        : undefined,
      update: isSet(object.update)
        ? UpdateExpression.fromJSON(object.update)
        : undefined,
      arrowFunction: isSet(object.arrowFunction)
        ? ArrowFunctionExpression.fromJSON(object.arrowFunction)
        : undefined,
      isSpread: isSet(object.isSpread) ? Boolean(object.isSpread) : undefined,
    };
  },

  toJSON(message: MaybeSpreadExpression): unknown {
    const obj: any = {};
    message.literal !== undefined &&
      (obj.literal = message.literal
        ? Literal.toJSON(message.literal)
        : undefined);
    message.array !== undefined &&
      (obj.array = message.array
        ? ArrayExpression.toJSON(message.array)
        : undefined);
    message.assignment !== undefined &&
      (obj.assignment = message.assignment
        ? AssignmentExpression.toJSON(message.assignment)
        : undefined);
    message.binary !== undefined &&
      (obj.binary = message.binary
        ? BinaryExpression.toJSON(message.binary)
        : undefined);
    message.call !== undefined &&
      (obj.call = message.call
        ? CallExpression.toJSON(message.call)
        : undefined);
    message.conditional !== undefined &&
      (obj.conditional = message.conditional
        ? ConditionalExpression.toJSON(message.conditional)
        : undefined);
    message.function !== undefined &&
      (obj.function = message.function
        ? FunctionExpression.toJSON(message.function)
        : undefined);
    message.identifier !== undefined &&
      (obj.identifier = message.identifier
        ? Identifier.toJSON(message.identifier)
        : undefined);
    message.logical !== undefined &&
      (obj.logical = message.logical
        ? LogicalExpression.toJSON(message.logical)
        : undefined);
    message.member !== undefined &&
      (obj.member = message.member
        ? MemberExpression.toJSON(message.member)
        : undefined);
    message.object !== undefined &&
      (obj.object = message.object
        ? ObjectExpression.toJSON(message.object)
        : undefined);
    message.unary !== undefined &&
      (obj.unary = message.unary
        ? UnaryExpression.toJSON(message.unary)
        : undefined);
    message.update !== undefined &&
      (obj.update = message.update
        ? UpdateExpression.toJSON(message.update)
        : undefined);
    message.arrowFunction !== undefined &&
      (obj.arrowFunction = message.arrowFunction
        ? ArrowFunctionExpression.toJSON(message.arrowFunction)
        : undefined);
    message.isSpread !== undefined && (obj.isSpread = message.isSpread);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<MaybeSpreadExpression>, I>>(
    object: I
  ): MaybeSpreadExpression {
    const message = createBaseMaybeSpreadExpression();
    message.literal =
      object.literal !== undefined && object.literal !== null
        ? Literal.fromPartial(object.literal)
        : undefined;
    message.array =
      object.array !== undefined && object.array !== null
        ? ArrayExpression.fromPartial(object.array)
        : undefined;
    message.assignment =
      object.assignment !== undefined && object.assignment !== null
        ? AssignmentExpression.fromPartial(object.assignment)
        : undefined;
    message.binary =
      object.binary !== undefined && object.binary !== null
        ? BinaryExpression.fromPartial(object.binary)
        : undefined;
    message.call =
      object.call !== undefined && object.call !== null
        ? CallExpression.fromPartial(object.call)
        : undefined;
    message.conditional =
      object.conditional !== undefined && object.conditional !== null
        ? ConditionalExpression.fromPartial(object.conditional)
        : undefined;
    message.function =
      object.function !== undefined && object.function !== null
        ? FunctionExpression.fromPartial(object.function)
        : undefined;
    message.identifier =
      object.identifier !== undefined && object.identifier !== null
        ? Identifier.fromPartial(object.identifier)
        : undefined;
    message.logical =
      object.logical !== undefined && object.logical !== null
        ? LogicalExpression.fromPartial(object.logical)
        : undefined;
    message.member =
      object.member !== undefined && object.member !== null
        ? MemberExpression.fromPartial(object.member)
        : undefined;
    message.object =
      object.object !== undefined && object.object !== null
        ? ObjectExpression.fromPartial(object.object)
        : undefined;
    message.unary =
      object.unary !== undefined && object.unary !== null
        ? UnaryExpression.fromPartial(object.unary)
        : undefined;
    message.update =
      object.update !== undefined && object.update !== null
        ? UpdateExpression.fromPartial(object.update)
        : undefined;
    message.arrowFunction =
      object.arrowFunction !== undefined && object.arrowFunction !== null
        ? ArrowFunctionExpression.fromPartial(object.arrowFunction)
        : undefined;
    message.isSpread = object.isSpread ?? undefined;
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
      MaybeSpreadExpression.encode(v!, writer.uint32(10).fork()).ldelim();
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
            MaybeSpreadExpression.decode(reader, reader.uint32())
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
        ? object.elements.map((e: any) => MaybeSpreadExpression.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ArrayExpression): unknown {
    const obj: any = {};
    if (message.elements) {
      obj.elements = message.elements.map((e) =>
        e ? MaybeSpreadExpression.toJSON(e) : undefined
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
      object.elements?.map((e) => MaybeSpreadExpression.fromPartial(e)) || [];
    return message;
  },
};

function createBaseAssignmentExpression(): AssignmentExpression {
  return { operator: 0, left: undefined, right: undefined };
}

export const AssignmentExpression = {
  encode(
    message: AssignmentExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.operator !== 0) {
      writer.uint32(8).int32(message.operator);
    }
    if (message.left !== undefined) {
      LVal.encode(message.left, writer.uint32(18).fork()).ldelim();
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
          message.left = LVal.decode(reader, reader.uint32());
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
        ? assignmentExpression_OperatorFromJSON(object.operator)
        : 0,
      left: isSet(object.left) ? LVal.fromJSON(object.left) : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
    };
  },

  toJSON(message: AssignmentExpression): unknown {
    const obj: any = {};
    message.operator !== undefined &&
      (obj.operator = assignmentExpression_OperatorToJSON(message.operator));
    message.left !== undefined &&
      (obj.left = message.left ? LVal.toJSON(message.left) : undefined);
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
    message.operator = object.operator ?? 0;
    message.left =
      object.left !== undefined && object.left !== null
        ? LVal.fromPartial(object.left)
        : undefined;
    message.right =
      object.right !== undefined && object.right !== null
        ? Expression.fromPartial(object.right)
        : undefined;
    return message;
  },
};

function createBaseBinaryExpression(): BinaryExpression {
  return { operator: 0, left: undefined, right: undefined };
}

export const BinaryExpression = {
  encode(
    message: BinaryExpression,
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

  decode(input: _m0.Reader | Uint8Array, length?: number): BinaryExpression {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBinaryExpression();
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

  fromJSON(object: any): BinaryExpression {
    return {
      operator: isSet(object.operator)
        ? binaryExpression_OperatorFromJSON(object.operator)
        : 0,
      left: isSet(object.left) ? Expression.fromJSON(object.left) : undefined,
      right: isSet(object.right)
        ? Expression.fromJSON(object.right)
        : undefined,
    };
  },

  toJSON(message: BinaryExpression): unknown {
    const obj: any = {};
    message.operator !== undefined &&
      (obj.operator = binaryExpression_OperatorToJSON(message.operator));
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

function createBaseCallExpression(): CallExpression {
  return { callee: undefined, arguments: [], option: undefined };
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
      MaybeSpreadExpression.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.option !== undefined) {
      writer.uint32(24).bool(message.option);
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
            MaybeSpreadExpression.decode(reader, reader.uint32())
          );
          break;
        case 3:
          message.option = reader.bool();
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
        ? object.arguments.map((e: any) => MaybeSpreadExpression.fromJSON(e))
        : [],
      option: isSet(object.option) ? Boolean(object.option) : undefined,
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
        e ? MaybeSpreadExpression.toJSON(e) : undefined
      );
    } else {
      obj.arguments = [];
    }
    message.option !== undefined && (obj.option = message.option);
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
      object.arguments?.map((e) => MaybeSpreadExpression.fromPartial(e)) || [];
    message.option = object.option ?? undefined;
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

function createBaseFunctionExpression(): FunctionExpression {
  return { id: undefined, params: [], body: [] };
}

export const FunctionExpression = {
  encode(
    message: FunctionExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== undefined) {
      Identifier.encode(message.id, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.params) {
      PatternLike.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    for (const v of message.body) {
      Statement.encode(v!, writer.uint32(26).fork()).ldelim();
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
          message.id = Identifier.decode(reader, reader.uint32());
          break;
        case 2:
          message.params.push(PatternLike.decode(reader, reader.uint32()));
          break;
        case 3:
          message.body.push(Statement.decode(reader, reader.uint32()));
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
      id: isSet(object.id) ? Identifier.fromJSON(object.id) : undefined,
      params: Array.isArray(object?.params)
        ? object.params.map((e: any) => PatternLike.fromJSON(e))
        : [],
      body: Array.isArray(object?.body)
        ? object.body.map((e: any) => Statement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: FunctionExpression): unknown {
    const obj: any = {};
    message.id !== undefined &&
      (obj.id = message.id ? Identifier.toJSON(message.id) : undefined);
    if (message.params) {
      obj.params = message.params.map((e) =>
        e ? PatternLike.toJSON(e) : undefined
      );
    } else {
      obj.params = [];
    }
    if (message.body) {
      obj.body = message.body.map((e) => (e ? Statement.toJSON(e) : undefined));
    } else {
      obj.body = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FunctionExpression>, I>>(
    object: I
  ): FunctionExpression {
    const message = createBaseFunctionExpression();
    message.id =
      object.id !== undefined && object.id !== null
        ? Identifier.fromPartial(object.id)
        : undefined;
    message.params =
      object.params?.map((e) => PatternLike.fromPartial(e)) || [];
    message.body = object.body?.map((e) => Statement.fromPartial(e)) || [];
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

function createBaseMemberExpression(): MemberExpression {
  return {
    object: undefined,
    propertyExpression: undefined,
    propertyIdentifier: undefined,
    computed: false,
    option: undefined,
  };
}

export const MemberExpression = {
  encode(
    message: MemberExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.object !== undefined) {
      Expression.encode(message.object, writer.uint32(10).fork()).ldelim();
    }
    if (message.propertyExpression !== undefined) {
      Expression.encode(
        message.propertyExpression,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.propertyIdentifier !== undefined) {
      Identifier.encode(
        message.propertyIdentifier,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.computed === true) {
      writer.uint32(32).bool(message.computed);
    }
    if (message.option !== undefined) {
      writer.uint32(40).bool(message.option);
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
          message.propertyExpression = Expression.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.propertyIdentifier = Identifier.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.computed = reader.bool();
          break;
        case 5:
          message.option = reader.bool();
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
      propertyExpression: isSet(object.propertyExpression)
        ? Expression.fromJSON(object.propertyExpression)
        : undefined,
      propertyIdentifier: isSet(object.propertyIdentifier)
        ? Identifier.fromJSON(object.propertyIdentifier)
        : undefined,
      computed: isSet(object.computed) ? Boolean(object.computed) : false,
      option: isSet(object.option) ? Boolean(object.option) : undefined,
    };
  },

  toJSON(message: MemberExpression): unknown {
    const obj: any = {};
    message.object !== undefined &&
      (obj.object = message.object
        ? Expression.toJSON(message.object)
        : undefined);
    message.propertyExpression !== undefined &&
      (obj.propertyExpression = message.propertyExpression
        ? Expression.toJSON(message.propertyExpression)
        : undefined);
    message.propertyIdentifier !== undefined &&
      (obj.propertyIdentifier = message.propertyIdentifier
        ? Identifier.toJSON(message.propertyIdentifier)
        : undefined);
    message.computed !== undefined && (obj.computed = message.computed);
    message.option !== undefined && (obj.option = message.option);
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
    message.propertyExpression =
      object.propertyExpression !== undefined &&
      object.propertyExpression !== null
        ? Expression.fromPartial(object.propertyExpression)
        : undefined;
    message.propertyIdentifier =
      object.propertyIdentifier !== undefined &&
      object.propertyIdentifier !== null
        ? Identifier.fromPartial(object.propertyIdentifier)
        : undefined;
    message.computed = object.computed ?? false;
    message.option = object.option ?? undefined;
    return message;
  },
};

function createBaseObjectExpression(): ObjectExpression {
  return { properties: [] };
}

export const ObjectExpression = {
  encode(
    message: ObjectExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.properties) {
      ObjectElement.encode(v!, writer.uint32(10).fork()).ldelim();
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
          message.properties.push(
            ObjectElement.decode(reader, reader.uint32())
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
      properties: Array.isArray(object?.properties)
        ? object.properties.map((e: any) => ObjectElement.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ObjectExpression): unknown {
    const obj: any = {};
    if (message.properties) {
      obj.properties = message.properties.map((e) =>
        e ? ObjectElement.toJSON(e) : undefined
      );
    } else {
      obj.properties = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectExpression>, I>>(
    object: I
  ): ObjectExpression {
    const message = createBaseObjectExpression();
    message.properties =
      object.properties?.map((e) => ObjectElement.fromPartial(e)) || [];
    return message;
  },
};

function createBaseObjectElement(): ObjectElement {
  return { method: undefined, property: undefined, spread: undefined };
}

export const ObjectElement = {
  encode(
    message: ObjectElement,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.method !== undefined) {
      ObjectMethod.encode(message.method, writer.uint32(10).fork()).ldelim();
    }
    if (message.property !== undefined) {
      ObjectProperty.encode(
        message.property,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.spread !== undefined) {
      Expression.encode(message.spread, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ObjectElement {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectElement();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.method = ObjectMethod.decode(reader, reader.uint32());
          break;
        case 2:
          message.property = ObjectProperty.decode(reader, reader.uint32());
          break;
        case 3:
          message.spread = Expression.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectElement {
    return {
      method: isSet(object.method)
        ? ObjectMethod.fromJSON(object.method)
        : undefined,
      property: isSet(object.property)
        ? ObjectProperty.fromJSON(object.property)
        : undefined,
      spread: isSet(object.spread)
        ? Expression.fromJSON(object.spread)
        : undefined,
    };
  },

  toJSON(message: ObjectElement): unknown {
    const obj: any = {};
    message.method !== undefined &&
      (obj.method = message.method
        ? ObjectMethod.toJSON(message.method)
        : undefined);
    message.property !== undefined &&
      (obj.property = message.property
        ? ObjectProperty.toJSON(message.property)
        : undefined);
    message.spread !== undefined &&
      (obj.spread = message.spread
        ? Expression.toJSON(message.spread)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectElement>, I>>(
    object: I
  ): ObjectElement {
    const message = createBaseObjectElement();
    message.method =
      object.method !== undefined && object.method !== null
        ? ObjectMethod.fromPartial(object.method)
        : undefined;
    message.property =
      object.property !== undefined && object.property !== null
        ? ObjectProperty.fromPartial(object.property)
        : undefined;
    message.spread =
      object.spread !== undefined && object.spread !== null
        ? Expression.fromPartial(object.spread)
        : undefined;
    return message;
  },
};

function createBaseObjectMethod(): ObjectMethod {
  return {
    kind: 0,
    keyExpression: undefined,
    keyIdentifier: undefined,
    keyStringLiteral: undefined,
    keyNumericLiteral: undefined,
    params: [],
    body: [],
    computed: false,
  };
}

export const ObjectMethod = {
  encode(
    message: ObjectMethod,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.kind !== 0) {
      writer.uint32(8).int32(message.kind);
    }
    if (message.keyExpression !== undefined) {
      Expression.encode(
        message.keyExpression,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.keyIdentifier !== undefined) {
      Identifier.encode(
        message.keyIdentifier,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.keyStringLiteral !== undefined) {
      writer.uint32(34).string(message.keyStringLiteral);
    }
    if (message.keyNumericLiteral !== undefined) {
      writer.uint32(40).uint64(message.keyNumericLiteral);
    }
    for (const v of message.params) {
      PatternLike.encode(v!, writer.uint32(50).fork()).ldelim();
    }
    for (const v of message.body) {
      Statement.encode(v!, writer.uint32(58).fork()).ldelim();
    }
    if (message.computed === true) {
      writer.uint32(64).bool(message.computed);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ObjectMethod {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseObjectMethod();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.kind = reader.int32() as any;
          break;
        case 2:
          message.keyExpression = Expression.decode(reader, reader.uint32());
          break;
        case 3:
          message.keyIdentifier = Identifier.decode(reader, reader.uint32());
          break;
        case 4:
          message.keyStringLiteral = reader.string();
          break;
        case 5:
          message.keyNumericLiteral = longToNumber(reader.uint64() as Long);
          break;
        case 6:
          message.params.push(PatternLike.decode(reader, reader.uint32()));
          break;
        case 7:
          message.body.push(Statement.decode(reader, reader.uint32()));
          break;
        case 8:
          message.computed = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ObjectMethod {
    return {
      kind: isSet(object.kind) ? objectMethod_KindFromJSON(object.kind) : 0,
      keyExpression: isSet(object.keyExpression)
        ? Expression.fromJSON(object.keyExpression)
        : undefined,
      keyIdentifier: isSet(object.keyIdentifier)
        ? Identifier.fromJSON(object.keyIdentifier)
        : undefined,
      keyStringLiteral: isSet(object.keyStringLiteral)
        ? String(object.keyStringLiteral)
        : undefined,
      keyNumericLiteral: isSet(object.keyNumericLiteral)
        ? Number(object.keyNumericLiteral)
        : undefined,
      params: Array.isArray(object?.params)
        ? object.params.map((e: any) => PatternLike.fromJSON(e))
        : [],
      body: Array.isArray(object?.body)
        ? object.body.map((e: any) => Statement.fromJSON(e))
        : [],
      computed: isSet(object.computed) ? Boolean(object.computed) : false,
    };
  },

  toJSON(message: ObjectMethod): unknown {
    const obj: any = {};
    message.kind !== undefined &&
      (obj.kind = objectMethod_KindToJSON(message.kind));
    message.keyExpression !== undefined &&
      (obj.keyExpression = message.keyExpression
        ? Expression.toJSON(message.keyExpression)
        : undefined);
    message.keyIdentifier !== undefined &&
      (obj.keyIdentifier = message.keyIdentifier
        ? Identifier.toJSON(message.keyIdentifier)
        : undefined);
    message.keyStringLiteral !== undefined &&
      (obj.keyStringLiteral = message.keyStringLiteral);
    message.keyNumericLiteral !== undefined &&
      (obj.keyNumericLiteral = Math.round(message.keyNumericLiteral));
    if (message.params) {
      obj.params = message.params.map((e) =>
        e ? PatternLike.toJSON(e) : undefined
      );
    } else {
      obj.params = [];
    }
    if (message.body) {
      obj.body = message.body.map((e) => (e ? Statement.toJSON(e) : undefined));
    } else {
      obj.body = [];
    }
    message.computed !== undefined && (obj.computed = message.computed);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ObjectMethod>, I>>(
    object: I
  ): ObjectMethod {
    const message = createBaseObjectMethod();
    message.kind = object.kind ?? 0;
    message.keyExpression =
      object.keyExpression !== undefined && object.keyExpression !== null
        ? Expression.fromPartial(object.keyExpression)
        : undefined;
    message.keyIdentifier =
      object.keyIdentifier !== undefined && object.keyIdentifier !== null
        ? Identifier.fromPartial(object.keyIdentifier)
        : undefined;
    message.keyStringLiteral = object.keyStringLiteral ?? undefined;
    message.keyNumericLiteral = object.keyNumericLiteral ?? undefined;
    message.params =
      object.params?.map((e) => PatternLike.fromPartial(e)) || [];
    message.body = object.body?.map((e) => Statement.fromPartial(e)) || [];
    message.computed = object.computed ?? false;
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

function createBaseArrowFunctionExpression(): ArrowFunctionExpression {
  return { params: [], statement: [], expression: undefined };
}

export const ArrowFunctionExpression = {
  encode(
    message: ArrowFunctionExpression,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.params) {
      PatternLike.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.statement) {
      Statement.encode(v!, writer.uint32(18).fork()).ldelim();
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
          message.params.push(PatternLike.decode(reader, reader.uint32()));
          break;
        case 2:
          message.statement.push(Statement.decode(reader, reader.uint32()));
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
        ? object.params.map((e: any) => PatternLike.fromJSON(e))
        : [],
      statement: Array.isArray(object?.statement)
        ? object.statement.map((e: any) => Statement.fromJSON(e))
        : [],
      expression: isSet(object.expression)
        ? Expression.fromJSON(object.expression)
        : undefined,
    };
  },

  toJSON(message: ArrowFunctionExpression): unknown {
    const obj: any = {};
    if (message.params) {
      obj.params = message.params.map((e) =>
        e ? PatternLike.toJSON(e) : undefined
      );
    } else {
      obj.params = [];
    }
    if (message.statement) {
      obj.statement = message.statement.map((e) =>
        e ? Statement.toJSON(e) : undefined
      );
    } else {
      obj.statement = [];
    }
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
      object.params?.map((e) => PatternLike.fromPartial(e)) || [];
    message.statement =
      object.statement?.map((e) => Statement.fromPartial(e)) || [];
    message.expression =
      object.expression !== undefined && object.expression !== null
        ? Expression.fromPartial(object.expression)
        : undefined;
    return message;
  },
};

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
