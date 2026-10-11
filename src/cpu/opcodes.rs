#[derive(Debug)]
pub enum NibbleCode {
    Misc = 0x0,
    Jump = 0x1,
    Subroutines = 0x2,
    BranchEqual = 0x3,
    BranchNotEqual = 0x4,
    BranchRegisterEqual = 0x5,
    SetRegister = 0x6,
    AddToRegister = 0x7,
    Arithmetic = 0x8,
    BranchRegisterNotEqual = 0x9,
    SetIndexRegister = 0xA,
    JumpWithOffset = 0xB,
    Random = 0xC,
    Draw = 0xD,
    SkipKey = 0xE,
    Timer = 0xF,
}

pub enum ArithmeticCode {
    Set = 0x0,
    Or = 0x1,
    And = 0x2,
    Xor = 0x3,
    Add = 0x4,
    SubtractXy = 0x5,
    ShiftRight = 0x6,
    SubtractYx = 0x7,
    ShiftLeft = 0xE,
}

pub enum ManipCode {
    SetVx = 0x07,
    SetDelay = 0x15,
    SetSound = 0x18,
    AddToIndex = 0x1E,
    GetKey = 0x0A,
    FontCharacter = 0x29,
    BinaryDecimalConversion = 0x33,
    StoreMem = 0x55,
    LoadMem = 0x65,
}