#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "3:0\\]
Data size select. Valid DSS values for controller mode operation are 0x3 to 0xF and for peripheral mode operation are 0x6 to 0xF. DSS values 0x0 to 0x2 are reserved and shall not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "15: 16-bits data size"]
    Bits16 = 15,
    #[doc = "14: 15-bits data size"]
    Bits15 = 14,
    #[doc = "13: 14-bits data size"]
    Bits14 = 13,
    #[doc = "12: 13-bits data size"]
    Bits13 = 12,
    #[doc = "11: 12-bits data size"]
    Bits12 = 11,
    #[doc = "10: 11-bits data size"]
    Bits11 = 10,
    #[doc = "9: 10-bits data size"]
    Bits10 = 9,
    #[doc = "8: 9-bits data size"]
    Bits9 = 8,
    #[doc = "7: 8-bits data size"]
    Bits8 = 7,
    #[doc = "6: 7-bits data size"]
    Bits7 = 6,
    #[doc = "5: 6-bits data size"]
    Bits6 = 5,
    #[doc = "4: 5-bits data size"]
    Bits5 = 4,
    #[doc = "3: 4-bits data size"]
    Bits4 = 3,
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
Data size select. Valid DSS values for controller mode operation are 0x3 to 0xF and for peripheral mode operation are 0x6 to 0xF. DSS values 0x0 to 0x2 are reserved and shall not be used."]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            15 => Some(Dss::Bits16),
            14 => Some(Dss::Bits15),
            13 => Some(Dss::Bits14),
            12 => Some(Dss::Bits13),
            11 => Some(Dss::Bits12),
            10 => Some(Dss::Bits11),
            9 => Some(Dss::Bits10),
            8 => Some(Dss::Bits9),
            7 => Some(Dss::Bits8),
            6 => Some(Dss::Bits7),
            5 => Some(Dss::Bits6),
            4 => Some(Dss::Bits5),
            3 => Some(Dss::Bits4),
            _ => None,
        }
    }
    #[doc = "16-bits data size"]
    #[inline(always)]
    pub fn is_bits_16(&self) -> bool {
        *self == Dss::Bits16
    }
    #[doc = "15-bits data size"]
    #[inline(always)]
    pub fn is_bits_15(&self) -> bool {
        *self == Dss::Bits15
    }
    #[doc = "14-bits data size"]
    #[inline(always)]
    pub fn is_bits_14(&self) -> bool {
        *self == Dss::Bits14
    }
    #[doc = "13-bits data size"]
    #[inline(always)]
    pub fn is_bits_13(&self) -> bool {
        *self == Dss::Bits13
    }
    #[doc = "12-bits data size"]
    #[inline(always)]
    pub fn is_bits_12(&self) -> bool {
        *self == Dss::Bits12
    }
    #[doc = "11-bits data size"]
    #[inline(always)]
    pub fn is_bits_11(&self) -> bool {
        *self == Dss::Bits11
    }
    #[doc = "10-bits data size"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        *self == Dss::Bits10
    }
    #[doc = "9-bits data size"]
    #[inline(always)]
    pub fn is_bits_9(&self) -> bool {
        *self == Dss::Bits9
    }
    #[doc = "8-bits data size"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == Dss::Bits8
    }
    #[doc = "7-bits data size"]
    #[inline(always)]
    pub fn is_bits_7(&self) -> bool {
        *self == Dss::Bits7
    }
    #[doc = "6-bits data size"]
    #[inline(always)]
    pub fn is_bits_6(&self) -> bool {
        *self == Dss::Bits6
    }
    #[doc = "5-bits data size"]
    #[inline(always)]
    pub fn is_bits_5(&self) -> bool {
        *self == Dss::Bits5
    }
    #[doc = "4-bits data size"]
    #[inline(always)]
    pub fn is_bits_4(&self) -> bool {
        *self == Dss::Bits4
    }
}
#[doc = "Field `DSS` writer - 3:0\\]
Data size select. Valid DSS values for controller mode operation are 0x3 to 0xF and for peripheral mode operation are 0x6 to 0xF. DSS values 0x0 to 0x2 are reserved and shall not be used."]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bits data size"]
    #[inline(always)]
    pub fn bits_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits16)
    }
    #[doc = "15-bits data size"]
    #[inline(always)]
    pub fn bits_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits15)
    }
    #[doc = "14-bits data size"]
    #[inline(always)]
    pub fn bits_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits14)
    }
    #[doc = "13-bits data size"]
    #[inline(always)]
    pub fn bits_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits13)
    }
    #[doc = "12-bits data size"]
    #[inline(always)]
    pub fn bits_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits12)
    }
    #[doc = "11-bits data size"]
    #[inline(always)]
    pub fn bits_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits11)
    }
    #[doc = "10-bits data size"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits10)
    }
    #[doc = "9-bits data size"]
    #[inline(always)]
    pub fn bits_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits9)
    }
    #[doc = "8-bits data size"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits8)
    }
    #[doc = "7-bits data size"]
    #[inline(always)]
    pub fn bits_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits7)
    }
    #[doc = "6-bits data size"]
    #[inline(always)]
    pub fn bits_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits6)
    }
    #[doc = "5-bits data size"]
    #[inline(always)]
    pub fn bits_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits5)
    }
    #[doc = "4-bits data size"]
    #[inline(always)]
    pub fn bits_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::Bits4)
    }
}
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::BitReader;
#[doc = "6:5\\]
Frame format select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "3: National Microwire frame format"]
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
Frame format select"]
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
    #[doc = "National Microwire frame format"]
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
Frame format select"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf, crate::Safe>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "National Microwire frame format"]
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
#[doc = "8:8\\]
CLKOUT polarity (Motorola SPI frame format only).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spo {
    #[doc = "1: SPI produces a steady state HIGH value on the CLKOUT"]
    High = 1,
    #[doc = "0: SPI produces a steady state LOW value on the CLKOUT"]
    Low = 0,
}
impl From<Spo> for bool {
    #[inline(always)]
    fn from(variant: Spo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPO` reader - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)."]
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
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spo::High
    }
    #[doc = "SPI produces a steady state LOW value on the CLKOUT"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spo::Low
    }
}
#[doc = "Field `SPO` writer - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)."]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG, Spo>;
impl<'a, REG> SpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI produces a steady state HIGH value on the CLKOUT"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::High)
    }
    #[doc = "SPI produces a steady state LOW value on the CLKOUT"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spo::Low)
    }
}
#[doc = "9:9\\]
CLKOUT phase (Motorola SPI frame format only). This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture clock edge.\n\nValue on reset: 0"]
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
CLKOUT phase (Motorola SPI frame format only). This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture clock edge."]
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
CLKOUT phase (Motorola SPI frame format only). This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture clock edge."]
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
#[doc = "10:10\\]
Hardware controlled chip select (CS) value. When set CS is zero till TX FIFO is empty, as in - a. CS is de-asserted b. All data bytes are transmitted c. CS is asserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwcsn {
    #[doc = "1: HWCS Enable"]
    Ena = 1,
    #[doc = "0: HWCS Disable"]
    Dis = 0,
}
impl From<Hwcsn> for bool {
    #[inline(always)]
    fn from(variant: Hwcsn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWCSN` reader - 10:10\\]
Hardware controlled chip select (CS) value. When set CS is zero till TX FIFO is empty, as in - a. CS is de-asserted b. All data bytes are transmitted c. CS is asserted"]
pub type HwcsnR = crate::BitReader<Hwcsn>;
impl HwcsnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwcsn {
        match self.bits {
            true => Hwcsn::Ena,
            false => Hwcsn::Dis,
        }
    }
    #[doc = "HWCS Enable"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Hwcsn::Ena
    }
    #[doc = "HWCS Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hwcsn::Dis
    }
}
#[doc = "Field `HWCSN` writer - 10:10\\]
Hardware controlled chip select (CS) value. When set CS is zero till TX FIFO is empty, as in - a. CS is de-asserted b. All data bytes are transmitted c. CS is asserted"]
pub type HwcsnW<'a, REG> = crate::BitWriter<'a, REG, Hwcsn>;
impl<'a, REG> HwcsnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HWCS Enable"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Hwcsn::Ena)
    }
    #[doc = "HWCS Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hwcsn::Dis)
    }
}
#[doc = "11:11\\]
This bit is used to reset transmit and receive FIFO pointers. This bit is auto cleared once the FIFO pointer reset operation is completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fiforst {
    #[doc = "1: Trigger FIFO pointers reset when written to 1."]
    RstTrig = 1,
    #[doc = "0: FIFO pointers reset completed when 0 is read"]
    RstDone = 0,
}
impl From<Fiforst> for bool {
    #[inline(always)]
    fn from(variant: Fiforst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFORST` reader - 11:11\\]
This bit is used to reset transmit and receive FIFO pointers. This bit is auto cleared once the FIFO pointer reset operation is completed."]
pub type FiforstR = crate::BitReader<Fiforst>;
impl FiforstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fiforst {
        match self.bits {
            true => Fiforst::RstTrig,
            false => Fiforst::RstDone,
        }
    }
    #[doc = "Trigger FIFO pointers reset when written to 1."]
    #[inline(always)]
    pub fn is_rst_trig(&self) -> bool {
        *self == Fiforst::RstTrig
    }
    #[doc = "FIFO pointers reset completed when 0 is read"]
    #[inline(always)]
    pub fn is_rst_done(&self) -> bool {
        *self == Fiforst::RstDone
    }
}
#[doc = "Field `FIFORST` writer - 11:11\\]
This bit is used to reset transmit and receive FIFO pointers. This bit is auto cleared once the FIFO pointer reset operation is completed."]
pub type FiforstW<'a, REG> = crate::BitWriter<'a, REG, Fiforst>;
impl<'a, REG> FiforstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger FIFO pointers reset when written to 1."]
    #[inline(always)]
    pub fn rst_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Fiforst::RstTrig)
    }
    #[doc = "FIFO pointers reset completed when 0 is read"]
    #[inline(always)]
    pub fn rst_done(self) -> &'a mut crate::W<REG> {
        self.variant(Fiforst::RstDone)
    }
}
#[doc = "12:12\\]
Clear shift register counter on CS inactive. This bit is relevant only in the peripheral mode, when CTL1.MS=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csclr {
    #[doc = "1: Enable automatic clear of shift register when CS goes inactive."]
    Ena = 1,
    #[doc = "0: Disable automatic clear of shift register when CS goes inactive."]
    Dis = 0,
}
impl From<Csclr> for bool {
    #[inline(always)]
    fn from(variant: Csclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCLR` reader - 12:12\\]
Clear shift register counter on CS inactive. This bit is relevant only in the peripheral mode, when CTL1.MS=0."]
pub type CsclrR = crate::BitReader<Csclr>;
impl CsclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csclr {
        match self.bits {
            true => Csclr::Ena,
            false => Csclr::Dis,
        }
    }
    #[doc = "Enable automatic clear of shift register when CS goes inactive."]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Csclr::Ena
    }
    #[doc = "Disable automatic clear of shift register when CS goes inactive."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Csclr::Dis
    }
}
#[doc = "Field `CSCLR` writer - 12:12\\]
Clear shift register counter on CS inactive. This bit is relevant only in the peripheral mode, when CTL1.MS=0."]
pub type CsclrW<'a, REG> = crate::BitWriter<'a, REG, Csclr>;
impl<'a, REG> CsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable automatic clear of shift register when CS goes inactive."]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::Ena)
    }
    #[doc = "Disable automatic clear of shift register when CS goes inactive."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::Dis)
    }
}
#[doc = "13:13\\]
CRC16 Endianness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcEnd {
    #[doc = "1: Auto-insertion of CRC16 is least-significant byte first"]
    CrcEndLsb = 1,
    #[doc = "0: Auto-insertion of CRC16 is most-significant byte first"]
    CrcEndMsb = 0,
}
impl From<CrcEnd> for bool {
    #[inline(always)]
    fn from(variant: CrcEnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_END` reader - 13:13\\]
CRC16 Endianness"]
pub type CrcEndR = crate::BitReader<CrcEnd>;
impl CrcEndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcEnd {
        match self.bits {
            true => CrcEnd::CrcEndLsb,
            false => CrcEnd::CrcEndMsb,
        }
    }
    #[doc = "Auto-insertion of CRC16 is least-significant byte first"]
    #[inline(always)]
    pub fn is_crc_end_lsb(&self) -> bool {
        *self == CrcEnd::CrcEndLsb
    }
    #[doc = "Auto-insertion of CRC16 is most-significant byte first"]
    #[inline(always)]
    pub fn is_crc_end_msb(&self) -> bool {
        *self == CrcEnd::CrcEndMsb
    }
}
#[doc = "Field `CRC_END` writer - 13:13\\]
CRC16 Endianness"]
pub type CrcEndW<'a, REG> = crate::BitWriter<'a, REG, CrcEnd>;
impl<'a, REG> CrcEndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-insertion of CRC16 is least-significant byte first"]
    #[inline(always)]
    pub fn crc_end_lsb(self) -> &'a mut crate::W<REG> {
        self.variant(CrcEnd::CrcEndLsb)
    }
    #[doc = "Auto-insertion of CRC16 is most-significant byte first"]
    #[inline(always)]
    pub fn crc_end_msb(self) -> &'a mut crate::W<REG> {
        self.variant(CrcEnd::CrcEndMsb)
    }
}
#[doc = "14:14\\]
Auto insert CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoCrc {
    #[doc = "1: Insert CRC into TXFIFO upon TXFIFO underflow"]
    Ena = 1,
    #[doc = "0: Do not insert CRC into TXFIFO upon TXFIFO underflow"]
    Dis = 0,
}
impl From<AutoCrc> for bool {
    #[inline(always)]
    fn from(variant: AutoCrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_CRC` reader - 14:14\\]
Auto insert CRC"]
pub type AutoCrcR = crate::BitReader<AutoCrc>;
impl AutoCrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoCrc {
        match self.bits {
            true => AutoCrc::Ena,
            false => AutoCrc::Dis,
        }
    }
    #[doc = "Insert CRC into TXFIFO upon TXFIFO underflow"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == AutoCrc::Ena
    }
    #[doc = "Do not insert CRC into TXFIFO upon TXFIFO underflow"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AutoCrc::Dis
    }
}
#[doc = "Field `AUTO_CRC` writer - 14:14\\]
Auto insert CRC"]
pub type AutoCrcW<'a, REG> = crate::BitWriter<'a, REG, AutoCrc>;
impl<'a, REG> AutoCrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Insert CRC into TXFIFO upon TXFIFO underflow"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(AutoCrc::Ena)
    }
    #[doc = "Do not insert CRC into TXFIFO upon TXFIFO underflow"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AutoCrc::Dis)
    }
}
#[doc = "15:15\\]
CRC polynomial selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcpoly {
    #[doc = "1: Selects 16-bit CCITT CRC polynomial"]
    Bit16 = 1,
    #[doc = "0: Selects 8-bit CCITT CRC polynomial"]
    Bit8 = 0,
}
impl From<Crcpoly> for bool {
    #[inline(always)]
    fn from(variant: Crcpoly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCPOLY` reader - 15:15\\]
CRC polynomial selection."]
pub type CrcpolyR = crate::BitReader<Crcpoly>;
impl CrcpolyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcpoly {
        match self.bits {
            true => Crcpoly::Bit16,
            false => Crcpoly::Bit8,
        }
    }
    #[doc = "Selects 16-bit CCITT CRC polynomial"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == Crcpoly::Bit16
    }
    #[doc = "Selects 8-bit CCITT CRC polynomial"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == Crcpoly::Bit8
    }
}
#[doc = "Field `CRCPOLY` writer - 15:15\\]
CRC polynomial selection."]
pub type CrcpolyW<'a, REG> = crate::BitWriter<'a, REG, Crcpoly>;
impl<'a, REG> CrcpolyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects 16-bit CCITT CRC polynomial"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(Crcpoly::Bit16)
    }
    #[doc = "Selects 8-bit CCITT CRC polynomial"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(Crcpoly::Bit8)
    }
}
#[doc = "16:16\\]
General purpose CRC enable. This bit enables transmit side CRC unit for general purpose use by software when SPI is disabled (CTL1.EN = 0). This bit must be 0 when SPI is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpcrcen {
    #[doc = "1: Transmit side CRC unit is available for general purpose software use"]
    Ena = 1,
    #[doc = "0: Transmit side CRC unit is not available for general purpose software use"]
    Dis = 0,
}
impl From<Gpcrcen> for bool {
    #[inline(always)]
    fn from(variant: Gpcrcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPCRCEN` reader - 16:16\\]
General purpose CRC enable. This bit enables transmit side CRC unit for general purpose use by software when SPI is disabled (CTL1.EN = 0). This bit must be 0 when SPI is enabled."]
pub type GpcrcenR = crate::BitReader<Gpcrcen>;
impl GpcrcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpcrcen {
        match self.bits {
            true => Gpcrcen::Ena,
            false => Gpcrcen::Dis,
        }
    }
    #[doc = "Transmit side CRC unit is available for general purpose software use"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Gpcrcen::Ena
    }
    #[doc = "Transmit side CRC unit is not available for general purpose software use"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Gpcrcen::Dis
    }
}
#[doc = "Field `GPCRCEN` writer - 16:16\\]
General purpose CRC enable. This bit enables transmit side CRC unit for general purpose use by software when SPI is disabled (CTL1.EN = 0). This bit must be 0 when SPI is enabled."]
pub type GpcrcenW<'a, REG> = crate::BitWriter<'a, REG, Gpcrcen>;
impl<'a, REG> GpcrcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit side CRC unit is available for general purpose software use"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Gpcrcen::Ena)
    }
    #[doc = "Transmit side CRC unit is not available for general purpose software use"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Gpcrcen::Dis)
    }
}
#[doc = "17:17\\]
The Idle value of POCI - when TXFIFO is empty and before data is writtern into TXFIFO - can be controlled by this field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PociIdleval {
    #[doc = "1: POCI outputs idle value of '1'"]
    IdleOne = 1,
    #[doc = "0: POCI output idle value of '0'"]
    IdleZero = 0,
}
impl From<PociIdleval> for bool {
    #[inline(always)]
    fn from(variant: PociIdleval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POCI_IDLEVAL` reader - 17:17\\]
The Idle value of POCI - when TXFIFO is empty and before data is writtern into TXFIFO - can be controlled by this field."]
pub type PociIdlevalR = crate::BitReader<PociIdleval>;
impl PociIdlevalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PociIdleval {
        match self.bits {
            true => PociIdleval::IdleOne,
            false => PociIdleval::IdleZero,
        }
    }
    #[doc = "POCI outputs idle value of '1'"]
    #[inline(always)]
    pub fn is_idle_one(&self) -> bool {
        *self == PociIdleval::IdleOne
    }
    #[doc = "POCI output idle value of '0'"]
    #[inline(always)]
    pub fn is_idle_zero(&self) -> bool {
        *self == PociIdleval::IdleZero
    }
}
#[doc = "Field `POCI_IDLEVAL` writer - 17:17\\]
The Idle value of POCI - when TXFIFO is empty and before data is writtern into TXFIFO - can be controlled by this field."]
pub type PociIdlevalW<'a, REG> = crate::BitWriter<'a, REG, PociIdleval>;
impl<'a, REG> PociIdlevalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POCI outputs idle value of '1'"]
    #[inline(always)]
    pub fn idle_one(self) -> &'a mut crate::W<REG> {
        self.variant(PociIdleval::IdleOne)
    }
    #[doc = "POCI output idle value of '0'"]
    #[inline(always)]
    pub fn idle_zero(self) -> &'a mut crate::W<REG> {
        self.variant(PociIdleval::IdleZero)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Data size select. Valid DSS values for controller mode operation are 0x3 to 0xF and for peripheral mode operation are 0x6 to 0xF. DSS values 0x0 to 0x2 are reserved and shall not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Frame format select"]
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
CLKOUT polarity (Motorola SPI frame format only)."]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
CLKOUT phase (Motorola SPI frame format only). This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture clock edge."]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Hardware controlled chip select (CS) value. When set CS is zero till TX FIFO is empty, as in - a. CS is de-asserted b. All data bytes are transmitted c. CS is asserted"]
    #[inline(always)]
    pub fn hwcsn(&self) -> HwcsnR {
        HwcsnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
This bit is used to reset transmit and receive FIFO pointers. This bit is auto cleared once the FIFO pointer reset operation is completed."]
    #[inline(always)]
    pub fn fiforst(&self) -> FiforstR {
        FiforstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear shift register counter on CS inactive. This bit is relevant only in the peripheral mode, when CTL1.MS=0."]
    #[inline(always)]
    pub fn csclr(&self) -> CsclrR {
        CsclrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
CRC16 Endianness"]
    #[inline(always)]
    pub fn crc_end(&self) -> CrcEndR {
        CrcEndR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Auto insert CRC"]
    #[inline(always)]
    pub fn auto_crc(&self) -> AutoCrcR {
        AutoCrcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
CRC polynomial selection."]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
General purpose CRC enable. This bit enables transmit side CRC unit for general purpose use by software when SPI is disabled (CTL1.EN = 0). This bit must be 0 when SPI is enabled."]
    #[inline(always)]
    pub fn gpcrcen(&self) -> GpcrcenR {
        GpcrcenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
The Idle value of POCI - when TXFIFO is empty and before data is writtern into TXFIFO - can be controlled by this field."]
    #[inline(always)]
    pub fn poci_idleval(&self) -> PociIdlevalR {
        PociIdlevalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Data size select. Valid DSS values for controller mode operation are 0x3 to 0xF and for peripheral mode operation are 0x6 to 0xF. DSS values 0x0 to 0x2 are reserved and shall not be used."]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DssW<Ctl0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Frame format select"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<Ctl0Spec> {
        FrfW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
CLKOUT polarity (Motorola SPI frame format only)."]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SpoW<Ctl0Spec> {
        SpoW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
CLKOUT phase (Motorola SPI frame format only). This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture clock edge."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SphW<Ctl0Spec> {
        SphW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Hardware controlled chip select (CS) value. When set CS is zero till TX FIFO is empty, as in - a. CS is de-asserted b. All data bytes are transmitted c. CS is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn hwcsn(&mut self) -> HwcsnW<Ctl0Spec> {
        HwcsnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
This bit is used to reset transmit and receive FIFO pointers. This bit is auto cleared once the FIFO pointer reset operation is completed."]
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FiforstW<Ctl0Spec> {
        FiforstW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear shift register counter on CS inactive. This bit is relevant only in the peripheral mode, when CTL1.MS=0."]
    #[inline(always)]
    #[must_use]
    pub fn csclr(&mut self) -> CsclrW<Ctl0Spec> {
        CsclrW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
CRC16 Endianness"]
    #[inline(always)]
    #[must_use]
    pub fn crc_end(&mut self) -> CrcEndW<Ctl0Spec> {
        CrcEndW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Auto insert CRC"]
    #[inline(always)]
    #[must_use]
    pub fn auto_crc(&mut self) -> AutoCrcW<Ctl0Spec> {
        AutoCrcW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
CRC polynomial selection."]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CrcpolyW<Ctl0Spec> {
        CrcpolyW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
General purpose CRC enable. This bit enables transmit side CRC unit for general purpose use by software when SPI is disabled (CTL1.EN = 0). This bit must be 0 when SPI is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpcrcen(&mut self) -> GpcrcenW<Ctl0Spec> {
        GpcrcenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
The Idle value of POCI - when TXFIFO is empty and before data is writtern into TXFIFO - can be controlled by this field."]
    #[inline(always)]
    #[must_use]
    pub fn poci_idleval(&mut self) -> PociIdlevalW<Ctl0Spec> {
        PociIdlevalW::new(self, 17)
    }
}
#[doc = "SPI control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
