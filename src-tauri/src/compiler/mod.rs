pub mod parser;

use serde::{Deserialize, Serialize};

// Повторно экспортируем структуры данных из main для использования внутри компилятора
use crate::Rung;

/// Перечисление всех поддерживаемых типов токенов инструкций Yaskawa Concurrent I/O
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Str(String),
    StrNot(String),
    And(String),
    AndNot(String),
    Or(String),
    OrNot(String),
    Out(String),
    Tmr { timer_id: u32, setpoint: u32 },
    Cnt { counter_id: u32, setpoint: u32 },
    Mov { src: String, dest: String },
    Unknown(String),
}

/// Результат статического анализа и валидации цепи
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub rung_id: usize,
    pub error_type: String,
    pub message: String,
}

/// На лету проверяет цепи на потенциальные логические ошибки (Double Coil, Недопустимые адреса)
pub fn validate_rungs(rungs: &[Rung]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut written_coils = std::collections::HashSet::new();

    for rung in rungs {
        for inst in &rung.instructions {
            // 1. Проверка на Double Coil (Запись в одну катушку в разных цепях)
            if inst.inst_type == "OUT" {
                if !written_coils.insert(inst.operand.clone()) {
                    errors.push(ValidationError {
                        rung_id: rung.id,
                        error_type: "DoubleCoil".to_string(),
                        message: format!("Внимание: Дублирование записи в катушку OUT {}. Это может вызвать гонку сигналов.", inst.operand),
                    });
                }

                // 2. Валидация диапазонов записи (Запрещено писать в Read-Only диапазоны 1xxxx, 3xxxx, 5xxxx, 8xxxx)
                let first_char = inst.operand.chars().next().unwrap_or(' ');
                if matches!(first_char, '1' | '3' | '5' | '8') {
                    let type_desc = match first_char {
                        '1' => "физический вход (External Input 1xxxx)",
                        '3' => "системный вход робота (Robot Input 3xxxx)",
                        '5' => "вход интерфейса INFORM (Interface Input 5xxxx)",
                        '8' => "системный статус/псевдо-вход (Control Status / Pseudo Input 8xxxx)",
                        _ => "read-only сигнал",
                    };
                    errors.push(ValidationError {
                        rung_id: rung.id,
                        error_type: "ReadOnlyAddress".to_string(),
                        message: format!("Ошибка: Попытка записи OUT в {}: {}. Сигнал доступен только для чтения.", type_desc, inst.operand),
                    });
                }
            }

            // 3. Валидация индекса бита (последний символ адреса от 0 до 7)
            if (inst.inst_type == "STR" || inst.inst_type == "AND" || inst.inst_type == "OR" || inst.inst_type == "OUT") 
                && inst.operand.len() == 5 
            {
                if let Some(last_char) = inst.operand.chars().last() {
                    if !('0'..='7').contains(&last_char) {
                        errors.push(ValidationError {
                            rung_id: rung.id,
                            error_type: "InvalidBitIndex".to_string(),
                            message: format!("Ошибка: Неверный индекс бита в адресе {}. Индекс должен быть в диапазоне 0-7.", inst.operand),
                        });
                    }
                }
            }
        }
    }

    errors
}
#[cfg(test)]
mod tests;