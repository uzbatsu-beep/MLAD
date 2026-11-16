/** Offline-only tri-state presentation for Ladder contacts and coils.
 * No HSES/robot data is accessed or implied by this module.
 */
import type { ContactExpr, GraphExpr } from "./graphModel.ts";

export type LogicState = "TRUE" | "FALSE" | "UNKNOWN";
export type SignalSnapshot = Readonly<Record<string, boolean | undefined>>;

export const STATE_COLORS: Readonly<Record<LogicState, string>> = {
  TRUE: "#46c784",
  FALSE: "#ef6c71",
  UNKNOWN: "#9baabb",
};

export const STATE_BACKGROUNDS: Readonly<Record<LogicState, string>> = {
  TRUE: "#183a30",
  FALSE: "#48272d",
  UNKNOWN: "#293541",
};

export const STATE_LABELS: Readonly<Record<LogicState, string>> = {
  TRUE: "ИСТИНА · 1",
  FALSE: "ЛОЖЬ · 0",
  UNKNOWN: "НЕИЗВЕСТНО · ?",
};

export function toLogicState(value: boolean | undefined | null): LogicState {
  return value === true ? "TRUE" : value === false ? "FALSE" : "UNKNOWN";
}

/** NO follows its raw bit, NC inverts it. Unknown never becomes false. */
export function contactLogicState(contact: Pick<ContactExpr, "relay" | "normallyClosed">,
  signals: SignalSnapshot): LogicState {
  const raw = signals[contact.relay];
  if (raw === undefined) return "UNKNOWN";
  return toLogicState(contact.normallyClosed ? !raw : raw);
}

/** Pure offline Boolean condition, not a live output value or robot scan. */
export function expressionLogicState(expr: GraphExpr, signals: SignalSnapshot): LogicState {
  const stack: { node: GraphExpr; visited: boolean }[] = [{node: expr, visited: false}];
  const values: (boolean | undefined)[] = [];
  let steps = 0;
  while (stack.length) {
    if (++steps > 1024) return "UNKNOWN";
    const entry = stack.pop()!;
    if (!entry.visited) {
      stack.push({node: entry.node, visited: true});
      if (entry.node.kind !== "contact") {
        stack.push({node: entry.node.right, visited: false}, {node: entry.node.left, visited: false});
      }
      continue;
    }
    if (entry.node.kind === "contact") {
      const state = contactLogicState(entry.node, signals);
      values.push(state === "UNKNOWN" ? undefined : state === "TRUE");
      continue;
    }
    const right = values.pop(), left = values.pop();
    if (entry.node.kind === "and") {
      values.push(left === false || right === false ? false :
        left === true && right === true ? true : undefined);
    } else {
      values.push(left === true || right === true ? true :
        left === false && right === false ? false : undefined);
    }
  }
  return values.length === 1 ? toLogicState(values[0]) : "UNKNOWN";
}

export function nextManualSignal(current: boolean | undefined): boolean | undefined {
  // Operator-controlled test input: ? -> 0 -> 1 -> ?.
  return current === undefined ? false : current === false ? true : undefined;
}
