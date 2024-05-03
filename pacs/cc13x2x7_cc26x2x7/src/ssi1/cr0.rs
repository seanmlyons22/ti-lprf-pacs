#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "15: 16-bit data"]
    _16Bit = 15,
    #[doc = "14: 15-bit data"]
    _15Bit = 14,
    #[doc = "13: 14-bit data"]
    _14Bit = 13,
    #[doc = "12: 13-bit data"]
    _13Bit = 12,
    #[doc = "11: 12-bit data"]
    _12Bit = 11,
    #[doc = "10: 11-bit data"]
    _11Bit = 10,
    #[doc = "9: 10-bit data"]
    _10Bit = 9,
    #[doc = "8: 9-bit data"]
    _9Bit = 8,
    #[doc = "7: 8-bit data"]
    _8Bit = 7,
    #[doc = "6: 7-bit data"]
    _7Bit = 6,
    #[doc = "5: 6-bit data"]
    _6Bit = 5,
    #[doc = "4: 5-bit data"]
    _5Bit = 4,
    #[doc = "3: 4-bit data"]
    _4Bit = 3,
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
#[doc = "Field `DSS` reader - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            15 => Some(Dss::_16Bit),
            14 => Some(Dss::_15Bit),
            13 => Some(Dss::_14Bit),
            12 => Some(Dss::_13Bit),
            11 => Some(Dss::_12Bit),
            10 => Some(Dss::_11Bit),
            9 => Some(Dss::_10Bit),
            8 => Some(Dss::_9Bit),
            7 => Some(Dss::_8Bit),
            6 => Some(Dss::_7Bit),
            5 => Some(Dss::_6Bit),
            4 => Some(Dss::_5Bit),
            3 => Some(Dss::_4Bit),
            _ => None,
        }
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Dss::_16Bit
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == Dss::_15Bit
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == Dss::_14Bit
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == Dss::_13Bit
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == Dss::_12Bit
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == Dss::_11Bit
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == Dss::_10Bit
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Dss::_9Bit
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Dss::_8Bit
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Dss::_7Bit
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Dss::_6Bit
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Dss::_5Bit
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn is_4_bit(&self) -> bool {
        *self == Dss::_4Bit
    }
}
#[doc = "Field `DSS` writer - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_16Bit)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_15Bit)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_14Bit)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_13Bit)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_12Bit)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_11Bit)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_10Bit)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_9Bit)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_8Bit)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_7Bit)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_6Bit)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_5Bit)
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn _4_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_4Bit)
    }
}
#[doc = "5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "2: National Microwire frame format"]
    NationalMicrowire = 2,
    #[doc = "1: TI synchronous serial frame format"]
    TiSyncSerial = 1,
    #[doc = "0: Motorola SPI frame format"]
    MotorolaSpi = 0,
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
#[doc = "Field `FRF` reader - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            2 => Some(Frf::NationalMicrowire),
            1 => Some(Frf::TiSyncSerial),
            0 => Some(Frf::MotorolaSpi),
            _ => None,
        }
    }
    #[doc = "National Microwire frame format"]
    #[inline(always)]
    pub fn is_national_microwire(&self) -> bool {
        *self == Frf::NationalMicrowire
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn is_ti_sync_serial(&self) -> bool {
        *self == Frf::TiSyncSerial
    }
    #[doc = "Motorola SPI frame format"]
    #[inline(always)]
    pub fn is_motorola_spi(&self) -> bool {
        *self == Frf::MotorolaSpi
    }
}
#[doc = "Field `FRF` writer - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "National Microwire frame format"]
    #[inline(always)]
    pub fn national_microwire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::NationalMicrowire)
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn ti_sync_serial(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::TiSyncSerial)
    }
    #[doc = "Motorola SPI frame format"]
    #[inline(always)]
    pub fn motorola_spi(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::MotorolaSpi)
    }
}
#[doc = "6:6\\]
CLKOUT polarity (Motorola SPI frame format only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spo {
    #[doc = "1: SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    High = 1,
    #[doc = "0: SSI produces a steady state LOW value on the CLKOUT pin when data is not being transferred."]
    Low = 0,
}
impl From<Spo> for bool {
    #[inline(always)]
    fn from(variant: Spo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPO` reader - 6:6\\]
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
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spo::High
    }
    #[doc = "SSI produces a steady state LOW value on the CLKOUT pin when data is not being transferred."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spo::Low
    }
}
#[doc = "Field `SPO` writer - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG, Spo>;
impl<'a, REG> SpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::High)
    }
    #[doc = "SSI produces a steady state LOW value on the CLKOUT pin when data is not being transferred."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::Low)
    }
}
#[doc = "7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sph {
    #[doc = "1: Data is captured on the second clock edge transition."]
    _2ndClkEdge = 1,
    #[doc = "0: Data is captured on the first clock edge transition."]
    _1stClkEdge = 0,
}
impl From<Sph> for bool {
    #[inline(always)]
    fn from(variant: Sph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPH` reader - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SphR = crate::BitReader<Sph>;
impl SphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sph {
        match self.bits {
            true => Sph::_2ndClkEdge,
            false => Sph::_1stClkEdge,
        }
    }
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn is_2nd_clk_edge(&self) -> bool {
        *self == Sph::_2ndClkEdge
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn is_1st_clk_edge(&self) -> bool {
        *self == Sph::_1stClkEdge
    }
}
#[doc = "Field `SPH` writer - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG, Sph>;
impl<'a, REG> SphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn _2nd_clk_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Sph::_2ndClkEdge)
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn _1st_clk_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Sph::_1stClkEdge)
    }
}
#[doc = "Field `SCR` reader - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DssW<Cr0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<Cr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SpoW<Cr0Spec> {
        SpoW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SphW<Cr0Spec> {
        SphW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<Cr0Spec> {
        ScrW::new(self, 8)
    }
}
#[doc = "Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {
    const RESET_VALUE: u32 = 0;
}
