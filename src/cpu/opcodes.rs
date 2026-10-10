pub enum NibbleCode {
    Misc = 0x0,
    Jump = 0x1,
    Skip3 = 0x3,
    Skip4 = 0x4,
    Skip5 = 0x5,
    SetRegister = 0x6,
    AddToRegister = 0x7,
    Arithmetic = 0x8,
    Skip9 = 0x9,
    SetIndexRegister = 0xA,
    JumpWithOffset = 0xB,
    Random = 0xC,
    Draw = 0xD,
    SkipKey = 0xE,
    Timer = 0xF,
}

pub enum MiscCodes {

}