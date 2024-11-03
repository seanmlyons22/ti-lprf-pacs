#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "0:0\\]
Data read-back format. Data is always stored in binary unsigned format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Df {
    #[doc = "1: Digital result reads Signed Binary. (2s complement), left aligned."]
    Signed = 1,
    #[doc = "0: Digital result reads as Binary Unsigned."]
    Unsigned = 0,
}
impl From<Df> for bool {
    #[inline(always)]
    fn from(variant: Df) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DF` reader - 0:0\\]
Data read-back format. Data is always stored in binary unsigned format."]
pub type DfR = crate::BitReader<Df>;
impl DfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Df {
        match self.bits {
            true => Df::Signed,
            false => Df::Unsigned,
        }
    }
    #[doc = "Digital result reads Signed Binary. (2s complement), left aligned."]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == Df::Signed
    }
    #[doc = "Digital result reads as Binary Unsigned."]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == Df::Unsigned
    }
}
#[doc = "Field `DF` writer - 0:0\\]
Data read-back format. Data is always stored in binary unsigned format."]
pub type DfW<'a, REG> = crate::BitWriter<'a, REG, Df>;
impl<'a, REG> DfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital result reads Signed Binary. (2s complement), left aligned."]
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(Df::Signed)
    }
    #[doc = "Digital result reads as Binary Unsigned."]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(Df::Unsigned)
    }
}
#[doc = "2:1\\]
Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Res {
    #[doc = "2: 8-bits resolution"]
    Bit8 = 2,
    #[doc = "1: 10-bits resolution"]
    Bit10 = 1,
    #[doc = "0: 12-bits resolution"]
    Bit12 = 0,
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(variant: Res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Res {
    type Ux = u8;
}
impl crate::IsEnum for Res {}
#[doc = "Field `RES` reader - 2:1\\]
Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
pub type ResR = crate::FieldReader<Res>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Res> {
        match self.bits {
            2 => Some(Res::Bit8),
            1 => Some(Res::Bit10),
            0 => Some(Res::Bit12),
            _ => None,
        }
    }
    #[doc = "8-bits resolution"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == Res::Bit8
    }
    #[doc = "10-bits resolution"]
    #[inline(always)]
    pub fn is_bit_10(&self) -> bool {
        *self == Res::Bit10
    }
    #[doc = "12-bits resolution"]
    #[inline(always)]
    pub fn is_bit_12(&self) -> bool {
        *self == Res::Bit12
    }
}
#[doc = "Field `RES` writer - 2:1\\]
Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, Res>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bits resolution"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bit8)
    }
    #[doc = "10-bits resolution"]
    #[inline(always)]
    pub fn bit_10(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bit10)
    }
    #[doc = "12-bits resolution"]
    #[inline(always)]
    pub fn bit_12(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bit12)
    }
}
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "8:8\\]
Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "1: DMA trigger enabled"]
    Enable = 1,
    #[doc = "0: DMA trigger not enabled"]
    Disable = 0,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - 8:8\\]
Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            true => Dmaen::Enable,
            false => Dmaen::Disable,
        }
    }
    #[doc = "DMA trigger enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaen::Enable
    }
    #[doc = "DMA trigger not enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaen::Disable
    }
}
#[doc = "Field `DMAEN` writer - 8:8\\]
Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA trigger enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enable)
    }
    #[doc = "DMA trigger not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disable)
    }
}
#[doc = "Field `RESERVED9` reader - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `RESERVED9` writer - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "10:10\\]
Enable FIFO based operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoen {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` reader - 10:10\\]
Enable FIFO based operation"]
pub type FifoenR = crate::BitReader<Fifoen>;
impl FifoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoen {
        match self.bits {
            true => Fifoen::Enable,
            false => Fifoen::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fifoen::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fifoen::Disable
    }
}
#[doc = "Field `FIFOEN` writer - 10:10\\]
Enable FIFO based operation"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG, Fifoen>;
impl<'a, REG> FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::Disable)
    }
}
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "20:16\\]
Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startadd {
    #[doc = "23: MEMCTL23 is selected as start address of a sequence or for a single conversion."]
    Addr23 = 23,
    #[doc = "22: MEMCTL22 is selected as start address of a sequence or for a single conversion."]
    Addr22 = 22,
    #[doc = "21: MEMCTL21 is selected as start address of a sequence or for a single conversion."]
    Addr21 = 21,
    #[doc = "20: MEMCTL20 is selected as start address of a sequence or for a single conversion."]
    Addr20 = 20,
    #[doc = "19: MEMCTL19 is selected as start address of a sequence or for a single conversion."]
    Addr19 = 19,
    #[doc = "18: MEMCTL18 is selected as start address of a sequence or for a single conversion."]
    Addr18 = 18,
    #[doc = "17: MEMCTL17 is selected as start address of a sequence or for a single conversion."]
    Addr17 = 17,
    #[doc = "16: MEMCTL16 is selected as start address of a sequence or for a single conversion."]
    Addr16 = 16,
    #[doc = "15: MEMCTL15 is selected as start address of a sequence or for a single conversion."]
    Addr15 = 15,
    #[doc = "14: MEMCTL14 is selected as start address of a sequence or for a single conversion."]
    Addr14 = 14,
    #[doc = "13: MEMCTL13 is selected as start address of a sequence or for a single conversion."]
    Addr13 = 13,
    #[doc = "12: MEMCTL12 is selected as start address of a sequence or for a single conversion."]
    Addr12 = 12,
    #[doc = "11: MEMCTL11 is selected as start address of a sequence or for a single conversion."]
    Addr11 = 11,
    #[doc = "10: MEMCTL10 is selected as start address of a sequence or for a single conversion."]
    Addr10 = 10,
    #[doc = "9: MEMCTL9 is selected as start address of a sequence or for a single conversion."]
    Addr09 = 9,
    #[doc = "8: MEMCTL8 is selected as start address of a sequence or for a single conversion."]
    Addr08 = 8,
    #[doc = "7: MEMCTL7 is selected as start address of a sequence or for a single conversion."]
    Addr07 = 7,
    #[doc = "6: MEMCTL6 is selected as start address of a sequence or for a single conversion."]
    Addr06 = 6,
    #[doc = "5: MEMCTL5 is selected as start address of a sequence or for a single conversion."]
    Addr05 = 5,
    #[doc = "4: MEMCTL4 is selected as start address of a sequence or for a single conversion."]
    Addr04 = 4,
    #[doc = "3: MEMCTL3 is selected as start address of a sequence or for a single conversion."]
    Addr03 = 3,
    #[doc = "2: MEMCTL2 is selected as start address of a sequence or for a single conversion."]
    Addr02 = 2,
    #[doc = "1: MEMCTL1 is selected as start address of a sequence or for a single conversion."]
    Addr01 = 1,
    #[doc = "0: MEMCTL0 is selected as start address of a sequence or for a single conversion."]
    Addr00 = 0,
}
impl From<Startadd> for u8 {
    #[inline(always)]
    fn from(variant: Startadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startadd {
    type Ux = u8;
}
impl crate::IsEnum for Startadd {}
#[doc = "Field `STARTADD` reader - 20:16\\]
Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type StartaddR = crate::FieldReader<Startadd>;
impl StartaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startadd> {
        match self.bits {
            23 => Some(Startadd::Addr23),
            22 => Some(Startadd::Addr22),
            21 => Some(Startadd::Addr21),
            20 => Some(Startadd::Addr20),
            19 => Some(Startadd::Addr19),
            18 => Some(Startadd::Addr18),
            17 => Some(Startadd::Addr17),
            16 => Some(Startadd::Addr16),
            15 => Some(Startadd::Addr15),
            14 => Some(Startadd::Addr14),
            13 => Some(Startadd::Addr13),
            12 => Some(Startadd::Addr12),
            11 => Some(Startadd::Addr11),
            10 => Some(Startadd::Addr10),
            9 => Some(Startadd::Addr09),
            8 => Some(Startadd::Addr08),
            7 => Some(Startadd::Addr07),
            6 => Some(Startadd::Addr06),
            5 => Some(Startadd::Addr05),
            4 => Some(Startadd::Addr04),
            3 => Some(Startadd::Addr03),
            2 => Some(Startadd::Addr02),
            1 => Some(Startadd::Addr01),
            0 => Some(Startadd::Addr00),
            _ => None,
        }
    }
    #[doc = "MEMCTL23 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_23(&self) -> bool {
        *self == Startadd::Addr23
    }
    #[doc = "MEMCTL22 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_22(&self) -> bool {
        *self == Startadd::Addr22
    }
    #[doc = "MEMCTL21 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_21(&self) -> bool {
        *self == Startadd::Addr21
    }
    #[doc = "MEMCTL20 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_20(&self) -> bool {
        *self == Startadd::Addr20
    }
    #[doc = "MEMCTL19 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_19(&self) -> bool {
        *self == Startadd::Addr19
    }
    #[doc = "MEMCTL18 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_18(&self) -> bool {
        *self == Startadd::Addr18
    }
    #[doc = "MEMCTL17 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_17(&self) -> bool {
        *self == Startadd::Addr17
    }
    #[doc = "MEMCTL16 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_16(&self) -> bool {
        *self == Startadd::Addr16
    }
    #[doc = "MEMCTL15 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_15(&self) -> bool {
        *self == Startadd::Addr15
    }
    #[doc = "MEMCTL14 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_14(&self) -> bool {
        *self == Startadd::Addr14
    }
    #[doc = "MEMCTL13 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_13(&self) -> bool {
        *self == Startadd::Addr13
    }
    #[doc = "MEMCTL12 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_12(&self) -> bool {
        *self == Startadd::Addr12
    }
    #[doc = "MEMCTL11 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_11(&self) -> bool {
        *self == Startadd::Addr11
    }
    #[doc = "MEMCTL10 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_10(&self) -> bool {
        *self == Startadd::Addr10
    }
    #[doc = "MEMCTL9 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_09(&self) -> bool {
        *self == Startadd::Addr09
    }
    #[doc = "MEMCTL8 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_08(&self) -> bool {
        *self == Startadd::Addr08
    }
    #[doc = "MEMCTL7 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_07(&self) -> bool {
        *self == Startadd::Addr07
    }
    #[doc = "MEMCTL6 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_06(&self) -> bool {
        *self == Startadd::Addr06
    }
    #[doc = "MEMCTL5 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_05(&self) -> bool {
        *self == Startadd::Addr05
    }
    #[doc = "MEMCTL4 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_04(&self) -> bool {
        *self == Startadd::Addr04
    }
    #[doc = "MEMCTL3 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_03(&self) -> bool {
        *self == Startadd::Addr03
    }
    #[doc = "MEMCTL2 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_02(&self) -> bool {
        *self == Startadd::Addr02
    }
    #[doc = "MEMCTL1 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_01(&self) -> bool {
        *self == Startadd::Addr01
    }
    #[doc = "MEMCTL0 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_00(&self) -> bool {
        *self == Startadd::Addr00
    }
}
#[doc = "Field `STARTADD` writer - 20:16\\]
Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type StartaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Startadd>;
impl<'a, REG> StartaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MEMCTL23 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_23(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr23)
    }
    #[doc = "MEMCTL22 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_22(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr22)
    }
    #[doc = "MEMCTL21 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_21(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr21)
    }
    #[doc = "MEMCTL20 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_20(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr20)
    }
    #[doc = "MEMCTL19 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_19(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr19)
    }
    #[doc = "MEMCTL18 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_18(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr18)
    }
    #[doc = "MEMCTL17 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_17(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr17)
    }
    #[doc = "MEMCTL16 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr16)
    }
    #[doc = "MEMCTL15 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_15(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr15)
    }
    #[doc = "MEMCTL14 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_14(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr14)
    }
    #[doc = "MEMCTL13 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_13(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr13)
    }
    #[doc = "MEMCTL12 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr12)
    }
    #[doc = "MEMCTL11 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_11(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr11)
    }
    #[doc = "MEMCTL10 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr10)
    }
    #[doc = "MEMCTL9 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_09(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr09)
    }
    #[doc = "MEMCTL8 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_08(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr08)
    }
    #[doc = "MEMCTL7 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_07(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr07)
    }
    #[doc = "MEMCTL6 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_06(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr06)
    }
    #[doc = "MEMCTL5 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_05(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr05)
    }
    #[doc = "MEMCTL4 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_04(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr04)
    }
    #[doc = "MEMCTL3 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_03(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr03)
    }
    #[doc = "MEMCTL2 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_02(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr02)
    }
    #[doc = "MEMCTL1 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_01(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr01)
    }
    #[doc = "MEMCTL0 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_00(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr00)
    }
}
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "28:24\\]
Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Endadd {
    #[doc = "23: MEMCTL23 is selected as end address of sequence."]
    Addr23 = 23,
    #[doc = "22: MEMCTL22 is selected as end address of sequence."]
    Addr22 = 22,
    #[doc = "21: MEMCTL21 is selected as end address of sequence."]
    Addr21 = 21,
    #[doc = "20: MEMCTL20 is selected as end address of sequence."]
    Addr20 = 20,
    #[doc = "19: MEMCTL19 is selected as end address of sequence."]
    Addr19 = 19,
    #[doc = "18: MEMCTL18 is selected as end address of sequence."]
    Addr18 = 18,
    #[doc = "17: MEMCTL17 is selected as end address of sequence."]
    Addr17 = 17,
    #[doc = "16: MEMCTL16 is selected as end address of sequence."]
    Addr16 = 16,
    #[doc = "15: MEMCTL15 is selected as end address of sequence."]
    Addr15 = 15,
    #[doc = "14: MEMCTL14 is selected as end address of sequence."]
    Addr14 = 14,
    #[doc = "13: MEMCTL13 is selected as end address of sequence."]
    Addr13 = 13,
    #[doc = "12: MEMCTL12 is selected as end address of sequence."]
    Addr12 = 12,
    #[doc = "11: MEMCTL11 is selected as end address of sequence."]
    Addr11 = 11,
    #[doc = "10: MEMCTL10 is selected as end address of sequence."]
    Addr10 = 10,
    #[doc = "9: MEMCTL9 is selected as end address of sequence."]
    Addr09 = 9,
    #[doc = "8: MEMCTL8 is selected as end address of sequence."]
    Addr08 = 8,
    #[doc = "7: MEMCTL7 is selected as end address of sequence."]
    Addr07 = 7,
    #[doc = "6: MEMCTL6 is selected as end address of sequence."]
    Addr06 = 6,
    #[doc = "5: MEMCTL5 is selected as end address of sequence."]
    Addr05 = 5,
    #[doc = "4: MEMCTL4 is selected as end address of sequence."]
    Addr04 = 4,
    #[doc = "3: MEMCTL3 is selected as end address of sequence."]
    Addr03 = 3,
    #[doc = "2: MEMCTL2 is selected as end address of sequence."]
    Addr02 = 2,
    #[doc = "1: MEMCTL1 is selected as end address of sequence."]
    Addr01 = 1,
    #[doc = "0: MEMCTL0 is selected as end address of sequence."]
    Addr00 = 0,
}
impl From<Endadd> for u8 {
    #[inline(always)]
    fn from(variant: Endadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Endadd {
    type Ux = u8;
}
impl crate::IsEnum for Endadd {}
#[doc = "Field `ENDADD` reader - 28:24\\]
Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type EndaddR = crate::FieldReader<Endadd>;
impl EndaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Endadd> {
        match self.bits {
            23 => Some(Endadd::Addr23),
            22 => Some(Endadd::Addr22),
            21 => Some(Endadd::Addr21),
            20 => Some(Endadd::Addr20),
            19 => Some(Endadd::Addr19),
            18 => Some(Endadd::Addr18),
            17 => Some(Endadd::Addr17),
            16 => Some(Endadd::Addr16),
            15 => Some(Endadd::Addr15),
            14 => Some(Endadd::Addr14),
            13 => Some(Endadd::Addr13),
            12 => Some(Endadd::Addr12),
            11 => Some(Endadd::Addr11),
            10 => Some(Endadd::Addr10),
            9 => Some(Endadd::Addr09),
            8 => Some(Endadd::Addr08),
            7 => Some(Endadd::Addr07),
            6 => Some(Endadd::Addr06),
            5 => Some(Endadd::Addr05),
            4 => Some(Endadd::Addr04),
            3 => Some(Endadd::Addr03),
            2 => Some(Endadd::Addr02),
            1 => Some(Endadd::Addr01),
            0 => Some(Endadd::Addr00),
            _ => None,
        }
    }
    #[doc = "MEMCTL23 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_23(&self) -> bool {
        *self == Endadd::Addr23
    }
    #[doc = "MEMCTL22 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_22(&self) -> bool {
        *self == Endadd::Addr22
    }
    #[doc = "MEMCTL21 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_21(&self) -> bool {
        *self == Endadd::Addr21
    }
    #[doc = "MEMCTL20 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_20(&self) -> bool {
        *self == Endadd::Addr20
    }
    #[doc = "MEMCTL19 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_19(&self) -> bool {
        *self == Endadd::Addr19
    }
    #[doc = "MEMCTL18 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_18(&self) -> bool {
        *self == Endadd::Addr18
    }
    #[doc = "MEMCTL17 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_17(&self) -> bool {
        *self == Endadd::Addr17
    }
    #[doc = "MEMCTL16 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_16(&self) -> bool {
        *self == Endadd::Addr16
    }
    #[doc = "MEMCTL15 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_15(&self) -> bool {
        *self == Endadd::Addr15
    }
    #[doc = "MEMCTL14 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_14(&self) -> bool {
        *self == Endadd::Addr14
    }
    #[doc = "MEMCTL13 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_13(&self) -> bool {
        *self == Endadd::Addr13
    }
    #[doc = "MEMCTL12 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_12(&self) -> bool {
        *self == Endadd::Addr12
    }
    #[doc = "MEMCTL11 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_11(&self) -> bool {
        *self == Endadd::Addr11
    }
    #[doc = "MEMCTL10 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_10(&self) -> bool {
        *self == Endadd::Addr10
    }
    #[doc = "MEMCTL9 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_09(&self) -> bool {
        *self == Endadd::Addr09
    }
    #[doc = "MEMCTL8 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_08(&self) -> bool {
        *self == Endadd::Addr08
    }
    #[doc = "MEMCTL7 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_07(&self) -> bool {
        *self == Endadd::Addr07
    }
    #[doc = "MEMCTL6 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_06(&self) -> bool {
        *self == Endadd::Addr06
    }
    #[doc = "MEMCTL5 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_05(&self) -> bool {
        *self == Endadd::Addr05
    }
    #[doc = "MEMCTL4 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_04(&self) -> bool {
        *self == Endadd::Addr04
    }
    #[doc = "MEMCTL3 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_03(&self) -> bool {
        *self == Endadd::Addr03
    }
    #[doc = "MEMCTL2 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_02(&self) -> bool {
        *self == Endadd::Addr02
    }
    #[doc = "MEMCTL1 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_01(&self) -> bool {
        *self == Endadd::Addr01
    }
    #[doc = "MEMCTL0 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_00(&self) -> bool {
        *self == Endadd::Addr00
    }
}
#[doc = "Field `ENDADD` writer - 28:24\\]
Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type EndaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Endadd>;
impl<'a, REG> EndaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MEMCTL23 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_23(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr23)
    }
    #[doc = "MEMCTL22 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_22(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr22)
    }
    #[doc = "MEMCTL21 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_21(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr21)
    }
    #[doc = "MEMCTL20 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_20(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr20)
    }
    #[doc = "MEMCTL19 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_19(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr19)
    }
    #[doc = "MEMCTL18 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_18(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr18)
    }
    #[doc = "MEMCTL17 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_17(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr17)
    }
    #[doc = "MEMCTL16 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr16)
    }
    #[doc = "MEMCTL15 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_15(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr15)
    }
    #[doc = "MEMCTL14 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_14(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr14)
    }
    #[doc = "MEMCTL13 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_13(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr13)
    }
    #[doc = "MEMCTL12 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr12)
    }
    #[doc = "MEMCTL11 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_11(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr11)
    }
    #[doc = "MEMCTL10 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr10)
    }
    #[doc = "MEMCTL9 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_09(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr09)
    }
    #[doc = "MEMCTL8 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_08(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr08)
    }
    #[doc = "MEMCTL7 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_07(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr07)
    }
    #[doc = "MEMCTL6 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_06(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr06)
    }
    #[doc = "MEMCTL5 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_05(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr05)
    }
    #[doc = "MEMCTL4 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_04(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr04)
    }
    #[doc = "MEMCTL3 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_03(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr03)
    }
    #[doc = "MEMCTL2 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_02(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr02)
    }
    #[doc = "MEMCTL1 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_01(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr01)
    }
    #[doc = "MEMCTL0 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_00(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr00)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data read-back format. Data is always stored in binary unsigned format."]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable FIFO based operation"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn startadd(&self) -> StartaddR {
        StartaddR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn endadd(&self) -> EndaddR {
        EndaddR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data read-back format. Data is always stored in binary unsigned format."]
    #[inline(always)]
    #[must_use]
    pub fn df(&mut self) -> DfW<Ctl2Spec> {
        DfW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Ctl2Spec> {
        ResW::new(self, 1)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Ctl2Spec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<Ctl2Spec> {
        DmaenW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Ctl2Spec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable FIFO based operation"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FifoenW<Ctl2Spec> {
        FifoenW::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<Ctl2Spec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    #[must_use]
    pub fn startadd(&mut self) -> StartaddW<Ctl2Spec> {
        StartaddW::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<Ctl2Spec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    #[must_use]
    pub fn endadd(&mut self) -> EndaddW<Ctl2Spec> {
        EndaddW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<Ctl2Spec> {
        Reserved29W::new(self, 29)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0;
}
