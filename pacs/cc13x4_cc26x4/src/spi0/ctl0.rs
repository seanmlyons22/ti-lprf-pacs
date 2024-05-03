#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "31: Data Size Select bits: 32"]
    Dss32 = 31,
    #[doc = "30: Data Size Select bits: 31"]
    Dss31 = 30,
    #[doc = "29: Data Size Select bits: 30"]
    Dss30 = 29,
    #[doc = "28: Data Size Select bits: 29"]
    Dss29 = 28,
    #[doc = "27: Data Size Select bits: 28"]
    Dss28 = 27,
    #[doc = "26: Data Size Select bits: 27"]
    Dss27 = 26,
    #[doc = "25: Data Size Select bits: 26"]
    Dss26 = 25,
    #[doc = "24: Data Size Select bits: 25"]
    Dss25 = 24,
    #[doc = "23: Data Size Select bits: 24"]
    Dss24 = 23,
    #[doc = "22: Data Size Select bits: 23"]
    Dss23 = 22,
    #[doc = "21: Data Size Select bits: 22"]
    Dss22 = 21,
    #[doc = "20: Data Size Select bits: 21"]
    Dss21 = 20,
    #[doc = "19: Data Size Select bits: 20"]
    Dss20 = 19,
    #[doc = "18: Data Size Select bits: 19"]
    Dss19 = 18,
    #[doc = "17: Data Size Select bits: 18"]
    Dss18 = 17,
    #[doc = "16: Data Size Select bits: 17"]
    Dss17 = 16,
    #[doc = "15: Data Size Select bits: 16"]
    Dss16 = 15,
    #[doc = "14: Data Size Select bits: 15"]
    Dss15 = 14,
    #[doc = "13: Data Size Select bits: 14"]
    Dss14 = 13,
    #[doc = "12: Data Size Select bits: 13"]
    Dss13 = 12,
    #[doc = "11: Data Size Select bits: 12"]
    Dss12 = 11,
    #[doc = "10: Data Size Select bits: 11"]
    Dss11 = 10,
    #[doc = "9: Data Size Select bits: 10"]
    Dss10 = 9,
    #[doc = "8: Data Size Select bits: 9"]
    Dss9 = 8,
    #[doc = "7: Data Size Select bits: 8"]
    Dss8 = 7,
    #[doc = "6: Data Size Select bits: 7"]
    Dss7 = 6,
    #[doc = "5: Data Size Select bits: 6"]
    Dss6 = 5,
    #[doc = "4: Data Size Select bits: 5"]
    Dss5 = 4,
    #[doc = "3: Data Size Select bits: 4"]
    Dss4 = 3,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
#[doc = "Field `DSS` reader - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            31 => Some(Dss::Dss32),
            30 => Some(Dss::Dss31),
            29 => Some(Dss::Dss30),
            28 => Some(Dss::Dss29),
            27 => Some(Dss::Dss28),
            26 => Some(Dss::Dss27),
            25 => Some(Dss::Dss26),
            24 => Some(Dss::Dss25),
            23 => Some(Dss::Dss24),
            22 => Some(Dss::Dss23),
            21 => Some(Dss::Dss22),
            20 => Some(Dss::Dss21),
            19 => Some(Dss::Dss20),
            18 => Some(Dss::Dss19),
            17 => Some(Dss::Dss18),
            16 => Some(Dss::Dss17),
            15 => Some(Dss::Dss16),
            14 => Some(Dss::Dss15),
            13 => Some(Dss::Dss14),
            12 => Some(Dss::Dss13),
            11 => Some(Dss::Dss12),
            10 => Some(Dss::Dss11),
            9 => Some(Dss::Dss10),
            8 => Some(Dss::Dss9),
            7 => Some(Dss::Dss8),
            6 => Some(Dss::Dss7),
            5 => Some(Dss::Dss6),
            4 => Some(Dss::Dss5),
            3 => Some(Dss::Dss4),
            _ => None,
        }
    }
    #[doc = "Data Size Select bits: 32"]
    #[inline(always)]
    pub fn is_dss_32(&self) -> bool {
        *self == Dss::Dss32
    }
    #[doc = "Data Size Select bits: 31"]
    #[inline(always)]
    pub fn is_dss_31(&self) -> bool {
        *self == Dss::Dss31
    }
    #[doc = "Data Size Select bits: 30"]
    #[inline(always)]
    pub fn is_dss_30(&self) -> bool {
        *self == Dss::Dss30
    }
    #[doc = "Data Size Select bits: 29"]
    #[inline(always)]
    pub fn is_dss_29(&self) -> bool {
        *self == Dss::Dss29
    }
    #[doc = "Data Size Select bits: 28"]
    #[inline(always)]
    pub fn is_dss_28(&self) -> bool {
        *self == Dss::Dss28
    }
    #[doc = "Data Size Select bits: 27"]
    #[inline(always)]
    pub fn is_dss_27(&self) -> bool {
        *self == Dss::Dss27
    }
    #[doc = "Data Size Select bits: 26"]
    #[inline(always)]
    pub fn is_dss_26(&self) -> bool {
        *self == Dss::Dss26
    }
    #[doc = "Data Size Select bits: 25"]
    #[inline(always)]
    pub fn is_dss_25(&self) -> bool {
        *self == Dss::Dss25
    }
    #[doc = "Data Size Select bits: 24"]
    #[inline(always)]
    pub fn is_dss_24(&self) -> bool {
        *self == Dss::Dss24
    }
    #[doc = "Data Size Select bits: 23"]
    #[inline(always)]
    pub fn is_dss_23(&self) -> bool {
        *self == Dss::Dss23
    }
    #[doc = "Data Size Select bits: 22"]
    #[inline(always)]
    pub fn is_dss_22(&self) -> bool {
        *self == Dss::Dss22
    }
    #[doc = "Data Size Select bits: 21"]
    #[inline(always)]
    pub fn is_dss_21(&self) -> bool {
        *self == Dss::Dss21
    }
    #[doc = "Data Size Select bits: 20"]
    #[inline(always)]
    pub fn is_dss_20(&self) -> bool {
        *self == Dss::Dss20
    }
    #[doc = "Data Size Select bits: 19"]
    #[inline(always)]
    pub fn is_dss_19(&self) -> bool {
        *self == Dss::Dss19
    }
    #[doc = "Data Size Select bits: 18"]
    #[inline(always)]
    pub fn is_dss_18(&self) -> bool {
        *self == Dss::Dss18
    }
    #[doc = "Data Size Select bits: 17"]
    #[inline(always)]
    pub fn is_dss_17(&self) -> bool {
        *self == Dss::Dss17
    }
    #[doc = "Data Size Select bits: 16"]
    #[inline(always)]
    pub fn is_dss_16(&self) -> bool {
        *self == Dss::Dss16
    }
    #[doc = "Data Size Select bits: 15"]
    #[inline(always)]
    pub fn is_dss_15(&self) -> bool {
        *self == Dss::Dss15
    }
    #[doc = "Data Size Select bits: 14"]
    #[inline(always)]
    pub fn is_dss_14(&self) -> bool {
        *self == Dss::Dss14
    }
    #[doc = "Data Size Select bits: 13"]
    #[inline(always)]
    pub fn is_dss_13(&self) -> bool {
        *self == Dss::Dss13
    }
    #[doc = "Data Size Select bits: 12"]
    #[inline(always)]
    pub fn is_dss_12(&self) -> bool {
        *self == Dss::Dss12
    }
    #[doc = "Data Size Select bits: 11"]
    #[inline(always)]
    pub fn is_dss_11(&self) -> bool {
        *self == Dss::Dss11
    }
    #[doc = "Data Size Select bits: 10"]
    #[inline(always)]
    pub fn is_dss_10(&self) -> bool {
        *self == Dss::Dss10
    }
    #[doc = "Data Size Select bits: 9"]
    #[inline(always)]
    pub fn is_dss_9(&self) -> bool {
        *self == Dss::Dss9
    }
    #[doc = "Data Size Select bits: 8"]
    #[inline(always)]
    pub fn is_dss_8(&self) -> bool {
        *self == Dss::Dss8
    }
    #[doc = "Data Size Select bits: 7"]
    #[inline(always)]
    pub fn is_dss_7(&self) -> bool {
        *self == Dss::Dss7
    }
    #[doc = "Data Size Select bits: 6"]
    #[inline(always)]
    pub fn is_dss_6(&self) -> bool {
        *self == Dss::Dss6
    }
    #[doc = "Data Size Select bits: 5"]
    #[inline(always)]
    pub fn is_dss_5(&self) -> bool {
        *self == Dss::Dss5
    }
    #[doc = "Data Size Select bits: 4"]
    #[inline(always)]
    pub fn is_dss_4(&self) -> bool {
        *self == Dss::Dss4
    }
}
#[doc = "Field `DSS` writer - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data Size Select bits: 32"]
    #[inline(always)]
    pub fn dss_32(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss32)
    }
    #[doc = "Data Size Select bits: 31"]
    #[inline(always)]
    pub fn dss_31(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss31)
    }
    #[doc = "Data Size Select bits: 30"]
    #[inline(always)]
    pub fn dss_30(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss30)
    }
    #[doc = "Data Size Select bits: 29"]
    #[inline(always)]
    pub fn dss_29(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss29)
    }
    #[doc = "Data Size Select bits: 28"]
    #[inline(always)]
    pub fn dss_28(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss28)
    }
    #[doc = "Data Size Select bits: 27"]
    #[inline(always)]
    pub fn dss_27(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss27)
    }
    #[doc = "Data Size Select bits: 26"]
    #[inline(always)]
    pub fn dss_26(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss26)
    }
    #[doc = "Data Size Select bits: 25"]
    #[inline(always)]
    pub fn dss_25(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss25)
    }
    #[doc = "Data Size Select bits: 24"]
    #[inline(always)]
    pub fn dss_24(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss24)
    }
    #[doc = "Data Size Select bits: 23"]
    #[inline(always)]
    pub fn dss_23(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss23)
    }
    #[doc = "Data Size Select bits: 22"]
    #[inline(always)]
    pub fn dss_22(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss22)
    }
    #[doc = "Data Size Select bits: 21"]
    #[inline(always)]
    pub fn dss_21(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss21)
    }
    #[doc = "Data Size Select bits: 20"]
    #[inline(always)]
    pub fn dss_20(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss20)
    }
    #[doc = "Data Size Select bits: 19"]
    #[inline(always)]
    pub fn dss_19(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss19)
    }
    #[doc = "Data Size Select bits: 18"]
    #[inline(always)]
    pub fn dss_18(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss18)
    }
    #[doc = "Data Size Select bits: 17"]
    #[inline(always)]
    pub fn dss_17(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss17)
    }
    #[doc = "Data Size Select bits: 16"]
    #[inline(always)]
    pub fn dss_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss16)
    }
    #[doc = "Data Size Select bits: 15"]
    #[inline(always)]
    pub fn dss_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss15)
    }
    #[doc = "Data Size Select bits: 14"]
    #[inline(always)]
    pub fn dss_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss14)
    }
    #[doc = "Data Size Select bits: 13"]
    #[inline(always)]
    pub fn dss_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss13)
    }
    #[doc = "Data Size Select bits: 12"]
    #[inline(always)]
    pub fn dss_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss12)
    }
    #[doc = "Data Size Select bits: 11"]
    #[inline(always)]
    pub fn dss_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss11)
    }
    #[doc = "Data Size Select bits: 10"]
    #[inline(always)]
    pub fn dss_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss10)
    }
    #[doc = "Data Size Select bits: 9"]
    #[inline(always)]
    pub fn dss_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss9)
    }
    #[doc = "Data Size Select bits: 8"]
    #[inline(always)]
    pub fn dss_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss8)
    }
    #[doc = "Data Size Select bits: 7"]
    #[inline(always)]
    pub fn dss_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss7)
    }
    #[doc = "Data Size Select bits: 6"]
    #[inline(always)]
    pub fn dss_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss6)
    }
    #[doc = "Data Size Select bits: 5"]
    #[inline(always)]
    pub fn dss_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss5)
    }
    #[doc = "Data Size Select bits: 4"]
    #[inline(always)]
    pub fn dss_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Dss4)
    }
}
#[doc = "6:5\\]
Frame format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "3: National MICROWIRE frame format"]
    Mircowire = 3,
    #[doc = "2: TI synchronous serial frame format"]
    TiSync = 2,
    #[doc = "1: Motorola SPI frame format (4 wire mode)"]
    Motorola4wire = 1,
    #[doc = "0: Motorola SPI frame format (3 wire mode)"]
    Motorola3wire = 0,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
#[doc = "Field `FRF` reader - 6:5\\]
Frame format Select"]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frf {
        match self.bits {
            3 => Frf::Mircowire,
            2 => Frf::TiSync,
            1 => Frf::Motorola4wire,
            0 => Frf::Motorola3wire,
            _ => unreachable!(),
        }
    }
    #[doc = "National MICROWIRE frame format"]
    #[inline(always)]
    pub fn is_mircowire(&self) -> bool {
        *self == Frf::Mircowire
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn is_ti_sync(&self) -> bool {
        *self == Frf::TiSync
    }
    #[doc = "Motorola SPI frame format (4 wire mode)"]
    #[inline(always)]
    pub fn is_motorola_4wire(&self) -> bool {
        *self == Frf::Motorola4wire
    }
    #[doc = "Motorola SPI frame format (3 wire mode)"]
    #[inline(always)]
    pub fn is_motorola_3wire(&self) -> bool {
        *self == Frf::Motorola3wire
    }
}
#[doc = "Field `FRF` writer - 6:5\\]
Frame format Select"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf, crate::Safe>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "National MICROWIRE frame format"]
    #[inline(always)]
    pub fn mircowire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Mircowire)
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn ti_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::TiSync)
    }
    #[doc = "Motorola SPI frame format (4 wire mode)"]
    #[inline(always)]
    pub fn motorola_4wire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Motorola4wire)
    }
    #[doc = "Motorola SPI frame format (3 wire mode)"]
    #[inline(always)]
    pub fn motorola_3wire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Motorola3wire)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
CLKOUT polarity (Motorola SPI frame format only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spo {
    #[doc = "1: SPI produces a steady state HIGH value on the CLKOUT when data is not being transferred."]
    High = 1,
    #[doc = "0: SPI produces a steady state LOW value on the CLKOUT when data is not being transferred."]
    Low = 0,
}
impl From<Spo> for bool {
    #[inline(always)]
    fn from(variant: Spo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPO` reader - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub type SpoR = crate::BitReader<Spo>;
impl SpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spo {
        match self.bits {
            true => Spo::High,
            false => Spo::Low,
        }
    }
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT when data is not being transferred."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spo::High
    }
    #[doc = "SPI produces a steady state LOW value on the CLKOUT when data is not being transferred."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spo::Low
    }
}
#[doc = "Field `SPO` writer - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG, Spo>;
impl<'a, REG> SpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT when data is not being transferred."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::High)
    }
    #[doc = "SPI produces a steady state LOW value on the CLKOUT when data is not being transferred."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::Low)
    }
}
#[doc = "9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sph {
    #[doc = "1: Data is captured on the second clock edge transition."]
    Second = 1,
    #[doc = "0: Data is captured on the first clock edge transition."]
    First = 0,
}
impl From<Sph> for bool {
    #[inline(always)]
    fn from(variant: Sph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPH` reader - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SphR = crate::BitReader<Sph>;
impl SphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sph {
        match self.bits {
            true => Sph::Second,
            false => Sph::First,
        }
    }
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == Sph::Second
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == Sph::First
    }
}
#[doc = "Field `SPH` writer - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG, Sph>;
impl<'a, REG> SphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(Sph::Second)
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(Sph::First)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED12` reader - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csclr {
    #[doc = "1: Enable automatic clear of shift register when CS gets inactive."]
    Enable = 1,
    #[doc = "0: Disable automatic clear of shift register when CS gets inactive."]
    Disable = 0,
}
impl From<Csclr> for bool {
    #[inline(always)]
    fn from(variant: Csclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCLR` reader - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
pub type CsclrR = crate::BitReader<Csclr>;
impl CsclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csclr {
        match self.bits {
            true => Csclr::Enable,
            false => Csclr::Disable,
        }
    }
    #[doc = "Enable automatic clear of shift register when CS gets inactive."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Csclr::Enable
    }
    #[doc = "Disable automatic clear of shift register when CS gets inactive."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Csclr::Disable
    }
}
#[doc = "Field `CSCLR` writer - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
pub type CsclrW<'a, REG> = crate::BitWriter<'a, REG, Csclr>;
impl<'a, REG> CsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable automatic clear of shift register when CS gets inactive."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::Enable)
    }
    #[doc = "Disable automatic clear of shift register when CS gets inactive."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::Disable)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Frame format Select"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
    #[inline(always)]
    pub fn csclr(&self) -> CsclrR {
        CsclrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Data Size Select. Note: Master mode: Values 0 - 2 are reserved and shall not be used. This will map to 4 bit mode. A value of 3h corresponds to 4-bit data (and so on). Slave mode: DSS should be no less than 6 which means the minimum frame length is 7 bits."]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DssW<Ctl0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Frame format Select"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<Ctl0Spec> {
        FrfW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Ctl0Spec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SpoW<Ctl0Spec> {
        SpoW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SphW<Ctl0Spec> {
        SphW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<Ctl0Spec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<Ctl0Spec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Clear shift register counter when CS gets inactive. This bit is relevant only in the slave mode, CTL1.MS = 0."]
    #[inline(always)]
    #[must_use]
    pub fn csclr(&mut self) -> CsclrW<Ctl0Spec> {
        CsclrW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Ctl0Spec> {
        Reserved15W::new(self, 15)
    }
}
#[doc = "SPI Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
