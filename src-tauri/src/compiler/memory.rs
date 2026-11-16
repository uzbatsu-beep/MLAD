//! Typed Concurrent I/O address model.
//!
//! Source tables: DX100 RE-CKI-A453 §2, DX200 RE-CKI-A465 §2,
//! YRC1000 Concurrent I/O §2. The ranges identify *classes*, not a
//! guarantee of installed hardware or permission to transmit writes.
//! A controller's application, options and firmware must be checked separately.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerFamily {
    DX100,
    DX200,
    YRC1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelayClass {
    GeneralInput,
    GeneralOutput,
    ExternalInput,
    ExternalOutput,
    SpecificInput,
    SpecificOutput,
    InterfacePanelInput,
    AuxiliaryRelay,
    ControlStatus,
    PseudoInput,
    NetworkInput,
    NetworkOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelayRange {
    pub class: RelayClass,
    pub first: u32,
    pub last: u32,
}

const fn range(class: RelayClass, first: u32, last: u32) -> RelayRange {
    RelayRange { class, first, last }
}

pub const DX100_RANGES: &[RelayRange] = &[
    range(RelayClass::GeneralInput, 10, 2567),
    range(RelayClass::GeneralOutput, 10010, 12567),
    range(RelayClass::ExternalInput, 20010, 22567),
    range(RelayClass::ExternalOutput, 30010, 32567),
    range(RelayClass::SpecificInput, 40010, 41607),
    range(RelayClass::SpecificOutput, 50010, 52007),
    range(RelayClass::InterfacePanelInput, 60010, 60647),
    range(RelayClass::AuxiliaryRelay, 70010, 79997),
    range(RelayClass::ControlStatus, 80010, 80647),
    range(RelayClass::PseudoInput, 82010, 82207),
    range(RelayClass::NetworkInput, 25010, 27567),
    range(RelayClass::NetworkOutput, 35010, 37567),
];

pub const DX200_RANGES: &[RelayRange] = &[
    range(RelayClass::GeneralInput, 10, 5127),
    range(RelayClass::GeneralOutput, 10010, 15127),
    range(RelayClass::ExternalInput, 20010, 25127),
    range(RelayClass::ExternalOutput, 30010, 35127),
    range(RelayClass::SpecificInput, 40010, 41607),
    range(RelayClass::SpecificOutput, 50010, 53007),
    range(RelayClass::InterfacePanelInput, 60010, 60647),
    range(RelayClass::AuxiliaryRelay, 70010, 79997),
    range(RelayClass::ControlStatus, 80010, 82007),
    range(RelayClass::PseudoInput, 82010, 82207),
    range(RelayClass::NetworkInput, 27010, 29567),
    range(RelayClass::NetworkOutput, 37010, 39567),
];

pub const YRC1000_RANGES: &[RelayRange] = &[
    range(RelayClass::GeneralInput, 10, 5127),
    range(RelayClass::GeneralOutput, 10010, 15127),
    range(RelayClass::ExternalInput, 20010, 25127),
    range(RelayClass::ExternalOutput, 30010, 35127),
    range(RelayClass::SpecificInput, 40010, 42567),
    range(RelayClass::SpecificOutput, 50010, 55127),
    range(RelayClass::InterfacePanelInput, 60010, 60647),
    range(RelayClass::AuxiliaryRelay, 70010, 79997),
    range(RelayClass::ControlStatus, 80010, 85127),
    range(RelayClass::PseudoInput, 87010, 87207),
    range(RelayClass::NetworkInput, 27010, 29567),
    range(RelayClass::NetworkOutput, 37010, 39567),
];

impl ControllerFamily {
    pub const fn ranges(self) -> &'static [RelayRange] {
        match self {
            Self::DX100 => DX100_RANGES,
            Self::DX200 => DX200_RANGES,
            Self::YRC1000 => YRC1000_RANGES,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressError {
    MalformedRelay,
    InvalidBitIndex,
    UnknownRelayRange,
    MalformedRegister,
    InvalidGroupStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelayBitAddress {
    pub number: u32,
    pub channel: u16,
    pub bit: u8,
    pub class: RelayClass,
}

impl RelayBitAddress {
    pub fn display(self) -> String {
        format!("#{:05}", self.number)
    }
}

/// Classify an exact five-decimal-digit CIO bit address. The optional '#'
/// is accepted because the existing MLAD editor stored bare numbers.
/// Absence of a range is NOT evidence that a device lacks a physical signal.
pub fn parse_relay(controller: ControllerFamily, input: &str) -> Result<RelayBitAddress, AddressError> {
    let digits = input.trim().strip_prefix('#').unwrap_or(input.trim());
    if digits.len() != 5 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(AddressError::MalformedRelay);
    }
    let number = digits.parse::<u32>().map_err(|_| AddressError::MalformedRelay)?;
    let bit = (number % 10) as u8;
    if bit > 7 {
        return Err(AddressError::InvalidBitIndex);
    }
    let class = controller
        .ranges()
        .iter()
        .find(|r| number >= r.first && number <= r.last)
        .map(|r| r.class)
        .ok_or(AddressError::UnknownRelayRange)?;
    Ok(RelayBitAddress {
        number,
        channel: (number / 10) as u16,
        bit,
        class,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayGroupWidth { Byte8, Word16 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelayGroupAddress {
    pub first_bit: RelayBitAddress,
    pub width: RelayGroupWidth,
}

/// W#XXXXX denotes a word operand; #XXXXX in a group instruction denotes
/// a byte operand. Whether a word is aligned is instruction-specific and
/// deliberately NOT assumed by this model.
pub fn parse_group(
    controller: ControllerFamily,
    input: &str,
    width: RelayGroupWidth,
) -> Result<RelayGroupAddress, AddressError> {
    let trimmed = input.trim();
    let digits = match width {
        RelayGroupWidth::Byte8 => trimmed.strip_prefix('#'),
        RelayGroupWidth::Word16 => trimmed.strip_prefix("W#"),
    }.ok_or(AddressError::InvalidGroupStart)?;
    let first_bit = parse_relay(controller, digits)?;
    if first_bit.bit != 0 {
        return Err(AddressError::InvalidGroupStart);
    }
    Ok(RelayGroupAddress { first_bit, width })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterClass {
    General,      // M000..M559
    AnalogOutput, // M560..M599
    AnalogInput,  // M600..M639
    System,       // M640..M999
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterAddress {
    pub number: u16,
    pub class: RegisterClass,
}

pub fn parse_register(input: &str) -> Result<RegisterAddress, AddressError> {
    let trimmed = input.trim();
    let digits = trimmed.strip_prefix('M').ok_or(AddressError::MalformedRegister)?;
    if digits.len() != 3 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(AddressError::MalformedRegister);
    }
    let number = digits.parse::<u16>().map_err(|_| AddressError::MalformedRegister)?;
    let class = match number {
        0..=559 => RegisterClass::General,
        560..=599 => RegisterClass::AnalogOutput,
        600..=639 => RegisterClass::AnalogInput,
        _ => RegisterClass::System,
    };
    Ok(RegisterAddress { number, class })
}

/// This is *not* a live I/O authorization or an HSES write capability.
/// Validity is a separate question from whether an instruction may write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LadderWritePolicy {
    Denied,
    Candidate,       // subject to instruction, ownership and target validation
    RequiresReview,  // application/system-dependent; not auto-approved
}

pub fn relay_write_policy(class: RelayClass) -> LadderWritePolicy {
    use RelayClass::*;
    match class {
        GeneralInput | ExternalOutput | AuxiliaryRelay | NetworkOutput =>
            LadderWritePolicy::Candidate,
        SpecificInput => LadderWritePolicy::RequiresReview,
        GeneralOutput | ExternalInput | SpecificOutput | InterfacePanelInput
        | ControlStatus | PseudoInput | NetworkInput =>
            LadderWritePolicy::Denied,
    }
}

pub fn register_write_policy(class: RegisterClass) -> LadderWritePolicy {
    match class {
        RegisterClass::General => LadderWritePolicy::Candidate,
        RegisterClass::AnalogOutput => LadderWritePolicy::RequiresReview,
        RegisterClass::AnalogInput | RegisterClass::System => LadderWritePolicy::Denied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dx100_and_dx200_have_distinct_capacity() {
        assert_eq!(parse_relay(ControllerFamily::DX100, "#02567").unwrap().class, RelayClass::GeneralInput);
        assert_eq!(parse_relay(ControllerFamily::DX100, "#02570"), Err(AddressError::UnknownRelayRange));
        assert_eq!(parse_relay(ControllerFamily::DX200, "#02570").unwrap().class, RelayClass::GeneralInput);
        assert_eq!(parse_relay(ControllerFamily::DX100, "#22567").unwrap().class, RelayClass::ExternalInput);
        assert_eq!(parse_relay(ControllerFamily::DX100, "#22570"), Err(AddressError::UnknownRelayRange));
        assert_eq!(parse_relay(ControllerFamily::DX200, "#22570").unwrap().class, RelayClass::ExternalInput);
    }

    #[test]
    fn controller_pseudo_input_is_not_interchangeable() {
        for c in [ControllerFamily::DX100, ControllerFamily::DX200] {
            assert_eq!(parse_relay(c, "#82010").unwrap().class, RelayClass::PseudoInput);
            assert_eq!(parse_relay(c, "#87010"), Err(AddressError::UnknownRelayRange));
        }
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#87010").unwrap().class, RelayClass::PseudoInput);
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#82010").unwrap().class, RelayClass::ControlStatus);
    }

    #[test]
    fn network_ranges_are_profile_specific() {
        assert_eq!(parse_relay(ControllerFamily::DX100, "#25010").unwrap().class, RelayClass::NetworkInput);
        assert_eq!(parse_relay(ControllerFamily::DX200, "#25010").unwrap().class, RelayClass::ExternalInput);
        assert_eq!(parse_relay(ControllerFamily::DX200, "#27010").unwrap().class, RelayClass::NetworkInput);
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#37010").unwrap().class, RelayClass::NetworkOutput);
        assert_eq!(parse_relay(ControllerFamily::DX100, "#35010").unwrap().class, RelayClass::NetworkOutput);
    }

    #[test]
    fn bit_format_is_strict() {
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#70018"), Err(AddressError::InvalidBitIndex));
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#70019"), Err(AddressError::InvalidBitIndex));
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#7001"), Err(AddressError::MalformedRelay));
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#70010extra"), Err(AddressError::MalformedRelay));
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#99990"), Err(AddressError::UnknownRelayRange));
        let a = parse_relay(ControllerFamily::YRC1000, "70010").unwrap();
        assert_eq!((a.channel, a.bit, a.display()), (7001, 0, "#70010".to_string()));
    }

    #[test]
    fn exact_range_bounds_and_gaps_are_checked() {
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#79997").unwrap().class, RelayClass::AuxiliaryRelay);
        assert_eq!(parse_relay(ControllerFamily::YRC1000, "#80010").unwrap().class, RelayClass::ControlStatus);
        assert_eq!(parse_relay(ControllerFamily::DX200, "#82007").unwrap().class, RelayClass::ControlStatus);
        assert_eq!(parse_relay(ControllerFamily::DX200, "#82010").unwrap().class, RelayClass::PseudoInput);
        assert_eq!(parse_relay(ControllerFamily::DX200, "#82020").unwrap().class, RelayClass::PseudoInput);
        assert_eq!(parse_relay(ControllerFamily::DX200, "#82210"), Err(AddressError::UnknownRelayRange));
    }

    #[test]
    fn groups_have_distinct_types_and_start_at_bit_zero() {
        let byte = parse_group(ControllerFamily::DX200, "#70010", RelayGroupWidth::Byte8).unwrap();
        assert_eq!((byte.first_bit.bit, byte.width), (0, RelayGroupWidth::Byte8));
        let word = parse_group(ControllerFamily::DX200, "W#70010", RelayGroupWidth::Word16).unwrap();
        assert_eq!(word.width, RelayGroupWidth::Word16);
        assert_eq!(parse_group(ControllerFamily::DX200, "#70011", RelayGroupWidth::Byte8), Err(AddressError::InvalidGroupStart));
        assert_eq!(parse_group(ControllerFamily::DX200, "#70010", RelayGroupWidth::Word16), Err(AddressError::InvalidGroupStart));
    }

    #[test]
    fn register_categories_and_formats() {
        assert_eq!(parse_register("M000").unwrap().class, RegisterClass::General);
        assert_eq!(parse_register("M559").unwrap().class, RegisterClass::General);
        assert_eq!(parse_register("M560").unwrap().class, RegisterClass::AnalogOutput);
        assert_eq!(parse_register("M599").unwrap().class, RegisterClass::AnalogOutput);
        assert_eq!(parse_register("M600").unwrap().class, RegisterClass::AnalogInput);
        assert_eq!(parse_register("M639").unwrap().class, RegisterClass::AnalogInput);
        assert_eq!(parse_register("M640").unwrap().class, RegisterClass::System);
        assert_eq!(parse_register("M999").unwrap().class, RegisterClass::System);
        assert_eq!(parse_register("M1000"), Err(AddressError::MalformedRegister));
        assert_eq!(parse_register("M1"), Err(AddressError::MalformedRegister));
    }

    #[test]
    fn write_policy_is_conservative() {
        assert_eq!(relay_write_policy(RelayClass::ExternalInput), LadderWritePolicy::Denied);
        assert_eq!(relay_write_policy(RelayClass::ExternalOutput), LadderWritePolicy::Candidate);
        assert_eq!(relay_write_policy(RelayClass::SpecificInput), LadderWritePolicy::RequiresReview);
        assert_eq!(relay_write_policy(RelayClass::ControlStatus), LadderWritePolicy::Denied);
        assert_eq!(register_write_policy(RegisterClass::System), LadderWritePolicy::Denied);
    }
}
