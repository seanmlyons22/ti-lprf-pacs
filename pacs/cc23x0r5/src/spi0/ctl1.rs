#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "0:0\\]
SPI enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: SPI Enabled and released for operation."]
    Ena = 1,
    #[doc = "0: SPI is disabled"]
    Dis = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
SPI enable."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::Ena,
            false => En::Dis,
        }
    }
    #[doc = "SPI Enabled and released for operation."]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == En::Ena
    }
    #[doc = "SPI is disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
}
#[doc = "Field `EN` writer - 0:0\\]
SPI enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI Enabled and released for operation."]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(En::Ena)
    }
    #[doc = "SPI is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
}
#[doc = "1:1\\]
Loop back mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "1: Enable loopback mode. Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    Ena = 1,
    #[doc = "0: Disable loopback mode. Normal serial port operation enabled."]
    Dis = 0,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - 1:1\\]
Loop back mode control"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            true => Lbm::Ena,
            false => Lbm::Dis,
        }
    }
    #[doc = "Enable loopback mode. Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Lbm::Ena
    }
    #[doc = "Disable loopback mode. Normal serial port operation enabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lbm::Dis
    }
}
#[doc = "Field `LBM` writer - 1:1\\]
Loop back mode control"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable loopback mode. Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Ena)
    }
    #[doc = "Disable loopback mode. Normal serial port operation enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Dis)
    }
}
#[doc = "2:2\\]
Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.EN=0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ms {
    #[doc = "1: Select Controller mode"]
    Controller = 1,
    #[doc = "0: Select Peripheral mode"]
    Peripheral = 0,
}
impl From<Ms> for bool {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - 2:2\\]
Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.EN=0."]
pub type MsR = crate::BitReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ms {
        match self.bits {
            true => Ms::Controller,
            false => Ms::Peripheral,
        }
    }
    #[doc = "Select Controller mode"]
    #[inline(always)]
    pub fn is_controller(&self) -> bool {
        *self == Ms::Controller
    }
    #[doc = "Select Peripheral mode"]
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == Ms::Peripheral
    }
}
#[doc = "Field `MS` writer - 2:2\\]
Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.EN=0."]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Controller mode"]
    #[inline(always)]
    pub fn controller(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Controller)
    }
    #[doc = "Select Peripheral mode"]
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Peripheral)
    }
}
#[doc = "3:3\\]
Peripheral data output disable. This bit is relevant only in the peripheral mode, MS=1. In multiple-peripheral systems, it is possible for a SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the POCI lines from multiple peripherals could be tied together. To operate in such systems, this bit field can be set if the SPI peripheral is not supposed to drive the POCI output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pod {
    #[doc = "1: SPI cannot drive the POCI output in peripheral mode."]
    Ena = 1,
    #[doc = "0: SPI can drive the POCI output in peripheral mode."]
    Dis = 0,
}
impl From<Pod> for bool {
    #[inline(always)]
    fn from(variant: Pod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POD` reader - 3:3\\]
Peripheral data output disable. This bit is relevant only in the peripheral mode, MS=1. In multiple-peripheral systems, it is possible for a SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the POCI lines from multiple peripherals could be tied together. To operate in such systems, this bit field can be set if the SPI peripheral is not supposed to drive the POCI output."]
pub type PodR = crate::BitReader<Pod>;
impl PodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pod {
        match self.bits {
            true => Pod::Ena,
            false => Pod::Dis,
        }
    }
    #[doc = "SPI cannot drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Pod::Ena
    }
    #[doc = "SPI can drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pod::Dis
    }
}
#[doc = "Field `POD` writer - 3:3\\]
Peripheral data output disable. This bit is relevant only in the peripheral mode, MS=1. In multiple-peripheral systems, it is possible for a SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the POCI lines from multiple peripherals could be tied together. To operate in such systems, this bit field can be set if the SPI peripheral is not supposed to drive the POCI output."]
pub type PodW<'a, REG> = crate::BitWriter<'a, REG, Pod>;
impl<'a, REG> PodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI cannot drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Pod::Ena)
    }
    #[doc = "SPI can drive the POCI output in peripheral mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pod::Dis)
    }
}
#[doc = "4:4\\]
MSB first select. Controls the direction of receive and transmit shift register. MSB first configuration (MSB = 1) must be selected when CRC feature is used for SPI communication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msb {
    #[doc = "1: MSB first"]
    Msb = 1,
    #[doc = "0: LSB first"]
    Lsb = 0,
}
impl From<Msb> for bool {
    #[inline(always)]
    fn from(variant: Msb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSB` reader - 4:4\\]
MSB first select. Controls the direction of receive and transmit shift register. MSB first configuration (MSB = 1) must be selected when CRC feature is used for SPI communication."]
pub type MsbR = crate::BitReader<Msb>;
impl MsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msb {
        match self.bits {
            true => Msb::Msb,
            false => Msb::Lsb,
        }
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Msb::Msb
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Msb::Lsb
    }
}
#[doc = "Field `MSB` writer - 4:4\\]
MSB first select. Controls the direction of receive and transmit shift register. MSB first configuration (MSB = 1) must be selected when CRC feature is used for SPI communication."]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG, Msb>;
impl<'a, REG> MsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Msb)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Msb::Lsb)
    }
}
#[doc = "5:5\\]
Parity enable. If enabled the last bit will be used as parity to evaluate the correct reception of the previous bits. In case of parity mismatch the parity error flag RIS.PER will be set. This feature is available only in SPI controller mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    #[doc = "1: Enable Parity function"]
    Ena = 1,
    #[doc = "0: Disable Parity function"]
    Dis = 0,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - 5:5\\]
Parity enable. If enabled the last bit will be used as parity to evaluate the correct reception of the previous bits. In case of parity mismatch the parity error flag RIS.PER will be set. This feature is available only in SPI controller mode."]
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            true => Pen::Ena,
            false => Pen::Dis,
        }
    }
    #[doc = "Enable Parity function"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Pen::Ena
    }
    #[doc = "Disable Parity function"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pen::Dis
    }
}
#[doc = "Field `PEN` writer - 5:5\\]
Parity enable. If enabled the last bit will be used as parity to evaluate the correct reception of the previous bits. In case of parity mismatch the parity error flag RIS.PER will be set. This feature is available only in SPI controller mode."]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Parity function"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Ena)
    }
    #[doc = "Disable Parity function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Dis)
    }
}
#[doc = "6:6\\]
Even parity select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pes {
    #[doc = "1: Even Parity mode"]
    Even = 1,
    #[doc = "0: Odd Parity mode"]
    Odd = 0,
}
impl From<Pes> for bool {
    #[inline(always)]
    fn from(variant: Pes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - 6:6\\]
Even parity select."]
pub type PesR = crate::BitReader<Pes>;
impl PesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pes {
        match self.bits {
            true => Pes::Even,
            false => Pes::Odd,
        }
    }
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pes::Even
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pes::Odd
    }
}
#[doc = "Field `PES` writer - 6:6\\]
Even parity select."]
pub type PesW<'a, REG> = crate::BitWriter<'a, REG, Pes>;
impl<'a, REG> PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Even)
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Odd)
    }
}
#[doc = "7:7\\]
Parity bit select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbs {
    #[doc = "1: Bit 1 is used for Parity, Bit 0 is ignored"]
    Bit1 = 1,
    #[doc = "0: Bit 0 is used for Parity"]
    Bit0 = 0,
}
impl From<Pbs> for bool {
    #[inline(always)]
    fn from(variant: Pbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBS` reader - 7:7\\]
Parity bit select"]
pub type PbsR = crate::BitReader<Pbs>;
impl PbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbs {
        match self.bits {
            true => Pbs::Bit1,
            false => Pbs::Bit0,
        }
    }
    #[doc = "Bit 1 is used for Parity, Bit 0 is ignored"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == Pbs::Bit1
    }
    #[doc = "Bit 0 is used for Parity"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == Pbs::Bit0
    }
}
#[doc = "Field `PBS` writer - 7:7\\]
Parity bit select"]
pub type PbsW<'a, REG> = crate::BitWriter<'a, REG, Pbs>;
impl<'a, REG> PbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit 1 is used for Parity, Bit 0 is ignored"]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbs::Bit1)
    }
    #[doc = "Bit 0 is used for Parity"]
    #[inline(always)]
    pub fn bit0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbs::Bit0)
    }
}
#[doc = "Field `RESERVED8` reader - 10:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "11:11\\]
Command/Data mode enable. This feature is applicable only in controller mode and for 8-bit transfers (CTL0.DSS = 7). The chip select pin is used for command/data signaling in Motorola SPI frame format (3-wire) operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cden {
    #[doc = "1: C/D Mode Enable"]
    Ena = 1,
    #[doc = "0: C/D Mode Disable"]
    Dis = 0,
}
impl From<Cden> for bool {
    #[inline(always)]
    fn from(variant: Cden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN` reader - 11:11\\]
Command/Data mode enable. This feature is applicable only in controller mode and for 8-bit transfers (CTL0.DSS = 7). The chip select pin is used for command/data signaling in Motorola SPI frame format (3-wire) operation."]
pub type CdenR = crate::BitReader<Cden>;
impl CdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cden {
        match self.bits {
            true => Cden::Ena,
            false => Cden::Dis,
        }
    }
    #[doc = "C/D Mode Enable"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Cden::Ena
    }
    #[doc = "C/D Mode Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cden::Dis
    }
}
#[doc = "Field `CDEN` writer - 11:11\\]
Command/Data mode enable. This feature is applicable only in controller mode and for 8-bit transfers (CTL0.DSS = 7). The chip select pin is used for command/data signaling in Motorola SPI frame format (3-wire) operation."]
pub type CdenW<'a, REG> = crate::BitWriter<'a, REG, Cden>;
impl<'a, REG> CdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "C/D Mode Enable"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Cden::Ena)
    }
    #[doc = "C/D Mode Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cden::Dis)
    }
}
#[doc = "15:12\\]
Commnd Data Model. This bit field value determines the behavior of C/D or CS signal when CDEN = 1. CS pin held low indicates command phase and CS pin held high indicates data phase. When CDMODE = 0x0, the CS pin is always held high during transfer indicating data phase only operation (manual mode). When CDMODE = 0xF, the CS pin is always held low during transfer indicating command phase only operation (manual mode). When CDMODE = 0x1 to 0xE, the CS pin is held low for the number of bytes indicated by CDMODE value for the command phase and held high for the remaining transfers in the data phase (automatic mode). When CDMODE is set to value 0x1 to 0xE, reading CDMODE at any time during operation indicates the remaining bytes to be transferred in the command phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdmode {
    #[doc = "15: Manual mode: Command"]
    Command = 15,
    #[doc = "0: Manual mode: Data"]
    Data = 0,
}
impl From<Cdmode> for u8 {
    #[inline(always)]
    fn from(variant: Cdmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdmode {
    type Ux = u8;
}
impl crate::IsEnum for Cdmode {}
#[doc = "Field `CDMODE` reader - 15:12\\]
Commnd Data Model. This bit field value determines the behavior of C/D or CS signal when CDEN = 1. CS pin held low indicates command phase and CS pin held high indicates data phase. When CDMODE = 0x0, the CS pin is always held high during transfer indicating data phase only operation (manual mode). When CDMODE = 0xF, the CS pin is always held low during transfer indicating command phase only operation (manual mode). When CDMODE = 0x1 to 0xE, the CS pin is held low for the number of bytes indicated by CDMODE value for the command phase and held high for the remaining transfers in the data phase (automatic mode). When CDMODE is set to value 0x1 to 0xE, reading CDMODE at any time during operation indicates the remaining bytes to be transferred in the command phase."]
pub type CdmodeR = crate::FieldReader<Cdmode>;
impl CdmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdmode> {
        match self.bits {
            15 => Some(Cdmode::Command),
            0 => Some(Cdmode::Data),
            _ => None,
        }
    }
    #[doc = "Manual mode: Command"]
    #[inline(always)]
    pub fn is_command(&self) -> bool {
        *self == Cdmode::Command
    }
    #[doc = "Manual mode: Data"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Cdmode::Data
    }
}
#[doc = "Field `CDMODE` writer - 15:12\\]
Commnd Data Model. This bit field value determines the behavior of C/D or CS signal when CDEN = 1. CS pin held low indicates command phase and CS pin held high indicates data phase. When CDMODE = 0x0, the CS pin is always held high during transfer indicating data phase only operation (manual mode). When CDMODE = 0xF, the CS pin is always held low during transfer indicating command phase only operation (manual mode). When CDMODE = 0x1 to 0xE, the CS pin is held low for the number of bytes indicated by CDMODE value for the command phase and held high for the remaining transfers in the data phase (automatic mode). When CDMODE is set to value 0x1 to 0xE, reading CDMODE at any time during operation indicates the remaining bytes to be transferred in the command phase."]
pub type CdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cdmode>;
impl<'a, REG> CdmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual mode: Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut crate::W<REG> {
        self.variant(Cdmode::Command)
    }
    #[doc = "Manual mode: Data"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Cdmode::Data)
    }
}
#[doc = "23:16\\]
Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the provided value. The transfer will be started with writing a data into the TX FIFO. Sending the data will be repeated provided value number of times, so the data will be transferred x+1 times in total. The behavior would be as if the data were be written into the TX FIFO as many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reptx {
    #[doc = "0: REPTX disable"]
    Dis = 0,
}
impl From<Reptx> for u8 {
    #[inline(always)]
    fn from(variant: Reptx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reptx {
    type Ux = u8;
}
impl crate::IsEnum for Reptx {}
#[doc = "Field `REPTX` reader - 23:16\\]
Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the provided value. The transfer will be started with writing a data into the TX FIFO. Sending the data will be repeated provided value number of times, so the data will be transferred x+1 times in total. The behavior would be as if the data were be written into the TX FIFO as many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
pub type ReptxR = crate::FieldReader<Reptx>;
impl ReptxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Reptx> {
        match self.bits {
            0 => Some(Reptx::Dis),
            _ => None,
        }
    }
    #[doc = "REPTX disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Reptx::Dis
    }
}
#[doc = "Field `REPTX` writer - 23:16\\]
Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the provided value. The transfer will be started with writing a data into the TX FIFO. Sending the data will be repeated provided value number of times, so the data will be transferred x+1 times in total. The behavior would be as if the data were be written into the TX FIFO as many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
pub type ReptxW<'a, REG> = crate::FieldWriter<'a, REG, 8, Reptx>;
impl<'a, REG> ReptxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "REPTX disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Reptx::Dis)
    }
}
#[doc = "Field `RTOUT` reader - 29:24\\]
Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RIS.RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
pub type RtoutR = crate::FieldReader;
#[doc = "Field `RTOUT` writer - 29:24\\]
Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RIS.RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
pub type RtoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SPI enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop back mode control"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.EN=0."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Peripheral data output disable. This bit is relevant only in the peripheral mode, MS=1. In multiple-peripheral systems, it is possible for a SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the POCI lines from multiple peripherals could be tied together. To operate in such systems, this bit field can be set if the SPI peripheral is not supposed to drive the POCI output."]
    #[inline(always)]
    pub fn pod(&self) -> PodR {
        PodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MSB first select. Controls the direction of receive and transmit shift register. MSB first configuration (MSB = 1) must be selected when CRC feature is used for SPI communication."]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Parity enable. If enabled the last bit will be used as parity to evaluate the correct reception of the previous bits. In case of parity mismatch the parity error flag RIS.PER will be set. This feature is available only in SPI controller mode."]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Even parity select."]
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Parity bit select"]
    #[inline(always)]
    pub fn pbs(&self) -> PbsR {
        PbsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Command/Data mode enable. This feature is applicable only in controller mode and for 8-bit transfers (CTL0.DSS = 7). The chip select pin is used for command/data signaling in Motorola SPI frame format (3-wire) operation."]
    #[inline(always)]
    pub fn cden(&self) -> CdenR {
        CdenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Commnd Data Model. This bit field value determines the behavior of C/D or CS signal when CDEN = 1. CS pin held low indicates command phase and CS pin held high indicates data phase. When CDMODE = 0x0, the CS pin is always held high during transfer indicating data phase only operation (manual mode). When CDMODE = 0xF, the CS pin is always held low during transfer indicating command phase only operation (manual mode). When CDMODE = 0x1 to 0xE, the CS pin is held low for the number of bytes indicated by CDMODE value for the command phase and held high for the remaining transfers in the data phase (automatic mode). When CDMODE is set to value 0x1 to 0xE, reading CDMODE at any time during operation indicates the remaining bytes to be transferred in the command phase."]
    #[inline(always)]
    pub fn cdmode(&self) -> CdmodeR {
        CdmodeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the provided value. The transfer will be started with writing a data into the TX FIFO. Sending the data will be repeated provided value number of times, so the data will be transferred x+1 times in total. The behavior would be as if the data were be written into the TX FIFO as many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
    #[inline(always)]
    pub fn reptx(&self) -> ReptxR {
        ReptxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RIS.RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rtout(&self) -> RtoutR {
        RtoutR::new(((self.bits >> 24) & 0x3f) as u8)
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
SPI enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Ctl1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop back mode control"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<Ctl1Spec> {
        LbmW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<Ctl1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Peripheral data output disable. This bit is relevant only in the peripheral mode, MS=1. In multiple-peripheral systems, it is possible for a SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the POCI lines from multiple peripherals could be tied together. To operate in such systems, this bit field can be set if the SPI peripheral is not supposed to drive the POCI output."]
    #[inline(always)]
    #[must_use]
    pub fn pod(&mut self) -> PodW<Ctl1Spec> {
        PodW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
MSB first select. Controls the direction of receive and transmit shift register. MSB first configuration (MSB = 1) must be selected when CRC feature is used for SPI communication."]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MsbW<Ctl1Spec> {
        MsbW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Parity enable. If enabled the last bit will be used as parity to evaluate the correct reception of the previous bits. In case of parity mismatch the parity error flag RIS.PER will be set. This feature is available only in SPI controller mode."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<Ctl1Spec> {
        PenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Even parity select."]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PesW<Ctl1Spec> {
        PesW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Parity bit select"]
    #[inline(always)]
    #[must_use]
    pub fn pbs(&mut self) -> PbsW<Ctl1Spec> {
        PbsW::new(self, 7)
    }
    #[doc = "Bit 11 - 11:11\\]
Command/Data mode enable. This feature is applicable only in controller mode and for 8-bit transfers (CTL0.DSS = 7). The chip select pin is used for command/data signaling in Motorola SPI frame format (3-wire) operation."]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self) -> CdenW<Ctl1Spec> {
        CdenW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Commnd Data Model. This bit field value determines the behavior of C/D or CS signal when CDEN = 1. CS pin held low indicates command phase and CS pin held high indicates data phase. When CDMODE = 0x0, the CS pin is always held high during transfer indicating data phase only operation (manual mode). When CDMODE = 0xF, the CS pin is always held low during transfer indicating command phase only operation (manual mode). When CDMODE = 0x1 to 0xE, the CS pin is held low for the number of bytes indicated by CDMODE value for the command phase and held high for the remaining transfers in the data phase (automatic mode). When CDMODE is set to value 0x1 to 0xE, reading CDMODE at any time during operation indicates the remaining bytes to be transferred in the command phase."]
    #[inline(always)]
    #[must_use]
    pub fn cdmode(&mut self) -> CdmodeW<Ctl1Spec> {
        CdmodeW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the provided value. The transfer will be started with writing a data into the TX FIFO. Sending the data will be repeated provided value number of times, so the data will be transferred x+1 times in total. The behavior would be as if the data were be written into the TX FIFO as many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn reptx(&mut self) -> ReptxW<Ctl1Spec> {
        ReptxW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RIS.RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
    #[inline(always)]
    #[must_use]
    pub fn rtout(&mut self) -> RtoutW<Ctl1Spec> {
        RtoutW::new(self, 24)
    }
}
#[doc = "SPI control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
