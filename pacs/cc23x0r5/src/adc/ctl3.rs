#[doc = "Register `CTL3` reader"]
pub type R = crate::R<Ctl3Spec>;
#[doc = "Register `CTL3` writer"]
pub type W = crate::W<Ctl3Spec>;
#[doc = "4:0\\]
ASC channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ascchsel {
    #[doc = "31: Selects channel 31"]
    Chan31 = 31,
    #[doc = "30: Selects channel 30"]
    Chan30 = 30,
    #[doc = "29: Selects channel 29"]
    Chan29 = 29,
    #[doc = "28: Selects channel 28"]
    Chan28 = 28,
    #[doc = "27: Selects channel 27"]
    Chan27 = 27,
    #[doc = "26: Selects channel 26"]
    Chan26 = 26,
    #[doc = "25: Selects channel 25"]
    Chan25 = 25,
    #[doc = "24: Selects channel 24"]
    Chan24 = 24,
    #[doc = "23: Selects channel 23"]
    Chan23 = 23,
    #[doc = "22: Selects channel 22"]
    Chan22 = 22,
    #[doc = "21: Selects channel 21"]
    Chan21 = 21,
    #[doc = "20: Selects channel 20"]
    Chan20 = 20,
    #[doc = "19: Selects channel 19"]
    Chan19 = 19,
    #[doc = "18: Selects channel 18"]
    Chan18 = 18,
    #[doc = "17: Selects channel 17"]
    Chan17 = 17,
    #[doc = "16: Selects channel 16"]
    Chan16 = 16,
    #[doc = "15: Selects channel 15"]
    Chan15 = 15,
    #[doc = "14: Selects channel 14"]
    Chan14 = 14,
    #[doc = "13: Selects channel 13"]
    Chan13 = 13,
    #[doc = "12: Selects channel 12"]
    Chan12 = 12,
    #[doc = "11: Selects channel 11"]
    Chan11 = 11,
    #[doc = "10: Selects channel 10"]
    Chan10 = 10,
    #[doc = "9: Selects channel 9"]
    Chan9 = 9,
    #[doc = "8: Selects channel 8"]
    Chan8 = 8,
    #[doc = "7: Selects channel 7"]
    Chan7 = 7,
    #[doc = "6: Selects channel 6"]
    Chan6 = 6,
    #[doc = "5: Selects channel 5"]
    Chan5 = 5,
    #[doc = "4: Selects channel 4"]
    Chan4 = 4,
    #[doc = "3: Selects channel 3"]
    Chan3 = 3,
    #[doc = "2: Selects channel 2"]
    Chan2 = 2,
    #[doc = "1: Selects channel 1"]
    Chan1 = 1,
    #[doc = "0: Selects channel 0"]
    Chan0 = 0,
}
impl From<Ascchsel> for u8 {
    #[inline(always)]
    fn from(variant: Ascchsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ascchsel {
    type Ux = u8;
}
impl crate::IsEnum for Ascchsel {}
#[doc = "Field `ASCCHSEL` reader - 4:0\\]
ASC channel select"]
pub type AscchselR = crate::FieldReader<Ascchsel>;
impl AscchselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascchsel {
        match self.bits {
            31 => Ascchsel::Chan31,
            30 => Ascchsel::Chan30,
            29 => Ascchsel::Chan29,
            28 => Ascchsel::Chan28,
            27 => Ascchsel::Chan27,
            26 => Ascchsel::Chan26,
            25 => Ascchsel::Chan25,
            24 => Ascchsel::Chan24,
            23 => Ascchsel::Chan23,
            22 => Ascchsel::Chan22,
            21 => Ascchsel::Chan21,
            20 => Ascchsel::Chan20,
            19 => Ascchsel::Chan19,
            18 => Ascchsel::Chan18,
            17 => Ascchsel::Chan17,
            16 => Ascchsel::Chan16,
            15 => Ascchsel::Chan15,
            14 => Ascchsel::Chan14,
            13 => Ascchsel::Chan13,
            12 => Ascchsel::Chan12,
            11 => Ascchsel::Chan11,
            10 => Ascchsel::Chan10,
            9 => Ascchsel::Chan9,
            8 => Ascchsel::Chan8,
            7 => Ascchsel::Chan7,
            6 => Ascchsel::Chan6,
            5 => Ascchsel::Chan5,
            4 => Ascchsel::Chan4,
            3 => Ascchsel::Chan3,
            2 => Ascchsel::Chan2,
            1 => Ascchsel::Chan1,
            0 => Ascchsel::Chan0,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects channel 31"]
    #[inline(always)]
    pub fn is_chan_31(&self) -> bool {
        *self == Ascchsel::Chan31
    }
    #[doc = "Selects channel 30"]
    #[inline(always)]
    pub fn is_chan_30(&self) -> bool {
        *self == Ascchsel::Chan30
    }
    #[doc = "Selects channel 29"]
    #[inline(always)]
    pub fn is_chan_29(&self) -> bool {
        *self == Ascchsel::Chan29
    }
    #[doc = "Selects channel 28"]
    #[inline(always)]
    pub fn is_chan_28(&self) -> bool {
        *self == Ascchsel::Chan28
    }
    #[doc = "Selects channel 27"]
    #[inline(always)]
    pub fn is_chan_27(&self) -> bool {
        *self == Ascchsel::Chan27
    }
    #[doc = "Selects channel 26"]
    #[inline(always)]
    pub fn is_chan_26(&self) -> bool {
        *self == Ascchsel::Chan26
    }
    #[doc = "Selects channel 25"]
    #[inline(always)]
    pub fn is_chan_25(&self) -> bool {
        *self == Ascchsel::Chan25
    }
    #[doc = "Selects channel 24"]
    #[inline(always)]
    pub fn is_chan_24(&self) -> bool {
        *self == Ascchsel::Chan24
    }
    #[doc = "Selects channel 23"]
    #[inline(always)]
    pub fn is_chan_23(&self) -> bool {
        *self == Ascchsel::Chan23
    }
    #[doc = "Selects channel 22"]
    #[inline(always)]
    pub fn is_chan_22(&self) -> bool {
        *self == Ascchsel::Chan22
    }
    #[doc = "Selects channel 21"]
    #[inline(always)]
    pub fn is_chan_21(&self) -> bool {
        *self == Ascchsel::Chan21
    }
    #[doc = "Selects channel 20"]
    #[inline(always)]
    pub fn is_chan_20(&self) -> bool {
        *self == Ascchsel::Chan20
    }
    #[doc = "Selects channel 19"]
    #[inline(always)]
    pub fn is_chan_19(&self) -> bool {
        *self == Ascchsel::Chan19
    }
    #[doc = "Selects channel 18"]
    #[inline(always)]
    pub fn is_chan_18(&self) -> bool {
        *self == Ascchsel::Chan18
    }
    #[doc = "Selects channel 17"]
    #[inline(always)]
    pub fn is_chan_17(&self) -> bool {
        *self == Ascchsel::Chan17
    }
    #[doc = "Selects channel 16"]
    #[inline(always)]
    pub fn is_chan_16(&self) -> bool {
        *self == Ascchsel::Chan16
    }
    #[doc = "Selects channel 15"]
    #[inline(always)]
    pub fn is_chan_15(&self) -> bool {
        *self == Ascchsel::Chan15
    }
    #[doc = "Selects channel 14"]
    #[inline(always)]
    pub fn is_chan_14(&self) -> bool {
        *self == Ascchsel::Chan14
    }
    #[doc = "Selects channel 13"]
    #[inline(always)]
    pub fn is_chan_13(&self) -> bool {
        *self == Ascchsel::Chan13
    }
    #[doc = "Selects channel 12"]
    #[inline(always)]
    pub fn is_chan_12(&self) -> bool {
        *self == Ascchsel::Chan12
    }
    #[doc = "Selects channel 11"]
    #[inline(always)]
    pub fn is_chan_11(&self) -> bool {
        *self == Ascchsel::Chan11
    }
    #[doc = "Selects channel 10"]
    #[inline(always)]
    pub fn is_chan_10(&self) -> bool {
        *self == Ascchsel::Chan10
    }
    #[doc = "Selects channel 9"]
    #[inline(always)]
    pub fn is_chan_9(&self) -> bool {
        *self == Ascchsel::Chan9
    }
    #[doc = "Selects channel 8"]
    #[inline(always)]
    pub fn is_chan_8(&self) -> bool {
        *self == Ascchsel::Chan8
    }
    #[doc = "Selects channel 7"]
    #[inline(always)]
    pub fn is_chan_7(&self) -> bool {
        *self == Ascchsel::Chan7
    }
    #[doc = "Selects channel 6"]
    #[inline(always)]
    pub fn is_chan_6(&self) -> bool {
        *self == Ascchsel::Chan6
    }
    #[doc = "Selects channel 5"]
    #[inline(always)]
    pub fn is_chan_5(&self) -> bool {
        *self == Ascchsel::Chan5
    }
    #[doc = "Selects channel 4"]
    #[inline(always)]
    pub fn is_chan_4(&self) -> bool {
        *self == Ascchsel::Chan4
    }
    #[doc = "Selects channel 3"]
    #[inline(always)]
    pub fn is_chan_3(&self) -> bool {
        *self == Ascchsel::Chan3
    }
    #[doc = "Selects channel 2"]
    #[inline(always)]
    pub fn is_chan_2(&self) -> bool {
        *self == Ascchsel::Chan2
    }
    #[doc = "Selects channel 1"]
    #[inline(always)]
    pub fn is_chan_1(&self) -> bool {
        *self == Ascchsel::Chan1
    }
    #[doc = "Selects channel 0"]
    #[inline(always)]
    pub fn is_chan_0(&self) -> bool {
        *self == Ascchsel::Chan0
    }
}
#[doc = "Field `ASCCHSEL` writer - 4:0\\]
ASC channel select"]
pub type AscchselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ascchsel, crate::Safe>;
impl<'a, REG> AscchselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects channel 31"]
    #[inline(always)]
    pub fn chan_31(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan31)
    }
    #[doc = "Selects channel 30"]
    #[inline(always)]
    pub fn chan_30(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan30)
    }
    #[doc = "Selects channel 29"]
    #[inline(always)]
    pub fn chan_29(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan29)
    }
    #[doc = "Selects channel 28"]
    #[inline(always)]
    pub fn chan_28(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan28)
    }
    #[doc = "Selects channel 27"]
    #[inline(always)]
    pub fn chan_27(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan27)
    }
    #[doc = "Selects channel 26"]
    #[inline(always)]
    pub fn chan_26(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan26)
    }
    #[doc = "Selects channel 25"]
    #[inline(always)]
    pub fn chan_25(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan25)
    }
    #[doc = "Selects channel 24"]
    #[inline(always)]
    pub fn chan_24(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan24)
    }
    #[doc = "Selects channel 23"]
    #[inline(always)]
    pub fn chan_23(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan23)
    }
    #[doc = "Selects channel 22"]
    #[inline(always)]
    pub fn chan_22(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan22)
    }
    #[doc = "Selects channel 21"]
    #[inline(always)]
    pub fn chan_21(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan21)
    }
    #[doc = "Selects channel 20"]
    #[inline(always)]
    pub fn chan_20(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan20)
    }
    #[doc = "Selects channel 19"]
    #[inline(always)]
    pub fn chan_19(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan19)
    }
    #[doc = "Selects channel 18"]
    #[inline(always)]
    pub fn chan_18(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan18)
    }
    #[doc = "Selects channel 17"]
    #[inline(always)]
    pub fn chan_17(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan17)
    }
    #[doc = "Selects channel 16"]
    #[inline(always)]
    pub fn chan_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan16)
    }
    #[doc = "Selects channel 15"]
    #[inline(always)]
    pub fn chan_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan15)
    }
    #[doc = "Selects channel 14"]
    #[inline(always)]
    pub fn chan_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan14)
    }
    #[doc = "Selects channel 13"]
    #[inline(always)]
    pub fn chan_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan13)
    }
    #[doc = "Selects channel 12"]
    #[inline(always)]
    pub fn chan_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan12)
    }
    #[doc = "Selects channel 11"]
    #[inline(always)]
    pub fn chan_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan11)
    }
    #[doc = "Selects channel 10"]
    #[inline(always)]
    pub fn chan_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan10)
    }
    #[doc = "Selects channel 9"]
    #[inline(always)]
    pub fn chan_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan9)
    }
    #[doc = "Selects channel 8"]
    #[inline(always)]
    pub fn chan_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan8)
    }
    #[doc = "Selects channel 7"]
    #[inline(always)]
    pub fn chan_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan7)
    }
    #[doc = "Selects channel 6"]
    #[inline(always)]
    pub fn chan_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan6)
    }
    #[doc = "Selects channel 5"]
    #[inline(always)]
    pub fn chan_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan5)
    }
    #[doc = "Selects channel 4"]
    #[inline(always)]
    pub fn chan_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan4)
    }
    #[doc = "Selects channel 3"]
    #[inline(always)]
    pub fn chan_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan3)
    }
    #[doc = "Selects channel 2"]
    #[inline(always)]
    pub fn chan_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan2)
    }
    #[doc = "Selects channel 1"]
    #[inline(always)]
    pub fn chan_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan1)
    }
    #[doc = "Selects channel 0"]
    #[inline(always)]
    pub fn chan_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascchsel::Chan0)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "8:8\\]
ASC sample time compare value select. This is used to select between SCOMP0 and SCOMP1 registers for ASC operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascstime {
    #[doc = "1: Select SCOMP1"]
    SelScomp1 = 1,
    #[doc = "0: Select SCOMP0"]
    SelScomp0 = 0,
}
impl From<Ascstime> for bool {
    #[inline(always)]
    fn from(variant: Ascstime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCSTIME` reader - 8:8\\]
ASC sample time compare value select. This is used to select between SCOMP0 and SCOMP1 registers for ASC operation."]
pub type AscstimeR = crate::BitReader<Ascstime>;
impl AscstimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascstime {
        match self.bits {
            true => Ascstime::SelScomp1,
            false => Ascstime::SelScomp0,
        }
    }
    #[doc = "Select SCOMP1"]
    #[inline(always)]
    pub fn is_sel_scomp1(&self) -> bool {
        *self == Ascstime::SelScomp1
    }
    #[doc = "Select SCOMP0"]
    #[inline(always)]
    pub fn is_sel_scomp0(&self) -> bool {
        *self == Ascstime::SelScomp0
    }
}
#[doc = "Field `ASCSTIME` writer - 8:8\\]
ASC sample time compare value select. This is used to select between SCOMP0 and SCOMP1 registers for ASC operation."]
pub type AscstimeW<'a, REG> = crate::BitWriter<'a, REG, Ascstime>;
impl<'a, REG> AscstimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select SCOMP1"]
    #[inline(always)]
    pub fn sel_scomp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascstime::SelScomp1)
    }
    #[doc = "Select SCOMP0"]
    #[inline(always)]
    pub fn sel_scomp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascstime::SelScomp0)
    }
}
#[doc = "Field `RESERVED9` reader - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "13:12\\]
Selects voltage reference for ASC operation. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ascvrsel {
    #[doc = "2: Internal reference"]
    Intref = 2,
    #[doc = "1: External reference from AREF+/AREF- pins"]
    Extref = 1,
    #[doc = "0: VDDS reference"]
    Vdds = 0,
}
impl From<Ascvrsel> for u8 {
    #[inline(always)]
    fn from(variant: Ascvrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ascvrsel {
    type Ux = u8;
}
impl crate::IsEnum for Ascvrsel {}
#[doc = "Field `ASCVRSEL` reader - 13:12\\]
Selects voltage reference for ASC operation. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type AscvrselR = crate::FieldReader<Ascvrsel>;
impl AscvrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ascvrsel> {
        match self.bits {
            2 => Some(Ascvrsel::Intref),
            1 => Some(Ascvrsel::Extref),
            0 => Some(Ascvrsel::Vdds),
            _ => None,
        }
    }
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == Ascvrsel::Intref
    }
    #[doc = "External reference from AREF+/AREF- pins"]
    #[inline(always)]
    pub fn is_extref(&self) -> bool {
        *self == Ascvrsel::Extref
    }
    #[doc = "VDDS reference"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == Ascvrsel::Vdds
    }
}
#[doc = "Field `ASCVRSEL` writer - 13:12\\]
Selects voltage reference for ASC operation. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type AscvrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ascvrsel>;
impl<'a, REG> AscvrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(Ascvrsel::Intref)
    }
    #[doc = "External reference from AREF+/AREF- pins"]
    #[inline(always)]
    pub fn extref(self) -> &'a mut crate::W<REG> {
        self.variant(Ascvrsel::Extref)
    }
    #[doc = "VDDS reference"]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut crate::W<REG> {
        self.variant(Ascvrsel::Vdds)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
ASC channel select"]
    #[inline(always)]
    pub fn ascchsel(&self) -> AscchselR {
        AscchselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
ASC sample time compare value select. This is used to select between SCOMP0 and SCOMP1 registers for ASC operation."]
    #[inline(always)]
    pub fn ascstime(&self) -> AscstimeR {
        AscstimeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects voltage reference for ASC operation. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    pub fn ascvrsel(&self) -> AscvrselR {
        AscvrselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
ASC channel select"]
    #[inline(always)]
    #[must_use]
    pub fn ascchsel(&mut self) -> AscchselW<Ctl3Spec> {
        AscchselW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ASC sample time compare value select. This is used to select between SCOMP0 and SCOMP1 registers for ASC operation."]
    #[inline(always)]
    #[must_use]
    pub fn ascstime(&mut self) -> AscstimeW<Ctl3Spec> {
        AscstimeW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects voltage reference for ASC operation. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    #[must_use]
    pub fn ascvrsel(&mut self) -> AscvrselW<Ctl3Spec> {
        AscvrselW::new(self, 12)
    }
}
#[doc = "Control Register 3. This register is used to configure ADC for ad-hoc single conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl3Spec;
impl crate::RegisterSpec for Ctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3::R`](R) reader structure"]
impl crate::Readable for Ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl3::W`](W) writer structure"]
impl crate::Writable for Ctl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for Ctl3Spec {
    const RESET_VALUE: u32 = 0;
}
