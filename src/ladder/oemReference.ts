/**
 * OEM manual evidence *labels*, not executable semantics.
 * Per-command topics refer to YRC1000 Concurrent I/O R12 §§12.2/12.3.
 * Full controller/firmware equivalence, runtime signal timing and native
 * editing permissions are explicitly unknown until isolated bench tests.
 */
export type OemManualEntry={
  mnemonic:"MOV"|"ADD"|"SUB"|"SHL"|"SHR";
  format:string;
  whenOn:string;
  carry:string;
  zero:string;
  error:string;
  limits:string;
  unresolved:string;
  section:string;
};
const entries:Record<OemManualEntry["mnemonic"],OemManualEntry>={
  MOV:{
    mnemonic:"MOV",format:"MOV S,D",
    whenOn:"При истинном входном условии передаёт S в D.",
    carry:"Для байтового D руководство описывает Carry при значении передачи более 256 (BCD); точная граница 256 требует проверки.",
    zero:"Не используется (в таблице: 0).",
    error:"Не используется (в таблице: 0).",
    limits:"S: M000–M999 / 0–65535 / byte / word; D: M000–M599 / byte / word.",
    unresolved:"Усечение до 8 бит, передача 256 в byte, конфликт записи в TMR/CNT и флаги в полном скане.",
    section:"§12.2.24",
  },
  ADD:{
    mnemonic:"ADD",format:"ADD S1,S2,D",
    whenOn:"16-битное беззнаковое сложение, D=(S1+S2) mod 65536.",
    carry:"1, если сумма ≥ 65536.",
    zero:"1, если результат D=0.",
    error:"Для этой команды не используется (0).",
    limits:"S1/S2: M000–M999 или 0–65535; D: M000–M599.",
    unresolved:"Порядок сканирования, EN во взаимодействии с другими командами, конфликт с TMR/CNT.",
    section:"§12.2.17",
  },
  SUB:{
    mnemonic:"SUB",format:"SUB S1,S2,D",
    whenOn:"16-битное беззнаковое вычитание, D=(S1−S2) mod 65536.",
    carry:"1 при отрицательной разности (S1<S2).",
    zero:"1, если результат D=0.",
    error:"Для этой команды не используется (0).",
    limits:"S1/S2: M000–M999 или 0–65535; D: M000–M599.",
    unresolved:"EN и приоритеты флагов в полном скане, дополнительные эффекты при пересечении регистров.",
    section:"§12.2.18",
  },
  SHL:{
    mnemonic:"SHL",format:"SHL S,n,D",
    whenOn:"Логический сдвиг 16-битного S влево n раз, в младшие биты вводится 0.",
    carry:"Последний вытесненный старший бит (при 1≤n≤15 в 16-битной эталонной модели).",
    zero:"В перечне команд указано, что Zero не используется (0).",
    error:"В перечне команд указано, что Error не используется (0).",
    limits:"S: M / 0–65535 / byte / word; n: 0–16; D: M / byte / word.",
    unresolved:"Carry при n=0 и n=16, точная ширина/усечение при байтовом адресе, порядок выполнения в скане.",
    section:"§12.2.29",
  },
  SHR:{
    mnemonic:"SHR",format:"SHR S,n,D",
    whenOn:"Логический сдвиг 16-битного S вправо n раз, в старшие биты вводится 0.",
    carry:"Последний вытесненный младший бит (при 1≤n≤15 в 16-битной эталонной модели).",
    zero:"В перечне команд указано, что Zero не используется (0).",
    error:"В перечне команд указано, что Error не используется (0).",
    limits:"S: M / 0–65535 / byte / word; n: 0–16; D: M / byte / word.",
    unresolved:"Carry при n=0 и n=16, точная ширина/усечение при байтовом адресе, порядок выполнения в скане.",
    section:"§12.2.30",
  },
};
export function oemManualEntry(mnemonic:string):OemManualEntry|null{
  return Object.hasOwn(entries,mnemonic)?entries[mnemonic as OemManualEntry["mnemonic"]]:null;
}
