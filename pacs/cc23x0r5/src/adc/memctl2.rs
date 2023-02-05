#[doc = "Register `MEMCTL2` reader"]
pub type R = crate::R<Memctl2Spec>;
#[doc = "Register `MEMCTL2` writer"]
pub type W = crate::W<Memctl2Spec>;
#[doc = "4:0\\]
Input channel select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chansel {
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
impl From<Chansel> for u8 {
    #[inline(always)]
    fn from(variant: Chansel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chansel {
    type Ux = u8;
}
impl crate::IsEnum for Chansel {}
#[doc = "Field `CHANSEL` reader - 4:0\\]
Input channel select."]
pub type ChanselR = crate::FieldReader<Chansel>;
impl ChanselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chansel {
        match self.bits {
            31 => Chansel::Chan31,
            30 => Chansel::Chan30,
            29 => Chansel::Chan29,
            28 => Chansel::Chan28,
            27 => Chansel::Chan27,
            26 => Chansel::Chan26,
            25 => Chansel::Chan25,
            24 => Chansel::Chan24,
            23 => Chansel::Chan23,
            22 => Chansel::Chan22,
            21 => Chansel::Chan21,
            20 => Chansel::Chan20,
            19 => Chansel::Chan19,
            18 => Chansel::Chan18,
            17 => Chansel::Chan17,
            16 => Chansel::Chan16,
            15 => Chansel::Chan15,
            14 => Chansel::Chan14,
            13 => Chansel::Chan13,
            12 => Chansel::Chan12,
            11 => Chansel::Chan11,
            10 => Chansel::Chan10,
            9 => Chansel::Chan9,
            8 => Chansel::Chan8,
            7 => Chansel::Chan7,
            6 => Chansel::Chan6,
            5 => Chansel::Chan5,
            4 => Chansel::Chan4,
            3 => Chansel::Chan3,
            2 => Chansel::Chan2,
            1 => Chansel::Chan1,
            0 => Chansel::Chan0,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects channel 31"]
    #[inline(always)]
    pub fn is_chan_31(&self) -> bool {
        *self == Chansel::Chan31
    }
    #[doc = "Selects channel 30"]
    #[inline(always)]
    pub fn is_chan_30(&self) -> bool {
        *self == Chansel::Chan30
    }
    #[doc = "Selects channel 29"]
    #[inline(always)]
    pub fn is_chan_29(&self) -> bool {
        *self == Chansel::Chan29
    }
    #[doc = "Selects channel 28"]
    #[inline(always)]
    pub fn is_chan_28(&self) -> bool {
        *self == Chansel::Chan28
    }
    #[doc = "Selects channel 27"]
    #[inline(always)]
    pub fn is_chan_27(&self) -> bool {
        *self == Chansel::Chan27
    }
    #[doc = "Selects channel 26"]
    #[inline(always)]
    pub fn is_chan_26(&self) -> bool {
        *self == Chansel::Chan26
    }
    #[doc = "Selects channel 25"]
    #[inline(always)]
    pub fn is_chan_25(&self) -> bool {
        *self == Chansel::Chan25
    }
    #[doc = "Selects channel 24"]
    #[inline(always)]
    pub fn is_chan_24(&self) -> bool {
        *self == Chansel::Chan24
    }
    #[doc = "Selects channel 23"]
    #[inline(always)]
    pub fn is_chan_23(&self) -> bool {
        *self == Chansel::Chan23
    }
    #[doc = "Selects channel 22"]
    #[inline(always)]
    pub fn is_chan_22(&self) -> bool {
        *self == Chansel::Chan22
    }
    #[doc = "Selects channel 21"]
    #[inline(always)]
    pub fn is_chan_21(&self) -> bool {
        *self == Chansel::Chan21
    }
    #[doc = "Selects channel 20"]
    #[inline(always)]
    pub fn is_chan_20(&self) -> bool {
        *self == Chansel::Chan20
    }
    #[doc = "Selects channel 19"]
    #[inline(always)]
    pub fn is_chan_19(&self) -> bool {
        *self == Chansel::Chan19
    }
    #[doc = "Selects channel 18"]
    #[inline(always)]
    pub fn is_chan_18(&self) -> bool {
        *self == Chansel::Chan18
    }
    #[doc = "Selects channel 17"]
    #[inline(always)]
    pub fn is_chan_17(&self) -> bool {
        *self == Chansel::Chan17
    }
    #[doc = "Selects channel 16"]
    #[inline(always)]
    pub fn is_chan_16(&self) -> bool {
        *self == Chansel::Chan16
    }
    #[doc = "Selects channel 15"]
    #[inline(always)]
    pub fn is_chan_15(&self) -> bool {
        *self == Chansel::Chan15
    }
    #[doc = "Selects channel 14"]
    #[inline(always)]
    pub fn is_chan_14(&self) -> bool {
        *self == Chansel::Chan14
    }
    #[doc = "Selects channel 13"]
    #[inline(always)]
    pub fn is_chan_13(&self) -> bool {
        *self == Chansel::Chan13
    }
    #[doc = "Selects channel 12"]
    #[inline(always)]
    pub fn is_chan_12(&self) -> bool {
        *self == Chansel::Chan12
    }
    #[doc = "Selects channel 11"]
    #[inline(always)]
    pub fn is_chan_11(&self) -> bool {
        *self == Chansel::Chan11
    }
    #[doc = "Selects channel 10"]
    #[inline(always)]
    pub fn is_chan_10(&self) -> bool {
        *self == Chansel::Chan10
    }
    #[doc = "Selects channel 9"]
    #[inline(always)]
    pub fn is_chan_9(&self) -> bool {
        *self == Chansel::Chan9
    }
    #[doc = "Selects channel 8"]
    #[inline(always)]
    pub fn is_chan_8(&self) -> bool {
        *self == Chansel::Chan8
    }
    #[doc = "Selects channel 7"]
    #[inline(always)]
    pub fn is_chan_7(&self) -> bool {
        *self == Chansel::Chan7
    }
    #[doc = "Selects channel 6"]
    #[inline(always)]
    pub fn is_chan_6(&self) -> bool {
        *self == Chansel::Chan6
    }
    #[doc = "Selects channel 5"]
    #[inline(always)]
    pub fn is_chan_5(&self) -> bool {
        *self == Chansel::Chan5
    }
    #[doc = "Selects channel 4"]
    #[inline(always)]
    pub fn is_chan_4(&self) -> bool {
        *self == Chansel::Chan4
    }
    #[doc = "Selects channel 3"]
    #[inline(always)]
    pub fn is_chan_3(&self) -> bool {
        *self == Chansel::Chan3
    }
    #[doc = "Selects channel 2"]
    #[inline(always)]
    pub fn is_chan_2(&self) -> bool {
        *self == Chansel::Chan2
    }
    #[doc = "Selects channel 1"]
    #[inline(always)]
    pub fn is_chan_1(&self) -> bool {
        *self == Chansel::Chan1
    }
    #[doc = "Selects channel 0"]
    #[inline(always)]
    pub fn is_chan_0(&self) -> bool {
        *self == Chansel::Chan0
    }
}
#[doc = "Field `CHANSEL` writer - 4:0\\]
Input channel select."]
pub type ChanselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Chansel, crate::Safe>;
impl<'a, REG> ChanselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects channel 31"]
    #[inline(always)]
    pub fn chan_31(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan31)
    }
    #[doc = "Selects channel 30"]
    #[inline(always)]
    pub fn chan_30(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan30)
    }
    #[doc = "Selects channel 29"]
    #[inline(always)]
    pub fn chan_29(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan29)
    }
    #[doc = "Selects channel 28"]
    #[inline(always)]
    pub fn chan_28(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan28)
    }
    #[doc = "Selects channel 27"]
    #[inline(always)]
    pub fn chan_27(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan27)
    }
    #[doc = "Selects channel 26"]
    #[inline(always)]
    pub fn chan_26(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan26)
    }
    #[doc = "Selects channel 25"]
    #[inline(always)]
    pub fn chan_25(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan25)
    }
    #[doc = "Selects channel 24"]
    #[inline(always)]
    pub fn chan_24(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan24)
    }
    #[doc = "Selects channel 23"]
    #[inline(always)]
    pub fn chan_23(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan23)
    }
    #[doc = "Selects channel 22"]
    #[inline(always)]
    pub fn chan_22(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan22)
    }
    #[doc = "Selects channel 21"]
    #[inline(always)]
    pub fn chan_21(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan21)
    }
    #[doc = "Selects channel 20"]
    #[inline(always)]
    pub fn chan_20(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan20)
    }
    #[doc = "Selects channel 19"]
    #[inline(always)]
    pub fn chan_19(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan19)
    }
    #[doc = "Selects channel 18"]
    #[inline(always)]
    pub fn chan_18(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan18)
    }
    #[doc = "Selects channel 17"]
    #[inline(always)]
    pub fn chan_17(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan17)
    }
    #[doc = "Selects channel 16"]
    #[inline(always)]
    pub fn chan_16(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan16)
    }
    #[doc = "Selects channel 15"]
    #[inline(always)]
    pub fn chan_15(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan15)
    }
    #[doc = "Selects channel 14"]
    #[inline(always)]
    pub fn chan_14(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan14)
    }
    #[doc = "Selects channel 13"]
    #[inline(always)]
    pub fn chan_13(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan13)
    }
    #[doc = "Selects channel 12"]
    #[inline(always)]
    pub fn chan_12(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan12)
    }
    #[doc = "Selects channel 11"]
    #[inline(always)]
    pub fn chan_11(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan11)
    }
    #[doc = "Selects channel 10"]
    #[inline(always)]
    pub fn chan_10(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan10)
    }
    #[doc = "Selects channel 9"]
    #[inline(always)]
    pub fn chan_9(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan9)
    }
    #[doc = "Selects channel 8"]
    #[inline(always)]
    pub fn chan_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan8)
    }
    #[doc = "Selects channel 7"]
    #[inline(always)]
    pub fn chan_7(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan7)
    }
    #[doc = "Selects channel 6"]
    #[inline(always)]
    pub fn chan_6(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan6)
    }
    #[doc = "Selects channel 5"]
    #[inline(always)]
    pub fn chan_5(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan5)
    }
    #[doc = "Selects channel 4"]
    #[inline(always)]
    pub fn chan_4(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan4)
    }
    #[doc = "Selects channel 3"]
    #[inline(always)]
    pub fn chan_3(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan3)
    }
    #[doc = "Selects channel 2"]
    #[inline(always)]
    pub fn chan_2(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan2)
    }
    #[doc = "Selects channel 1"]
    #[inline(always)]
    pub fn chan_1(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan1)
    }
    #[doc = "Selects channel 0"]
    #[inline(always)]
    pub fn chan_0(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan0)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "9:8\\]
Voltage reference selection. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vrsel {
    #[doc = "2: Internal reference"]
    Intref = 2,
    #[doc = "1: External reference from AREF+/AREF- pins"]
    Extref = 1,
    #[doc = "0: VDDS reference"]
    Vdds = 0,
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(variant: Vrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vrsel {
    type Ux = u8;
}
impl crate::IsEnum for Vrsel {}
#[doc = "Field `VRSEL` reader - 9:8\\]
Voltage reference selection. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type VrselR = crate::FieldReader<Vrsel>;
impl VrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vrsel> {
        match self.bits {
            2 => Some(Vrsel::Intref),
            1 => Some(Vrsel::Extref),
            0 => Some(Vrsel::Vdds),
            _ => None,
        }
    }
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == Vrsel::Intref
    }
    #[doc = "External reference from AREF+/AREF- pins"]
    #[inline(always)]
    pub fn is_extref(&self) -> bool {
        *self == Vrsel::Extref
    }
    #[doc = "VDDS reference"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == Vrsel::Vdds
    }
}
#[doc = "Field `VRSEL` writer - 9:8\\]
Voltage reference selection. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type VrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vrsel>;
impl<'a, REG> VrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Intref)
    }
    #[doc = "External reference from AREF+/AREF- pins"]
    #[inline(always)]
    pub fn extref(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Extref)
    }
    #[doc = "VDDS reference"]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Vdds)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "12:12\\]
Selects the source of sample timer period between SCOMP0 and SCOMP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stime {
    #[doc = "1: Select SCOMP1"]
    SelScomp1 = 1,
    #[doc = "0: Select SCOMP0"]
    SelScomp0 = 0,
}
impl From<Stime> for bool {
    #[inline(always)]
    fn from(variant: Stime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIME` reader - 12:12\\]
Selects the source of sample timer period between SCOMP0 and SCOMP1."]
pub type StimeR = crate::BitReader<Stime>;
impl StimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stime {
        match self.bits {
            true => Stime::SelScomp1,
            false => Stime::SelScomp0,
        }
    }
    #[doc = "Select SCOMP1"]
    #[inline(always)]
    pub fn is_sel_scomp1(&self) -> bool {
        *self == Stime::SelScomp1
    }
    #[doc = "Select SCOMP0"]
    #[inline(always)]
    pub fn is_sel_scomp0(&self) -> bool {
        *self == Stime::SelScomp0
    }
}
#[doc = "Field `STIME` writer - 12:12\\]
Selects the source of sample timer period between SCOMP0 and SCOMP1."]
pub type StimeW<'a, REG> = crate::BitWriter<'a, REG, Stime>;
impl<'a, REG> StimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select SCOMP1"]
    #[inline(always)]
    pub fn sel_scomp1(self) -> &'a mut crate::W<REG> {
        self.variant(Stime::SelScomp1)
    }
    #[doc = "Select SCOMP0"]
    #[inline(always)]
    pub fn sel_scomp0(self) -> &'a mut crate::W<REG> {
        self.variant(Stime::SelScomp0)
    }
}
#[doc = "Field `RESERVED13` reader - 23:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader<u16>;
#[doc = "24:24\\]
Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "1: Next conversion requires a trigger"]
    TriggerNext = 1,
    #[doc = "0: Next conversion is automatic"]
    AutoNext = 0,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` reader - 24:24\\]
Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
pub type TrigR = crate::BitReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            true => Trig::TriggerNext,
            false => Trig::AutoNext,
        }
    }
    #[doc = "Next conversion requires a trigger"]
    #[inline(always)]
    pub fn is_trigger_next(&self) -> bool {
        *self == Trig::TriggerNext
    }
    #[doc = "Next conversion is automatic"]
    #[inline(always)]
    pub fn is_auto_next(&self) -> bool {
        *self == Trig::AutoNext
    }
}
#[doc = "Field `TRIG` writer - 24:24\\]
Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next conversion requires a trigger"]
    #[inline(always)]
    pub fn trigger_next(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::TriggerNext)
    }
    #[doc = "Next conversion is automatic"]
    #[inline(always)]
    pub fn auto_next(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::AutoNext)
    }
}
#[doc = "Field `RESERVED25` reader - 27:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "28:28\\]
Enable window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wincomp {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Wincomp> for bool {
    #[inline(always)]
    fn from(variant: Wincomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINCOMP` reader - 28:28\\]
Enable window comparator."]
pub type WincompR = crate::BitReader<Wincomp>;
impl WincompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wincomp {
        match self.bits {
            true => Wincomp::Enable,
            false => Wincomp::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wincomp::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wincomp::Disable
    }
}
#[doc = "Field `WINCOMP` writer - 28:28\\]
Enable window comparator."]
pub type WincompW<'a, REG> = crate::BitWriter<'a, REG, Wincomp>;
impl<'a, REG> WincompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wincomp::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wincomp::Disable)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Input channel select."]
    #[inline(always)]
    pub fn chansel(&self) -> ChanselR {
        ChanselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Voltage reference selection. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    pub fn vrsel(&self) -> VrselR {
        VrselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Selects the source of sample timer period between SCOMP0 and SCOMP1."]
    #[inline(always)]
    pub fn stime(&self) -> StimeR {
        StimeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:23 - 23:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Enable window comparator."]
    #[inline(always)]
    pub fn wincomp(&self) -> WincompR {
        WincompR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Input channel select."]
    #[inline(always)]
    #[must_use]
    pub fn chansel(&mut self) -> ChanselW<Memctl2Spec> {
        ChanselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Voltage reference selection. AREF- must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VrselW<Memctl2Spec> {
        VrselW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Selects the source of sample timer period between SCOMP0 and SCOMP1."]
    #[inline(always)]
    #[must_use]
    pub fn stime(&mut self) -> StimeW<Memctl2Spec> {
        StimeW::new(self, 12)
    }
    #[doc = "Bit 24 - 24:24\\]
Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TrigW<Memctl2Spec> {
        TrigW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Enable window comparator."]
    #[inline(always)]
    #[must_use]
    pub fn wincomp(&mut self) -> WincompW<Memctl2Spec> {
        WincompW::new(self, 28)
    }
}
#[doc = "Conversion Memory Control Register 2. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memctl2Spec;
impl crate::RegisterSpec for Memctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memctl2::R`](R) reader structure"]
impl crate::Readable for Memctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`memctl2::W`](W) writer structure"]
impl crate::Writable for Memctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMCTL2 to value 0"]
impl crate::Resettable for Memctl2Spec {
    const RESET_VALUE: u32 = 0;
}
