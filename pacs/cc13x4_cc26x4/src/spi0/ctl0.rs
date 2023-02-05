#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSS` reader - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
pub type DSS_R = crate::FieldReader<u8, DSS_A>;
#[doc = "4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSS_A {
    #[doc = "31: Data Size Select bits: 32"]
    DSS_32 = 31,
    #[doc = "30: Data Size Select bits: 31"]
    DSS_31 = 30,
    #[doc = "29: Data Size Select bits: 30"]
    DSS_30 = 29,
    #[doc = "28: Data Size Select bits: 29"]
    DSS_29 = 28,
    #[doc = "27: Data Size Select bits: 28"]
    DSS_28 = 27,
    #[doc = "26: Data Size Select bits: 27"]
    DSS_27 = 26,
    #[doc = "25: Data Size Select bits: 26"]
    DSS_26 = 25,
    #[doc = "24: Data Size Select bits: 25"]
    DSS_25 = 24,
    #[doc = "23: Data Size Select bits: 24"]
    DSS_24 = 23,
    #[doc = "22: Data Size Select bits: 23"]
    DSS_23 = 22,
    #[doc = "21: Data Size Select bits: 22"]
    DSS_22 = 21,
    #[doc = "20: Data Size Select bits: 21"]
    DSS_21 = 20,
    #[doc = "19: Data Size Select bits: 20"]
    DSS_20 = 19,
    #[doc = "18: Data Size Select bits: 19"]
    DSS_19 = 18,
    #[doc = "17: Data Size Select bits: 18"]
    DSS_18 = 17,
    #[doc = "16: Data Size Select bits: 17"]
    DSS_17 = 16,
    #[doc = "15: Data Size Select bits: 16"]
    DSS_16 = 15,
    #[doc = "14: Data Size Select bits: 15"]
    DSS_15 = 14,
    #[doc = "13: Data Size Select bits: 14"]
    DSS_14 = 13,
    #[doc = "12: Data Size Select bits: 13"]
    DSS_13 = 12,
    #[doc = "11: Data Size Select bits: 12"]
    DSS_12 = 11,
    #[doc = "10: Data Size Select bits: 11"]
    DSS_11 = 10,
    #[doc = "9: Data Size Select bits: 10"]
    DSS_10 = 9,
    #[doc = "8: Data Size Select bits: 9"]
    DSS_9 = 8,
    #[doc = "7: Data Size Select bits: 8"]
    DSS_8 = 7,
    #[doc = "6: Data Size Select bits: 7"]
    DSS_7 = 6,
    #[doc = "5: Data Size Select bits: 6"]
    DSS_6 = 5,
    #[doc = "4: Data Size Select bits: 5"]
    DSS_5 = 4,
    #[doc = "3: Data Size Select bits: 4"]
    DSS_4 = 3,
}
impl From<DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSS_A) -> Self {
        variant as _
    }
}
impl DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSS_A> {
        match self.bits {
            31 => Some(DSS_A::DSS_32),
            30 => Some(DSS_A::DSS_31),
            29 => Some(DSS_A::DSS_30),
            28 => Some(DSS_A::DSS_29),
            27 => Some(DSS_A::DSS_28),
            26 => Some(DSS_A::DSS_27),
            25 => Some(DSS_A::DSS_26),
            24 => Some(DSS_A::DSS_25),
            23 => Some(DSS_A::DSS_24),
            22 => Some(DSS_A::DSS_23),
            21 => Some(DSS_A::DSS_22),
            20 => Some(DSS_A::DSS_21),
            19 => Some(DSS_A::DSS_20),
            18 => Some(DSS_A::DSS_19),
            17 => Some(DSS_A::DSS_18),
            16 => Some(DSS_A::DSS_17),
            15 => Some(DSS_A::DSS_16),
            14 => Some(DSS_A::DSS_15),
            13 => Some(DSS_A::DSS_14),
            12 => Some(DSS_A::DSS_13),
            11 => Some(DSS_A::DSS_12),
            10 => Some(DSS_A::DSS_11),
            9 => Some(DSS_A::DSS_10),
            8 => Some(DSS_A::DSS_9),
            7 => Some(DSS_A::DSS_8),
            6 => Some(DSS_A::DSS_7),
            5 => Some(DSS_A::DSS_6),
            4 => Some(DSS_A::DSS_5),
            3 => Some(DSS_A::DSS_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DSS_32`"]
    #[inline(always)]
    pub fn is_dss_32(&self) -> bool {
        *self == DSS_A::DSS_32
    }
    #[doc = "Checks if the value of the field is `DSS_31`"]
    #[inline(always)]
    pub fn is_dss_31(&self) -> bool {
        *self == DSS_A::DSS_31
    }
    #[doc = "Checks if the value of the field is `DSS_30`"]
    #[inline(always)]
    pub fn is_dss_30(&self) -> bool {
        *self == DSS_A::DSS_30
    }
    #[doc = "Checks if the value of the field is `DSS_29`"]
    #[inline(always)]
    pub fn is_dss_29(&self) -> bool {
        *self == DSS_A::DSS_29
    }
    #[doc = "Checks if the value of the field is `DSS_28`"]
    #[inline(always)]
    pub fn is_dss_28(&self) -> bool {
        *self == DSS_A::DSS_28
    }
    #[doc = "Checks if the value of the field is `DSS_27`"]
    #[inline(always)]
    pub fn is_dss_27(&self) -> bool {
        *self == DSS_A::DSS_27
    }
    #[doc = "Checks if the value of the field is `DSS_26`"]
    #[inline(always)]
    pub fn is_dss_26(&self) -> bool {
        *self == DSS_A::DSS_26
    }
    #[doc = "Checks if the value of the field is `DSS_25`"]
    #[inline(always)]
    pub fn is_dss_25(&self) -> bool {
        *self == DSS_A::DSS_25
    }
    #[doc = "Checks if the value of the field is `DSS_24`"]
    #[inline(always)]
    pub fn is_dss_24(&self) -> bool {
        *self == DSS_A::DSS_24
    }
    #[doc = "Checks if the value of the field is `DSS_23`"]
    #[inline(always)]
    pub fn is_dss_23(&self) -> bool {
        *self == DSS_A::DSS_23
    }
    #[doc = "Checks if the value of the field is `DSS_22`"]
    #[inline(always)]
    pub fn is_dss_22(&self) -> bool {
        *self == DSS_A::DSS_22
    }
    #[doc = "Checks if the value of the field is `DSS_21`"]
    #[inline(always)]
    pub fn is_dss_21(&self) -> bool {
        *self == DSS_A::DSS_21
    }
    #[doc = "Checks if the value of the field is `DSS_20`"]
    #[inline(always)]
    pub fn is_dss_20(&self) -> bool {
        *self == DSS_A::DSS_20
    }
    #[doc = "Checks if the value of the field is `DSS_19`"]
    #[inline(always)]
    pub fn is_dss_19(&self) -> bool {
        *self == DSS_A::DSS_19
    }
    #[doc = "Checks if the value of the field is `DSS_18`"]
    #[inline(always)]
    pub fn is_dss_18(&self) -> bool {
        *self == DSS_A::DSS_18
    }
    #[doc = "Checks if the value of the field is `DSS_17`"]
    #[inline(always)]
    pub fn is_dss_17(&self) -> bool {
        *self == DSS_A::DSS_17
    }
    #[doc = "Checks if the value of the field is `DSS_16`"]
    #[inline(always)]
    pub fn is_dss_16(&self) -> bool {
        *self == DSS_A::DSS_16
    }
    #[doc = "Checks if the value of the field is `DSS_15`"]
    #[inline(always)]
    pub fn is_dss_15(&self) -> bool {
        *self == DSS_A::DSS_15
    }
    #[doc = "Checks if the value of the field is `DSS_14`"]
    #[inline(always)]
    pub fn is_dss_14(&self) -> bool {
        *self == DSS_A::DSS_14
    }
    #[doc = "Checks if the value of the field is `DSS_13`"]
    #[inline(always)]
    pub fn is_dss_13(&self) -> bool {
        *self == DSS_A::DSS_13
    }
    #[doc = "Checks if the value of the field is `DSS_12`"]
    #[inline(always)]
    pub fn is_dss_12(&self) -> bool {
        *self == DSS_A::DSS_12
    }
    #[doc = "Checks if the value of the field is `DSS_11`"]
    #[inline(always)]
    pub fn is_dss_11(&self) -> bool {
        *self == DSS_A::DSS_11
    }
    #[doc = "Checks if the value of the field is `DSS_10`"]
    #[inline(always)]
    pub fn is_dss_10(&self) -> bool {
        *self == DSS_A::DSS_10
    }
    #[doc = "Checks if the value of the field is `DSS_9`"]
    #[inline(always)]
    pub fn is_dss_9(&self) -> bool {
        *self == DSS_A::DSS_9
    }
    #[doc = "Checks if the value of the field is `DSS_8`"]
    #[inline(always)]
    pub fn is_dss_8(&self) -> bool {
        *self == DSS_A::DSS_8
    }
    #[doc = "Checks if the value of the field is `DSS_7`"]
    #[inline(always)]
    pub fn is_dss_7(&self) -> bool {
        *self == DSS_A::DSS_7
    }
    #[doc = "Checks if the value of the field is `DSS_6`"]
    #[inline(always)]
    pub fn is_dss_6(&self) -> bool {
        *self == DSS_A::DSS_6
    }
    #[doc = "Checks if the value of the field is `DSS_5`"]
    #[inline(always)]
    pub fn is_dss_5(&self) -> bool {
        *self == DSS_A::DSS_5
    }
    #[doc = "Checks if the value of the field is `DSS_4`"]
    #[inline(always)]
    pub fn is_dss_4(&self) -> bool {
        *self == DSS_A::DSS_4
    }
}
#[doc = "Field `DSS` writer - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
pub type DSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, DSS_A, 5, O>;
impl<'a, const O: u8> DSS_W<'a, O> {
    #[doc = "Data Size Select bits: 32"]
    #[inline(always)]
    pub fn dss_32(self) -> &'a mut W {
        self.variant(DSS_A::DSS_32)
    }
    #[doc = "Data Size Select bits: 31"]
    #[inline(always)]
    pub fn dss_31(self) -> &'a mut W {
        self.variant(DSS_A::DSS_31)
    }
    #[doc = "Data Size Select bits: 30"]
    #[inline(always)]
    pub fn dss_30(self) -> &'a mut W {
        self.variant(DSS_A::DSS_30)
    }
    #[doc = "Data Size Select bits: 29"]
    #[inline(always)]
    pub fn dss_29(self) -> &'a mut W {
        self.variant(DSS_A::DSS_29)
    }
    #[doc = "Data Size Select bits: 28"]
    #[inline(always)]
    pub fn dss_28(self) -> &'a mut W {
        self.variant(DSS_A::DSS_28)
    }
    #[doc = "Data Size Select bits: 27"]
    #[inline(always)]
    pub fn dss_27(self) -> &'a mut W {
        self.variant(DSS_A::DSS_27)
    }
    #[doc = "Data Size Select bits: 26"]
    #[inline(always)]
    pub fn dss_26(self) -> &'a mut W {
        self.variant(DSS_A::DSS_26)
    }
    #[doc = "Data Size Select bits: 25"]
    #[inline(always)]
    pub fn dss_25(self) -> &'a mut W {
        self.variant(DSS_A::DSS_25)
    }
    #[doc = "Data Size Select bits: 24"]
    #[inline(always)]
    pub fn dss_24(self) -> &'a mut W {
        self.variant(DSS_A::DSS_24)
    }
    #[doc = "Data Size Select bits: 23"]
    #[inline(always)]
    pub fn dss_23(self) -> &'a mut W {
        self.variant(DSS_A::DSS_23)
    }
    #[doc = "Data Size Select bits: 22"]
    #[inline(always)]
    pub fn dss_22(self) -> &'a mut W {
        self.variant(DSS_A::DSS_22)
    }
    #[doc = "Data Size Select bits: 21"]
    #[inline(always)]
    pub fn dss_21(self) -> &'a mut W {
        self.variant(DSS_A::DSS_21)
    }
    #[doc = "Data Size Select bits: 20"]
    #[inline(always)]
    pub fn dss_20(self) -> &'a mut W {
        self.variant(DSS_A::DSS_20)
    }
    #[doc = "Data Size Select bits: 19"]
    #[inline(always)]
    pub fn dss_19(self) -> &'a mut W {
        self.variant(DSS_A::DSS_19)
    }
    #[doc = "Data Size Select bits: 18"]
    #[inline(always)]
    pub fn dss_18(self) -> &'a mut W {
        self.variant(DSS_A::DSS_18)
    }
    #[doc = "Data Size Select bits: 17"]
    #[inline(always)]
    pub fn dss_17(self) -> &'a mut W {
        self.variant(DSS_A::DSS_17)
    }
    #[doc = "Data Size Select bits: 16"]
    #[inline(always)]
    pub fn dss_16(self) -> &'a mut W {
        self.variant(DSS_A::DSS_16)
    }
    #[doc = "Data Size Select bits: 15"]
    #[inline(always)]
    pub fn dss_15(self) -> &'a mut W {
        self.variant(DSS_A::DSS_15)
    }
    #[doc = "Data Size Select bits: 14"]
    #[inline(always)]
    pub fn dss_14(self) -> &'a mut W {
        self.variant(DSS_A::DSS_14)
    }
    #[doc = "Data Size Select bits: 13"]
    #[inline(always)]
    pub fn dss_13(self) -> &'a mut W {
        self.variant(DSS_A::DSS_13)
    }
    #[doc = "Data Size Select bits: 12"]
    #[inline(always)]
    pub fn dss_12(self) -> &'a mut W {
        self.variant(DSS_A::DSS_12)
    }
    #[doc = "Data Size Select bits: 11"]
    #[inline(always)]
    pub fn dss_11(self) -> &'a mut W {
        self.variant(DSS_A::DSS_11)
    }
    #[doc = "Data Size Select bits: 10"]
    #[inline(always)]
    pub fn dss_10(self) -> &'a mut W {
        self.variant(DSS_A::DSS_10)
    }
    #[doc = "Data Size Select bits: 9"]
    #[inline(always)]
    pub fn dss_9(self) -> &'a mut W {
        self.variant(DSS_A::DSS_9)
    }
    #[doc = "Data Size Select bits: 8"]
    #[inline(always)]
    pub fn dss_8(self) -> &'a mut W {
        self.variant(DSS_A::DSS_8)
    }
    #[doc = "Data Size Select bits: 7"]
    #[inline(always)]
    pub fn dss_7(self) -> &'a mut W {
        self.variant(DSS_A::DSS_7)
    }
    #[doc = "Data Size Select bits: 6"]
    #[inline(always)]
    pub fn dss_6(self) -> &'a mut W {
        self.variant(DSS_A::DSS_6)
    }
    #[doc = "Data Size Select bits: 5"]
    #[inline(always)]
    pub fn dss_5(self) -> &'a mut W {
        self.variant(DSS_A::DSS_5)
    }
    #[doc = "Data Size Select bits: 4"]
    #[inline(always)]
    pub fn dss_4(self) -> &'a mut W {
        self.variant(DSS_A::DSS_4)
    }
}
#[doc = "Field `FRF` reader - 6:5\\]
Frame format Select"]
pub type FRF_R = crate::FieldReader<u8, FRF_A>;
#[doc = "6:5\\]
Frame format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRF_A {
    #[doc = "3: National MICROWIRE frame format"]
    MIRCOWIRE = 3,
    #[doc = "2: TI synchronous serial frame format"]
    TI_SYNC = 2,
    #[doc = "1: Motorola SPI frame format (4 wire mode)"]
    MOTOROLA_4WIRE = 1,
    #[doc = "0: Motorola SPI frame format (3 wire mode)"]
    MOTOROLA_3WIRE = 0,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as _
    }
}
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            3 => FRF_A::MIRCOWIRE,
            2 => FRF_A::TI_SYNC,
            1 => FRF_A::MOTOROLA_4WIRE,
            0 => FRF_A::MOTOROLA_3WIRE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MIRCOWIRE`"]
    #[inline(always)]
    pub fn is_mircowire(&self) -> bool {
        *self == FRF_A::MIRCOWIRE
    }
    #[doc = "Checks if the value of the field is `TI_SYNC`"]
    #[inline(always)]
    pub fn is_ti_sync(&self) -> bool {
        *self == FRF_A::TI_SYNC
    }
    #[doc = "Checks if the value of the field is `MOTOROLA_4WIRE`"]
    #[inline(always)]
    pub fn is_motorola_4wire(&self) -> bool {
        *self == FRF_A::MOTOROLA_4WIRE
    }
    #[doc = "Checks if the value of the field is `MOTOROLA_3WIRE`"]
    #[inline(always)]
    pub fn is_motorola_3wire(&self) -> bool {
        *self == FRF_A::MOTOROLA_3WIRE
    }
}
#[doc = "Field `FRF` writer - 6:5\\]
Frame format Select"]
pub type FRF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, FRF_A, 2, O>;
impl<'a, const O: u8> FRF_W<'a, O> {
    #[doc = "National MICROWIRE frame format"]
    #[inline(always)]
    pub fn mircowire(self) -> &'a mut W {
        self.variant(FRF_A::MIRCOWIRE)
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn ti_sync(self) -> &'a mut W {
        self.variant(FRF_A::TI_SYNC)
    }
    #[doc = "Motorola SPI frame format (4 wire mode)"]
    #[inline(always)]
    pub fn motorola_4wire(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA_4WIRE)
    }
    #[doc = "Motorola SPI frame format (3 wire mode)"]
    #[inline(always)]
    pub fn motorola_3wire(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA_3WIRE)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `SPO` reader - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub type SPO_R = crate::BitReader<SPO_A>;
#[doc = "8:8\\]
CLKOUT polarity (Motorola SPI frame format only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPO_A {
    #[doc = "1: SPI produces a steady state HIGH value on the CLKOUT when data is not being transferred."]
    HIGH = 1,
    #[doc = "0: SPI produces a steady state LOW value on the CLKOUT when data is not being transferred."]
    LOW = 0,
}
impl From<SPO_A> for bool {
    #[inline(always)]
    fn from(variant: SPO_A) -> Self {
        variant as u8 != 0
    }
}
impl SPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPO_A {
        match self.bits {
            true => SPO_A::HIGH,
            false => SPO_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPO_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPO_A::LOW
    }
}
#[doc = "Field `SPO` writer - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub type SPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, SPO_A, O>;
impl<'a, const O: u8> SPO_W<'a, O> {
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT when data is not being transferred."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPO_A::HIGH)
    }
    #[doc = "SPI produces a steady state LOW value on the CLKOUT when data is not being transferred."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPO_A::LOW)
    }
}
#[doc = "Field `SPH` reader - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SPH_R = crate::BitReader<SPH_A>;
#[doc = "9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPH_A {
    #[doc = "1: Data is captured on the second clock edge transition."]
    SECOND = 1,
    #[doc = "0: Data is captured on the first clock edge transition."]
    FIRST = 0,
}
impl From<SPH_A> for bool {
    #[inline(always)]
    fn from(variant: SPH_A) -> Self {
        variant as u8 != 0
    }
}
impl SPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPH_A {
        match self.bits {
            true => SPH_A::SECOND,
            false => SPH_A::FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == SPH_A::SECOND
    }
    #[doc = "Checks if the value of the field is `FIRST`"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == SPH_A::FIRST
    }
}
#[doc = "Field `SPH` writer - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, SPH_A, O>;
impl<'a, const O: u8> SPH_W<'a, O> {
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(SPH_A::SECOND)
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn first(self) -> &'a mut W {
        self.variant(SPH_A::FIRST)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED12` reader - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CSCLR` reader - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
pub type CSCLR_R = crate::BitReader<CSCLR_A>;
#[doc = "14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCLR_A {
    #[doc = "1: Enable automatic clear of shift register when CS gets inactive."]
    ENABLE = 1,
    #[doc = "0: Disable automatic clear of shift register when CS gets inactive."]
    DISABLE = 0,
}
impl From<CSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CSCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCLR_A {
        match self.bits {
            true => CSCLR_A::ENABLE,
            false => CSCLR_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CSCLR_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CSCLR_A::DISABLE
    }
}
#[doc = "Field `CSCLR` writer - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
pub type CSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, CSCLR_A, O>;
impl<'a, const O: u8> CSCLR_W<'a, O> {
    #[doc = "Enable automatic clear of shift register when CS gets inactive."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CSCLR_A::ENABLE)
    }
    #[doc = "Disable automatic clear of shift register when CS gets inactive."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CSCLR_A::DISABLE)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Frame format Select"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
    #[inline(always)]
    pub fn csclr(&self) -> CSCLR_R {
        CSCLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DSS_W<0> {
        DSS_W::new(self)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Frame format Select"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<5> {
        FRF_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SPO_W<8> {
        SPO_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SPH_W<9> {
        SPH_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
    #[inline(always)]
    #[must_use]
    pub fn csclr(&mut self) -> CSCLR_W<14> {
        CSCLR_W::new(self)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
