use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, space0, space1, digit1},
    combinator::{map, recognize},
    IResult,
};
use crate::LadderInstruction;

/// Парсер пятизначного адреса операнда (например, "70010") или буквенно-цифрового идентификатора
fn parse_operand(input: &str) -> IResult<&str, String> {
    map(
        recognize(take_while1(|c: char| c.is_ascii_alphanumeric())),

        |s: &str| s.to_string()
    )(input)
}

/// Разбирает одну строку инструкции типа "Мнемоника Операнд" (например, "STR 10010")
fn parse_bit_instruction(input: &str) -> IResult<&str, (String, String)> {
    let (input, mnemonic) = alt((
        tag("STR-NOT"),
        tag("STR"),
        tag("AND-NOT"),
        tag("AND"),
        tag("OR-NOT"),
        tag("OR"),
        tag("OUT"),
    ))(input)?;

    let (input, _) = space1(input)?;
    let (input, operand) = parse_operand(input)?;

    Ok((input, (mnemonic.to_string(), operand)))
}

/// Разбирает функциональный блок таймера: "TMR [номер], [уставка]" (например, "TMR 1, 10")
fn parse_timer_instruction(input: &str) -> IResult<&str, (String, String)> {
    let (input, _) = tag("TMR")(input)?;
    let (input, _) = space1(input)?;
    let (input, timer_id) = digit1(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;
    let (input, setpoint) = digit1(input)?;

    // Объединяем параметры в строку через запятую для совместимости с базовой моделью
    let operand_str = format!("{},{}", timer_id, setpoint);
    Ok((input, ("TMR".to_string(), operand_str)))
}

/// Главная функция разбора одной текстовой строки листинга
pub fn parse_line(line: &str) -> Option<LadderInstruction> {
    let trimmed = line.trim();
    
    // Игнорируем пустые строки, заголовки секций и комментарии
    if trimmed.is_empty() || trimmed.starts_with('/') || trimmed.starts_with(';') || trimmed.starts_with("IOCPT") || trimmed == "END" {
        return None;
    }

    // Удаляем комментарии в конце строки перед парсингом кода
    let code_part = trimmed.split(';').next()?.trim();

    // Пробуем разобрать как битовую инструкцию или как таймер
    let parse_result: IResult<&str, (String, String)> = alt((
        parse_bit_instruction,
        parse_timer_instruction,
    ))(code_part);

    match parse_result {
        Ok((_, (mnemonic, operand))) => Some(LadderInstruction {
            id: uuid::Uuid::new_v4().to_string(), // Генерируем уникальный ID для фронтенд-нод
            inst_type: mnemonic,
            operand,
        }),
        Err(_) => None,
    }
}

/// Парсит весь текст файла .LST и группирует инструкции в логические цепи (Rungs)
pub fn parse_lst_to_rungs(content: &str) -> Vec<crate::Rung> {
    let mut rungs = Vec::new();
    let mut current_instructions = Vec::new();
    let mut current_comment: Option<String> = None;
    let mut rung_counter = 1;

    for line in content.lines() {
        let trimmed = line.trim();

        // Если строка содержит глобальный комментарий к цепи, запоминаем его
        if trimmed.starts_with(';') {
            current_comment = Some(trimmed.trim_start_matches(';').trim().to_string());
            continue;
        }

        if let Some(inst) = parse_line(line) {
            let is_output = inst.inst_type == "OUT" || inst.inst_type == "TMR";
            current_instructions.push(inst);

            // В логике Yaskawa инструкция OUT или функциональный блок завершают текущую цепь
            if is_output {
                rungs.push(crate::Rung {
                    id: rung_counter,
                    comment: current_comment.take(),
                    instructions: std::mem::take(&mut current_instructions),
                    is_active: false,
                });
                rung_counter += 1;
            }
        }
    }

    // Собираем остаточные инструкции, если файл завершился без явного OUT
    if !current_instructions.is_empty() {
        rungs.push(crate::Rung {
            id: rung_counter,
            comment: current_comment,
            instructions: current_instructions,
            is_active: false,
        });
    }

    rungs
}
