/** UI syntax-only validation. Rust's controller-specific memory validator remains authoritative.
 * No accepted value constitutes permission to write a Yaskawa controller.
 */
export type InlineOperandKind = "relay" | "group" | "current" | "preset";

export function inlineOperandError(raw: string, kind: InlineOperandKind): string | null {
  const value=raw.trim();
  if (value.length>24) return "Значение слишком длинное";
  if (kind==="relay" || kind==="group") {
    if (!/^#[0-9]{5}$/.test(value)) return "Адрес реле: #XXXXX (пять цифр)";
    const bit=Number(value.slice(-1));
    if (bit>7) return "Индекс бита должен быть 0–7";
    if (kind==="group" && bit!==0) return "GSTR/GOUT: адрес байта должен оканчиваться на 0";
    return null;
  }
  if (kind==="current") {
    if (!/^M[0-9]{3}$/.test(value)) return "Текущий регистр: M000–M559";
    if (Number(value.slice(1))>559) return "Для TMR/CNT допускаются только M000–M559";
    return null;
  }
  if (/^M[0-9]{3}$/.test(value)) return null;
  if (/^[0-9]+$/.test(value) && Number(value)<=65535) return null;
  return "Уставка: M000–M999 или целое 0–65535";
}
