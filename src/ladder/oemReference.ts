/**
 * OEM manual evidence *labels*, not executable semantics.
 * Per-command topics refer to YRC1000 Concurrent I/O R12 §§12.2/12.3.
 * Full controller/firmware equivalence, runtime signal timing and native
 * editing permissions are explicitly unknown until isolated bench tests.
 */
export type OemManualEntry={
  mnemonic:string;
  format:string;
  whenOn:string;
  carry:string;
  zero:string;
  error:string;
  limits:string;
  unresolved:string;
  section:string;
};
const entries:Record<string,OemManualEntry>={
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

// Expanded YRC1000 Concurrent I/O R12 §12.2 evidence. All forms are
// non-executable source annotations; these flag fields explicitly do NOT
// expose firmware-calculated values, runtime state or a robot write interface.
function manualOnly(mnemonic:string,format:string,whenOn:string,
  limits:string,section:string,unresolved:string):OemManualEntry {
  const notSimulated="Руководство описывает флаг; MLAD не вычисляет его значение в реальном скане.";
  return {
    mnemonic,format,whenOn,limits,section,
    carry:notSimulated,zero:notSimulated,error:notSimulated,
    unresolved:unresolved+"; условия EN/сканирования и конкретная прошивка не проверены.",
  };
}
Object.assign(entries,{
  MUL:manualOnly("MUL","MUL S1,S2,Dn",
    "Умножение двух 16-битных беззнаковых источников; результат в паре Dn (low), Dn+1 (high).",
    "S1/S2: M000–M999 или 0–65535; Dn/Dn+1: соседние M, консервативно Dn=M000–M598.",
    "§12.2.19","Статический анализ диапазонов пар и системные флаги не моделируются"),
  DIV:manualOnly("DIV","DIV S1,S2,D",
    "Частное при истинном входном условии; деление на ноль связано с Error Flag.",
    "S1/S2: M000–M999 или 0–65535; D: M000–M599.",
    "§12.2.20","Деление на ноль, состояние Error Flag и значение D при ошибке не вычисляются"),
  MOD:manualOnly("MOD","MOD S1,S2,D",
    "Остаток деления при истинном входном условии; деление на ноль связано с Error Flag.",
    "S1/S2: M000–M999 или 0–65535; D: M000–M599.",
    "§12.2.21","Деление на ноль, Error Flag и значение D при ошибке не моделируются"),
  BIN:manualOnly("BIN","BIN S,D",
    "Преобразование BCD в двоичное представление по руководству.",
    "S: M000–M999, byte либо word relay (не immediate); D: M000–M599, byte либо word.",
    "§12.2.22","Значение при невалидном BCD и граничные byte/Carry не вычисляются"),
  BCD:manualOnly("BCD","BCD S,D",
    "Преобразование двоичного S в BCD при истинном входном условии.",
    "S: M000–M999, byte либо word relay (не immediate); D: M000–M599, byte либо word.",
    "§12.2.23","Ошибка для S≥10000, byte-переполнение и значения флагов не вычисляются"),
  WAND:manualOnly("WAND","WAND S1,S2,D",
    "Побитовый AND источников S1 и S2.",
    "S1/S2: M000–M999, 0–65535, byte/word; D: M000–M599, byte/word.",
    "§12.2.25","Совмещение byte/word и перекрывающиеся назначения не моделируются"),
  WOR:manualOnly("WOR","WOR S1,S2,D",
    "Побитовый OR источников S1 и S2.",
    "S1/S2: M000–M999, 0–65535, byte/word; D: M000–M599, byte/word.",
    "§12.2.26","Совмещение byte/word и перекрывающиеся назначения не моделируются"),
  WXOR:manualOnly("WXOR","WXOR S1,S2,D",
    "Побитовый XOR источников S1 и S2.",
    "S1/S2: M000–M999, 0–65535, byte/word; D: M000–M599, byte/word.",
    "§12.2.27","Совмещение byte/word и перекрывающиеся назначения не моделируются"),
  WNOT:manualOnly("WNOT","WNOT S,D",
    "Побитовое отрицание значения S.",
    "S: M000–M999, 0–65535, byte/word; D: M000–M599, byte/word.",
    "§12.2.28","Ширина инверсии для byte/word в живом скане не моделируется"),
  ROL:manualOnly("ROL","ROL S,n,D",
    "Циклический сдвиг 16 бит влево на n позиций.",
    "S: M000–M999, 0–65535, byte/word; n: 0–16; D: M000–M599, byte/word.",
    "§12.2.31","Carry при n=0/16 и влияние 8-битных групп не вычисляются"),
  ROR:manualOnly("ROR","ROR S,n,D",
    "Циклический сдвиг 16 бит вправо на n позиций.",
    "S: M000–M999, 0–65535, byte/word; n: 0–16; D: M000–M599, byte/word.",
    "§12.2.32","Carry при n=0/16 и влияние 8-битных групп не вычисляются"),
  BMOV:manualOnly("BMOV","BMOV S,n,D",
    "Передача n байт из исходной области S в D (не n слов).",
    "S: M000–M999 либо byte relay; n: 1–999 байт; D: M000–M599 либо byte relay.",
    "§12.2.33","Пересечение диапазонов, выход за границы и младший байт регистра не проверяются"),
  CMP:manualOnly("CMP","CMP S1,S2",
    "Сравнение S1 с S2; отдельного операнда D нет.",
    "S1/S2: M000–M999, 0–65535, byte/word.",
    "§12.2.34","Carry/Zero и условия EN в реальном скане не моделируются"),
  PLS:manualOnly("PLS","PLS #XXXXX",
    "Импульсный выход по положительному фронту входного условия.",
    "Один relay #XXXXX, не M-регистр и не групповая операция.",
    "§12.2.15","Ширина импульса и дублирование катушек в прошивке не подтверждены"),
  PLF:manualOnly("PLF","PLF #XXXXX",
    "Импульсный выход по отрицательному фронту входного условия.",
    "Один relay #XXXXX, не M-регистр и не групповая операция.",
    "§12.2.16","Ширина импульса и дублирование катушек в прошивке не подтверждены"),
} satisfies Record<string,OemManualEntry>);

export function oemManualEntry(mnemonic:string):OemManualEntry|null{
  return Object.hasOwn(entries,mnemonic)?entries[mnemonic]??null:null;
}
