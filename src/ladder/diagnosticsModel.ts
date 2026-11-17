/** Never drop individual diagnostic locations when condensing repeated warnings. */
export type DiagnosticItem={
  severity:"Warning"|"Error";line:number;code:string;message:string;
};
export type DiagnosticGroup={
  severity:"Warning"|"Error";code:string;firstLine:number;items:DiagnosticItem[];
};
export function groupDiagnostics(items:readonly DiagnosticItem[]):DiagnosticGroup[]{
  const groups=new Map<string,DiagnosticGroup>();
  for(const item of items){
    const key=item.severity+"\u0000"+item.code;
    let group=groups.get(key);
    if(!group){
      group={severity:item.severity,code:item.code,firstLine:item.line,items:[]};
      groups.set(key,group);
    }
    group.items.push(item);
    group.firstLine=Math.min(group.firstLine,item.line);
  }
  return [...groups.values()].sort((a,b)=>
    (a.severity===b.severity?0:a.severity==="Error"?-1:1)||
    b.items.length-a.items.length||a.firstLine-b.firstLine);
}
