#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
SPI enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "0:0\\]
SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "1: Enable module function"]
    ENABLE = 1,
    #[doc = "0: Disable module function"]
    DISABLE = 0,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            true => ENABLE_A::ENABLE,
            false => ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
SPI enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Enable module function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
    }
    #[doc = "Disable module function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
}
#[doc = "Field `LBM` reader - 1:1\\]
Loop back mode"]
pub type LBM_R = crate::BitReader<LBM_A>;
#[doc = "1:1\\]
Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBM_A {
    #[doc = "1: Enable loopback mode.Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    ENABLE = 1,
    #[doc = "0: Disable loopback mode. Normal serial port operation enabled."]
    DISABLE = 0,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
impl LBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBM_A {
        match self.bits {
            true => LBM_A::ENABLE,
            false => LBM_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LBM_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LBM_A::DISABLE
    }
}
#[doc = "Field `LBM` writer - 1:1\\]
Loop back mode"]
pub type LBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, LBM_A, O>;
impl<'a, const O: u8> LBM_W<'a, O> {
    #[doc = "Enable loopback mode.Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LBM_A::ENABLE)
    }
    #[doc = "Disable loopback mode. Normal serial port operation enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LBM_A::DISABLE)
    }
}
#[doc = "Field `MS` reader - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
pub type MS_R = crate::BitReader<MS_A>;
#[doc = "2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MS_A {
    #[doc = "1: Select Master Mode"]
    ENABLE = 1,
    #[doc = "0: Select Slave Mode"]
    DISABLE = 0,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
impl MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            true => MS_A::ENABLE,
            false => MS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MS_A::DISABLE
    }
}
#[doc = "Field `MS` writer - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
pub type MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, MS_A, O>;
impl<'a, const O: u8> MS_W<'a, O> {
    #[doc = "Select Master Mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MS_A::ENABLE)
    }
    #[doc = "Select Slave Mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MS_A::DISABLE)
    }
}
#[doc = "Field `SOD` reader - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
pub type SOD_R = crate::BitReader<SOD_A>;
#[doc = "3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOD_A {
    #[doc = "1: SPI cannot drive the MISO output via TX in slave mode."]
    ENABLE = 1,
    #[doc = "0: SPI can drive the MISO output via TX in slave mode."]
    DISABLE = 0,
}
impl From<SOD_A> for bool {
    #[inline(always)]
    fn from(variant: SOD_A) -> Self {
        variant as u8 != 0
    }
}
impl SOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOD_A {
        match self.bits {
            true => SOD_A::ENABLE,
            false => SOD_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOD_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOD_A::DISABLE
    }
}
#[doc = "Field `SOD` writer - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
pub type SOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, SOD_A, O>;
impl<'a, const O: u8> SOD_W<'a, O> {
    #[doc = "SPI cannot drive the MISO output via TX in slave mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOD_A::ENABLE)
    }
    #[doc = "SPI can drive the MISO output via TX in slave mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOD_A::DISABLE)
    }
}
#[doc = "Field `MSB` reader - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
pub type MSB_R = crate::BitReader<MSB_A>;
#[doc = "4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSB_A {
    #[doc = "1: MSB first"]
    ENABLE = 1,
    #[doc = "0: LSB first"]
    DISABLE = 0,
}
impl From<MSB_A> for bool {
    #[inline(always)]
    fn from(variant: MSB_A) -> Self {
        variant as u8 != 0
    }
}
impl MSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSB_A {
        match self.bits {
            true => MSB_A::ENABLE,
            false => MSB_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MSB_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MSB_A::DISABLE
    }
}
#[doc = "Field `MSB` writer - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
pub type MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, MSB_A, O>;
impl<'a, const O: u8> MSB_W<'a, O> {
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MSB_A::ENABLE)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MSB_A::DISABLE)
    }
}
#[doc = "Field `PEN` reader - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
pub type PEN_R = crate::BitReader<PEN_A>;
#[doc = "5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEN_A {
    #[doc = "1: Enable Parity function"]
    ENABLE = 1,
    #[doc = "0: Disable Parity function"]
    DISABLE = 0,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            true => PEN_A::ENABLE,
            false => PEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PEN_A::DISABLE
    }
}
#[doc = "Field `PEN` writer - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, PEN_A, O>;
impl<'a, const O: u8> PEN_W<'a, O> {
    #[doc = "Enable Parity function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEN_A::ENABLE)
    }
    #[doc = "Disable Parity function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEN_A::DISABLE)
    }
}
#[doc = "Field `PES` reader - 6:6\\]
Even Parity Select"]
pub type PES_R = crate::BitReader<PES_A>;
#[doc = "6:6\\]
Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES_A {
    #[doc = "1: Even Parity mode"]
    ENABLE = 1,
    #[doc = "0: Odd Parity mode"]
    DISABLE = 0,
}
impl From<PES_A> for bool {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as u8 != 0
    }
}
impl PES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PES_A {
        match self.bits {
            true => PES_A::ENABLE,
            false => PES_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PES_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PES_A::DISABLE
    }
}
#[doc = "Field `PES` writer - 6:6\\]
Even Parity Select"]
pub type PES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, PES_A, O>;
impl<'a, const O: u8> PES_W<'a, O> {
    #[doc = "Even Parity mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PES_A::ENABLE)
    }
    #[doc = "Odd Parity mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PES_A::DISABLE)
    }
}
#[doc = "Field `PBS` reader - 7:7\\]
Parity Bit Select"]
pub type PBS_R = crate::BitReader<PBS_A>;
#[doc = "7:7\\]
Parity Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBS_A {
    #[doc = "1: Bit 1 is used for Parity, Bit 0 is ignored"]
    ENABLE = 1,
    #[doc = "0: Bit 0 is used for Parity"]
    DISABLE = 0,
}
impl From<PBS_A> for bool {
    #[inline(always)]
    fn from(variant: PBS_A) -> Self {
        variant as u8 != 0
    }
}
impl PBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBS_A {
        match self.bits {
            true => PBS_A::ENABLE,
            false => PBS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PBS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PBS_A::DISABLE
    }
}
#[doc = "Field `PBS` writer - 7:7\\]
Parity Bit Select"]
pub type PBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, PBS_A, O>;
impl<'a, const O: u8> PBS_W<'a, O> {
    #[doc = "Bit 1 is used for Parity, Bit 0 is ignored"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PBS_A::ENABLE)
    }
    #[doc = "Bit 0 is used for Parity"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PBS_A::DISABLE)
    }
}
#[doc = "Field `RESERVED8` reader - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED8` writer - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIFORST` reader - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
pub type FIFORST_R = crate::BitReader<FIFORST_A>;
#[doc = "10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFORST_A {
    #[doc = "1: Set FIFO pointers reset trigger"]
    SET = 1,
    #[doc = "0: Clear FIFO pointers reset trigger"]
    CLR = 0,
}
impl From<FIFORST_A> for bool {
    #[inline(always)]
    fn from(variant: FIFORST_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFORST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFORST_A {
        match self.bits {
            true => FIFORST_A::SET,
            false => FIFORST_A::CLR,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == FIFORST_A::SET
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == FIFORST_A::CLR
    }
}
#[doc = "Field `FIFORST` writer - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
pub type FIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, FIFORST_A, O>;
impl<'a, const O: u8> FIFORST_W<'a, O> {
    #[doc = "Set FIFO pointers reset trigger"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(FIFORST_A::SET)
    }
    #[doc = "Clear FIFO pointers reset trigger"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(FIFORST_A::CLR)
    }
}
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `REPEATTX` reader - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
pub type REPEATTX_R = crate::FieldReader<u8, REPEATTX_A>;
#[doc = "23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPEATTX_A {
    #[doc = "0: REPEATTX disable"]
    DISABLE = 0,
}
impl From<REPEATTX_A> for u8 {
    #[inline(always)]
    fn from(variant: REPEATTX_A) -> Self {
        variant as _
    }
}
impl REPEATTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REPEATTX_A> {
        match self.bits {
            0 => Some(REPEATTX_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REPEATTX_A::DISABLE
    }
}
#[doc = "Field `REPEATTX` writer - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
pub type REPEATTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, REPEATTX_A, 8, O>;
impl<'a, const O: u8> REPEATTX_W<'a, O> {
    #[doc = "REPEATTX disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REPEATTX_A::DISABLE)
    }
}
#[doc = "Field `RXTIMEOUT` reader - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
pub type RXTIMEOUT_R = crate::FieldReader<u8, RXTIMEOUT_A>;
#[doc = "29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXTIMEOUT_A {
    #[doc = "63: Highest possible value"]
    MAXIMUM = 63,
    #[doc = "0: Smallest value"]
    MINIMUM = 0,
}
impl From<RXTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXTIMEOUT_A) -> Self {
        variant as _
    }
}
impl RXTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXTIMEOUT_A> {
        match self.bits {
            63 => Some(RXTIMEOUT_A::MAXIMUM),
            0 => Some(RXTIMEOUT_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == RXTIMEOUT_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == RXTIMEOUT_A::MINIMUM
    }
}
#[doc = "Field `RXTIMEOUT` writer - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
pub type RXTIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL1_SPEC, u8, RXTIMEOUT_A, 6, O>;
impl<'a, const O: u8> RXTIMEOUT_W<'a, O> {
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::MAXIMUM)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(RXTIMEOUT_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SPI enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop back mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Even Parity Select"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Parity Bit Select"]
    #[inline(always)]
    pub fn pbs(&self) -> PBS_R {
        PBS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
    #[inline(always)]
    pub fn repeattx(&self) -> REPEATTX_R {
        REPEATTX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
    #[inline(always)]
    pub fn rxtimeout(&self) -> RXTIMEOUT_R {
        RXTIMEOUT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<1> {
        LBM_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE = 0."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<2> {
        MS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode: Data output disabled This bit is relevant only in the slave mode, MS=0. In multiple-slave systems, it is possible for an SPI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RX lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SPI slave is not supposed to drive the TX line."]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SOD_W<3> {
        SOD_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MSB_W<4> {
        MSB_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Parity enable if enabled the last bit will be used as parity to evaluate the right transmission of the previous bits. In case of a parity mismatch the parity error flag RIS.PER will be set."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<5> {
        PEN_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<6> {
        PES_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Parity Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn pbs(&mut self) -> PBS_W<7> {
        PBS_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is used to reset transmit and receive FIFO pointers. The pointers are held at a reset value until this bit is cleared to zero."]
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FIFORST_W<10> {
        FIFORST_W::new(self)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Counter to repeat last transfer. A value of 0 disables this feature. After a non-zero value (X) is written to this register, SPI transfer can be started with writing a data into the TX Buffer. The data will be transferred X+1 times in total. The behavior is identical as if the data were be written into the TX Buffer that many times as defined by the value here additionally. It can be used to clean a transfer or to pull a certain amount of data by a slave. This feature can be used only in the master mode."]
    #[inline(always)]
    #[must_use]
    pub fn repeattx(&mut self) -> REPEATTX_W<16> {
        REPEATTX_W::new(self)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Receive Timeout (only for Slave mode). This register defines the number of clock cycles after which the Receive Timeout interrupt is set. A value of 0 disables this function."]
    #[inline(always)]
    #[must_use]
    pub fn rxtimeout(&mut self) -> RXTIMEOUT_W<24> {
        RXTIMEOUT_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> RESERVED30_W<30> {
        RESERVED30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x04"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
