#[cfg(test)]
mod compiler_tests {
    use crate::compiler::parser::{parse_line, parse_lst_to_rungs};
    use crate::compiler::validate_rungs;

    #[test]
    fn test_parse_valid_bit_instruction() {
        let line = "STR 10010";
        let parsed = parse_line(line);
        assert!(parsed.is_some());
        
        let inst = parsed.unwrap();
        assert_eq!(inst.inst_type, "STR");
        assert_eq!(inst.operand, "10010");
    }

    #[test]
    fn test_parse_timer_instruction() {
        let line = "TMR 5, 25";
        let parsed = parse_line(line);
        assert!(parsed.is_some());
        
        let inst = parsed.unwrap();
        assert_eq!(inst.inst_type, "TMR");
        assert_eq!(inst.operand, "5,25");
    }

    #[test]
    fn test_parse_ignore_comments_and_headers() {
        assert!(parse_line("; Это просто текстовый комментарий").is_none());
        assert!(parse_line("/JOB").is_none());
        assert!(parse_line("IOCPT STRSTART").is_none());
        assert!(parse_line("END").is_none());
        assert!(parse_line("   ").is_none());
    }

    #[test]
    fn test_parse_entire_lst_content() {
        let mock_lst = r#"
            /JOB
            //NAME CONCURRENT
            //INST
            IOCPT STRSTART
            ; Включение первой катушки пользователя
            STR 10010
            AND 70011
            OUT 70012
            ; Цепь задержки с таймером
            STR 70012
            TMR 1, 50
            END
        "#;

        let rungs = parse_lst_to_rungs(mock_lst);
        
        // По логике ПЛК Yaskawa, инструкции OUT и TMR завершают каждую цепь (Rung)
        assert_eq!(rungs.len(), 2);
        
        // Проверка первой цепи
        assert_eq!(rungs[0].id, 1);
        assert_eq!(rungs[0].comment, Some("Включение первой катушки пользователя".to_string()));
        assert_eq!(rungs[0].instructions.len(), 3);
        assert_eq!(rungs[0].instructions[2].inst_type, "OUT");
        assert_eq!(rungs[0].instructions[2].operand, "70012");

        // Проверка второй цепи
        assert_eq!(rungs[1].id, 2);
        assert_eq!(rungs[1].comment, Some("Цепь задержки с таймером".to_string()));
        assert_eq!(rungs[1].instructions.len(), 2);
        assert_eq!(rungs[1].instructions[1].inst_type, "TMR");
        assert_eq!(rungs[1].instructions[1].operand, "1,50");
    }

    #[test]
    fn test_compiler_validation_double_coil() {
        // Создаем фиктивный проект, где два разных Rung пытаются записать в одно реле 70020
        let mock_lst = r#"
            STR 10010
            OUT 70020
            STR 10011
            OUT 70020
        "#;

        let rungs = parse_lst_to_rungs(mock_lst);
        let errors = validate_rungs(&rungs);

        // Анализатор должен поймать ошибку DoubleCoil во второй цепи
        assert!(!errors.is_empty());
        assert_eq!(errors[0].error_type, "DoubleCoil");
        assert!(errors[0].message.contains("70020"));
    }

    #[test]
    fn test_compiler_validation_readonly_address() {
        // Попытка записать значение в аппаратные входы серии 1xxxx
        let mock_lst = r#"
            STR 70010
            OUT 10012
        "#;

        let rungs = parse_lst_to_rungs(mock_lst);
        let errors = validate_rungs(&rungs);

        assert!(!errors.is_empty());
        assert_eq!(errors[0].error_type, "ReadOnlyAddress");
    }

    #[test]
    fn test_compiler_validation_invalid_bit_index() {
        // Попытка указать индекс бита 8 (допустимо только 0-7)
        let mock_lst = r#"
            STR 70018
            OUT 70019
        "#;

        let rungs = parse_lst_to_rungs(mock_lst);
        let errors = validate_rungs(&rungs);

        // Должно быть обнаружено две ошибки для обеих инструкций
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].error_type, "InvalidBitIndex");
        assert_eq!(errors[1].error_type, "InvalidBitIndex");
    }

    #[test]
    fn test_readonly_external_input_and_writable_external_output() {
        let input = parse_lst_to_rungs("STR 70010\nOUT 20010\n");
        let errors = validate_rungs(&input);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, "ReadOnlyAddress");

        let output = parse_lst_to_rungs("STR 70010\nOUT 30010\n");
        assert!(validate_rungs(&output).is_empty());
    }

    #[test]
    fn test_profile_specific_address_validation() {
        use crate::compiler::memory::ControllerFamily;
        use crate::compiler::validate_rungs_for_controller;

        let ladder = parse_lst_to_rungs("STR 02570\nOUT 70010\n");
        let dx100_errors = validate_rungs_for_controller(&ladder, ControllerFamily::DX100);
        assert_eq!(dx100_errors.len(), 1);
        assert_eq!(dx100_errors[0].error_type, "UnknownAddress");
        assert!(validate_rungs_for_controller(&ladder, ControllerFamily::DX200).is_empty());
    }

    #[test]
    fn test_nc_contact_bit_index_and_duplicate_normalization() {
        use crate::{LadderInstruction, Rung};

        let rungs = vec![
            Rung { id: 1, comment: None, is_active: false, instructions: vec![
                LadderInstruction { id: "1".into(), inst_type: "STR-NOT".into(), operand: "#70018".into() },
                LadderInstruction { id: "2".into(), inst_type: "OUT".into(), operand: "#70020".into() },
            ] },
            Rung { id: 2, comment: None, is_active: false, instructions: vec![
                LadderInstruction { id: "3".into(), inst_type: "STR".into(), operand: "70011".into() },
                LadderInstruction { id: "4".into(), inst_type: "OUT".into(), operand: "70020".into() },
            ] },
        ];
        let errors = validate_rungs(&rungs);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].error_type, "InvalidBitIndex");
        assert_eq!(errors[1].error_type, "DoubleCoil");
    }
}
