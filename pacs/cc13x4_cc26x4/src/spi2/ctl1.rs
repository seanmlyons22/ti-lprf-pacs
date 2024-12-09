#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "0:0\\]
SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: Enable module function"]
    Enable = 1,
    #[doc = "0: Disable module function"]
    Disable = 0,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
SPI enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            true => Enable::Enable,
            false => Enable::Disable,
        }
    }
    #[doc = "Enable module function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
    #[doc = "Disable module function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
SPI enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable module function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
    #[doc = "Disable module function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
}
#[doc = "1:1\\]
Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "1: Enable loopback mode.Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    Enable = 1,
    #[doc = "0: Disable loopback mode. Normal serial port operation enabled."]
    Disable = 0,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - 1:1\\]
Loop back mode"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            true => Lbm::Enable,
            false => Lbm::Disable,
        }
    }
    #[doc = "Enable loopback mode.Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbm::Enable
    }
    #[doc = "Disable loopback mode. Normal serial port operation enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbm::Disable
    }
}
#[doc = "Field `LBM` writer - 1:1\\]
Loop back mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable loopback mode.Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Enable)
    }
    #[doc = "Disable loopback mode. Normal serial port operation enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Disable)
    }
}
#[doc = "2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ms {
    #[doc = "1: Select Master Mode"]
    Enable = 1,
    #[doc = "0: Select Slave Mode"]
    Disable = 0,
}
impl From<Ms> for bool {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
pub type MsR = crate::BitReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ms {
        match self.bits {
            true => Ms::Enable,
            false => Ms::Disable,
        }
    }
    #[doc = "Select Master Mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ms::Enable
    }
    #[doc = "Select Slave Mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ms::Disable
    }
}
#[doc = "Field `MS` writer - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Master Mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Enable)
    }
    #[doc = "Select Slave Mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Disable)
    }
}
#[doc = "3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sod {
    #[doc = "1: SPI cannot drive the MISO output via TX in slave mode."]
    Enable = 1,
    #[doc = "0: SPI can drive the MISO output via TX in slave mode."]
    Disable = 0,
}
impl From<Sod> for bool {
    #[inline(always)]
    fn from(variant: Sod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOD` reader - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
pub type SodR = crate::BitReader<Sod>;
impl SodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sod {
        match self.bits {
            true => Sod::Enable,
            false => Sod::Disable,
        }
    }
    #[doc = "SPI cannot drive the MISO output via TX in slave mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sod::Enable
    }
    #[doc = "SPI can drive the MISO output via TX in slave mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sod::Disable
    }
}
#[doc = "Field `SOD` writer - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG, Sod>;
impl<'a, REG> SodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI cannot drive the MISO output via TX in slave mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sod::Enable)
    }
    #[doc = "SPI can drive the MISO output via TX in slave mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sod::Disable)
    }
}
#[doc = "4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msb {
    #[doc = "1: MSB first"]
    Enable = 1,
    #[doc = "0: LSB first"]
    Disable = 0,
}
impl From<Msb> for bool {
    #[inline(always)]
    fn from(variant: Msb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSB` reader - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
pub type MsbR = crate::BitReader<Msb>;
impl MsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msb {
        match self.bits {
            true => Msb::Enable,
            false => Msb::Disable,
        }
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Msb::Enable
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Msb::Disable
    }
}
#[doc = "Field `MSB` writer - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG, Msb>;
impl<'a, REG> MsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Enable)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Disable)
    }
}
#[doc = "5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    #[doc = "1: Enable Parity function"]
    Enable = 1,
    #[doc = "0: Disable Parity function"]
    Disable = 0,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            true => Pen::Enable,
            false => Pen::Disable,
        }
    }
    #[doc = "Enable Parity function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pen::Enable
    }
    #[doc = "Disable Parity function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pen::Disable
    }
}
#[doc = "Field `PEN` writer - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Parity function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Enable)
    }
    #[doc = "Disable Parity function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Disable)
    }
}
#[doc = "6:6\\]
Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pes {
    #[doc = "1: Even Parity mode"]
    Enable = 1,
    #[doc = "0: Odd Parity mode"]
    Disable = 0,
}
impl From<Pes> for bool {
    #[inline(always)]
    fn from(variant: Pes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - 6:6\\]
Even Parity Select"]
pub type PesR = crate::BitReader<Pes>;
impl PesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pes {
        match self.bits {
            true => Pes::Enable,
            false => Pes::Disable,
        }
    }
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pes::Enable
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pes::Disable
    }
}
#[doc = "Field `PES` writer - 6:6\\]
Even Parity Select"]
pub type PesW<'a, REG> = crate::BitWriter<'a, REG, Pes>;
impl<'a, REG> PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Enable)
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Disable)
    }
}
#[doc = "7:7\\]
Parity Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbs {
    #[doc = "1: Bit 1 is used for Parity, Bit 0 is ignored"]
    Enable = 1,
    #[doc = "0: Bit 0 is used for Parity"]
    Disable = 0,
}
impl From<Pbs> for bool {
    #[inline(always)]
    fn from(variant: Pbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBS` reader - 7:7\\]
Parity Bit Select"]
pub type PbsR = crate::BitReader<Pbs>;
impl PbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbs {
        match self.bits {
            true => Pbs::Enable,
            false => Pbs::Disable,
        }
    }
    #[doc = "Bit 1 is used for Parity, Bit 0 is ignored"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pbs::Enable
    }
    #[doc = "Bit 0 is used for Parity"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pbs::Disable
    }
}
#[doc = "Field `PBS` writer - 7:7\\]
Parity Bit Select"]
pub type PbsW<'a, REG> = crate::BitWriter<'a, REG, Pbs>;
impl<'a, REG> PbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit 1 is used for Parity, Bit 0 is ignored"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pbs::Enable)
    }
    #[doc = "Bit 0 is used for Parity"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pbs::Disable)
    }
}
#[doc = "Field `RESERVED8` reader - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `RESERVED8` writer - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fiforst {
    #[doc = "1: Set FIFO pointers reset trigger"]
    Set = 1,
    #[doc = "0: Clear FIFO pointers reset trigger"]
    Clr = 0,
}
impl From<Fiforst> for bool {
    #[inline(always)]
    fn from(variant: Fiforst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFORST` reader - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
pub type FiforstR = crate::BitReader<Fiforst>;
impl FiforstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fiforst {
        match self.bits {
            true => Fiforst::Set,
            false => Fiforst::Clr,
        }
    }
    #[doc = "Set FIFO pointers reset trigger"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Fiforst::Set
    }
    #[doc = "Clear FIFO pointers reset trigger"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Fiforst::Clr
    }
}
#[doc = "Field `FIFORST` writer - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
pub type FiforstW<'a, REG> = crate::BitWriter<'a, REG, Fiforst>;
impl<'a, REG> FiforstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set FIFO pointers reset trigger"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Fiforst::Set)
    }
    #[doc = "Clear FIFO pointers reset trigger"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Fiforst::Clr)
    }
}
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Repeattx {
    #[doc = "0: REPEATTX disable"]
    Disable = 0,
}
impl From<Repeattx> for u8 {
    #[inline(always)]
    fn from(variant: Repeattx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Repeattx {
    type Ux = u8;
}
impl crate::IsEnum for Repeattx {}
#[doc = "Field `REPEATTX` reader - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
pub type RepeattxR = crate::FieldReader<Repeattx>;
impl RepeattxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Repeattx> {
        match self.bits {
            0 => Some(Repeattx::Disable),
            _ => None,
        }
    }
    #[doc = "REPEATTX disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Repeattx::Disable
    }
}
#[doc = "Field `REPEATTX` writer - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
pub type RepeattxW<'a, REG> = crate::FieldWriter<'a, REG, 8, Repeattx>;
impl<'a, REG> RepeattxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "REPEATTX disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Repeattx::Disable)
    }
}
#[doc = "29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtimeout {
    #[doc = "63: Highest possible value"]
    Maximum = 63,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Rxtimeout> for u8 {
    #[inline(always)]
    fn from(variant: Rxtimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtimeout {
    type Ux = u8;
}
impl crate::IsEnum for Rxtimeout {}
#[doc = "Field `RXTIMEOUT` reader - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
pub type RxtimeoutR = crate::FieldReader<Rxtimeout>;
impl RxtimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxtimeout> {
        match self.bits {
            63 => Some(Rxtimeout::Maximum),
            0 => Some(Rxtimeout::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Rxtimeout::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Rxtimeout::Minimum
    }
}
#[doc = "Field `RXTIMEOUT` writer - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
pub type RxtimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 6, Rxtimeout>;
impl<'a, REG> RxtimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Maximum)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Minimum)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SPI enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop back mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Even Parity Select"]
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Parity Bit Select"]
    #[inline(always)]
    pub fn pbs(&self) -> PbsR {
        PbsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
    #[inline(always)]
    pub fn fiforst(&self) -> FiforstR {
        FiforstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
    #[inline(always)]
    pub fn repeattx(&self) -> RepeattxR {
        RepeattxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rxtimeout(&self) -> RxtimeoutR {
        RxtimeoutR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Ctl1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<Ctl1Spec> {
        LbmW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<Ctl1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SodW<Ctl1Spec> {
        SodW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MsbW<Ctl1Spec> {
        MsbW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<Ctl1Spec> {
        PenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PesW<Ctl1Spec> {
        PesW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Parity Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn pbs(&mut self) -> PbsW<Ctl1Spec> {
        PbsW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Ctl1Spec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FiforstW<Ctl1Spec> {
        FiforstW::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<Ctl1Spec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
    #[inline(always)]
    #[must_use]
    pub fn repeattx(&mut self) -> RepeattxW<Ctl1Spec> {
        RepeattxW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
    #[inline(always)]
    #[must_use]
    pub fn rxtimeout(&mut self) -> RxtimeoutW<Ctl1Spec> {
        RxtimeoutW::new(self, 24)
    }
}
#[doc = "SPI Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x04"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x04;
}
