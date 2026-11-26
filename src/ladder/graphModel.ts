/** Offline-only graph draft and bounded layout helpers. No CIOPRG file writer. */

import type { Controller, SemanticNode as CoreSemanticNode, SemanticRung as CoreSemanticRung } from "../engineering/types.ts";
export type { Controller } from "../engineering/types.ts";

export type ContactExpr = {
  kind: "contact";
  id: string;
  relay: string;
  normallyClosed: boolean;
};
export type JoinExpr = {
  kind: "and" | "or";
  id: string;
  left: GraphExpr;
  right: GraphExpr;
};
export type GraphExpr = ContactExpr | JoinExpr;

export type DraftRung = {
  id: string;
  output: string;
  comment: string;
  expression: GraphExpr;
};
export type GraphDraft = { version: 1; controller: Controller; rungs: DraftRung[] };

// Graph rendering accepts the minimal projection; IPC nodes retain byte spans.
export type SemanticNode = Pick<CoreSemanticNode, "op" | "line">;
export type SemanticRung = Omit<CoreSemanticRung, "inputs">;

export type WireSegment = { x1: number; y1: number; x2: number; y2: number };
export type ContactLocation = ContactExpr & {
  x: number; y: number; width: number; height: number;
};
export type CircuitLayout = {
  width: number;
  height: number;
  contacts: ContactLocation[];
  wires: WireSegment[];
};
type Size = { width: number; height: number };
const CONTACT_WIDTH = 144;
const CONTACT_HEIGHT = 90;
const SERIES_GAP = 30;
const PARALLEL_X = 54;
const PARALLEL_Y = 28;

export function relayText(number: number): string {
  return "#" + String(number).padStart(5, "0");
}

export function makeContact(relay = "#70010", normallyClosed = false): ContactExpr {
  const id = typeof crypto !== "undefined" && typeof crypto.randomUUID === "function"
    ? crypto.randomUUID()
    : "contact-" + Date.now().toString(36) + "-" + Math.random().toString(36).slice(2);
  return { kind: "contact", id, relay, normallyClosed };
}

function makeId(): string {
  return makeContact().id;
}

export function newDraft(): GraphDraft {
  return { version: 1, controller: "YRC1000", rungs: [
    { id: makeId(), comment: "Демонстрационная цепь — только JSON-черновик",
      output: "#70020", expression: makeContact("#20010") },
  ] };
}

export function appendDraftRung(draft: GraphDraft): GraphDraft {
  if (draft.rungs.length >= 128) throw new Error("Достигнут предел 128 цепей.");
  const n = draft.rungs.length;
  return { ...draft, rungs: [...draft.rungs, {
    id: makeId(), comment: "", output: relayText(70020 + n * 10),
    expression: makeContact("#70010"),
  }] };
}

/** Treat the expression as an immutable tree. Node IDs remain stable on edits. */
export function changeAt(
  expression: GraphExpr, targetId: string, update: (old: GraphExpr) => GraphExpr,
): GraphExpr {
  if (expression.id === targetId) return update(expression);
  if (expression.kind === "contact") return expression;
  const left = changeAt(expression.left, targetId, update);
  const right = changeAt(expression.right, targetId, update);
  if (left === expression.left && right === expression.right) return expression;
  return { ...expression, left, right };
}

export function findContact(expression: GraphExpr, targetId: string): ContactExpr | null {
  const stack: GraphExpr[] = [expression];
  while (stack.length) {
    const node = stack.pop()!;
    if (node.id === targetId && node.kind === "contact") return node;
    if (node.kind !== "contact") stack.push(node.right, node.left);
  }
  return null;
}

export function addBranch(
  expression: GraphExpr, targetId: string | null, kind: "and" | "or",
): GraphExpr {
  const id = targetId ?? expression.id;
  return changeAt(expression, id, old => ({
    kind, id: makeId(), left: old, right: makeContact("#70010"),
  }));
}

export function deleteContact(expression: GraphExpr, id: string): GraphExpr {
  if (expression.id === id) return expression; // can't delete final contact
  if (expression.kind === "contact") return expression;
  if (expression.left.id === id) return expression.right;
  if (expression.right.id === id) return expression.left;
  const left = deleteContact(expression.left, id);
  const right = deleteContact(expression.right, id);
  return left === expression.left && right === expression.right
    ? expression : { ...expression, left, right };
}

/** Read-only conversion from Rust's indexed AST; errors never become a guessed graph. */
export function semanticToExpression(
  nodes: SemanticNode[], root: number, maxNodes = 256,
): GraphExpr {
  if (!Number.isInteger(root) || root < 0 || root >= nodes.length) {
    throw new Error("Узел корня отсутствует в ограниченном семантическом предпросмотре.");
  }
  const reachable = new Set<number>();
  const pending = [root];
  while (pending.length) {
    const id = pending.pop()!;
    if (reachable.has(id)) continue;
    if (id < 0 || id >= nodes.length || reachable.size >= maxNodes) {
      throw new Error("Слишком большое или повреждённое выражение для графического просмотра.");
    }
    const node = nodes[id];
    if (!node) throw new Error("Недоступный узел AST.");
    reachable.add(id);
    if ("And" in node.op || "Or" in node.op) {
      const child = "And" in node.op ? node.op.And : node.op.Or;
      if (child.left >= id || child.right >= id || child.left < 0 || child.right < 0) {
        throw new Error("Недопустимые ссылки внутри AST.");
      }
      pending.push(child.left, child.right);
    } else if (!("Contact" in node.op)) {
      throw new Error("Неподдерживаемый тип AST.");
    }
  }
  const built = new Map<number, GraphExpr>();
  for (const id of [...reachable].sort((a, b) => a - b)) {
    const op = nodes[id].op;
    if ("Contact" in op) {
      built.set(id, { kind: "contact", id: "ast-" + id,
        relay: relayText(op.Contact.relay), normallyClosed: op.Contact.inverted });
    } else {
      const p = "And" in op ? op.And : op.Or;
      const left = built.get(p.left), right = built.get(p.right);
      if (!left || !right) throw new Error("Неполный граф AST.");
      built.set(id, { kind: "And" in op ? "and" : "or", id: "ast-" + id, left, right });
    }
  }
  return built.get(root)!;
}

/**
 * Lay out contacts and branch wiring as an electrical-style left-to-right
 * network: AND = series; OR = parallel. No hardware energization inference.
 * Iterative traversal guards renderer resources and untrusted source AST.
 */
export function layoutCircuit(expression: GraphExpr): CircuitLayout {
  const sizes = new Map<string, Size>();
  const post: GraphExpr[] = [];
  const stack: { node: GraphExpr; depth: number; after: boolean }[] =
    [{ node: expression, depth: 1, after: false }];
  let elements = 0;
  const seen = new Set<string>();
  while (stack.length) {
    const {node,depth,after} = stack.pop()!;
    if (!after) {
      elements++;
      if (elements > 256 || depth > 32 || seen.has(node.id))
        throw new Error("Слишком сложная структура или повторяющиеся идентификаторы элементов.");
      seen.add(node.id);
      stack.push({node,depth,after:true});
      if (node.kind !== "contact") {
        stack.push({node:node.right,depth:depth+1,after:false},
          {node:node.left,depth:depth+1,after:false});
      }
    } else {
      post.push(node);
      if (node.kind === "contact") {
        sizes.set(node.id, {width:CONTACT_WIDTH,height:CONTACT_HEIGHT});
      } else {
        const l=sizes.get(node.left.id)!, r=sizes.get(node.right.id)!;
        sizes.set(node.id, node.kind==="and" ?
          { width:l.width + SERIES_GAP + r.width, height:Math.max(l.height,r.height) } :
          { width:Math.max(l.width,r.width) + PARALLEL_X*2,
            height:l.height+PARALLEL_Y+r.height });
      }
    }
  }
  const contacts: ContactLocation[] = [];
  const wires: WireSegment[] = [];
  const rootSize=sizes.get(expression.id)!;
  const draw: {node:GraphExpr;x:number;y:number}[]=[{node:expression,x:0,y:0}];
  function line(x1:number,y1:number,x2:number,y2:number) { wires.push({x1,y1,x2,y2}); }
  while(draw.length) {
    const {node,x,y}=draw.pop()!;
    const box=sizes.get(node.id)!;
    if(node.kind==="contact") {
      contacts.push({...node,x,y,width:box.width,height:box.height});
      continue;
    }
    const l=sizes.get(node.left.id)!,r=sizes.get(node.right.id)!;
    if(node.kind==="and") {
      const ly=y+(box.height-l.height)/2, ry=y+(box.height-r.height)/2;
      line(x+l.width,y+box.height/2,x+l.width+SERIES_GAP,y+box.height/2);
      draw.push({node:node.right,x:x+l.width+SERIES_GAP,y:ry},
        {node:node.left,x,y:ly});
    } else {
      const yTop=y+l.height/2;
      const yBottom=y+l.height+PARALLEL_Y+r.height/2;
      const mid=y+box.height/2;
      const xL=x+PARALLEL_X/2, xR=x+box.width-PARALLEL_X/2;
      const childX=x+PARALLEL_X;
      line(x,mid,xL,mid);
      line(xL,yTop,xL,yBottom);
      line(xR,yTop,xR,yBottom);
      line(xR,mid,x+box.width,mid);
      line(xL,yTop,childX,yTop); line(xL,yBottom,childX,yBottom);
      line(childX+l.width,yTop,xR,yTop);
      line(childX+r.width,yBottom,xR,yBottom);
      draw.push({node:node.right,x:childX,y:y+l.height+PARALLEL_Y},
        {node:node.left,x:childX,y});
    }
  }
  return {width:rootSize.width,height:rootSize.height,contacts,wires};
}

/** Independent offline condition evaluation for synthetic draft tests. */
export function evaluateDraft(expression: GraphExpr, signals: Record<string, boolean | undefined>): boolean | undefined {
  const order: GraphExpr[]=[];
  const stack:[GraphExpr,boolean][]=[[expression,false]];
  while (stack.length) {
    const [x, done]=stack.pop()!;
    if(done) order.push(x);
    else {
      stack.push([x,true]);
      if(x.kind!=="contact") stack.push([x.right,false],[x.left,false]);
    }
    if(order.length+stack.length>1200) throw new Error("Слишком сложное выражение.");
  }
  const values=new Map<string,boolean|undefined>();
  for(const n of order) {
    if(n.kind==="contact") {
      const value=signals[n.relay];
      values.set(n.id,value===undefined?undefined:(n.normallyClosed?!value:value));
    } else {
      const a=values.get(n.left.id),b=values.get(n.right.id);
      values.set(n.id,n.kind==="and"?(a===false||b===false?false:a===true&&b===true?true:undefined):
        (a===true||b===true?true:a===false&&b===false?false:undefined));
    }
  }
  return values.get(expression.id);
}
