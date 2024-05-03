#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LBM` reader - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "1:1\\]
Synchronous serial interface enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sse {
    #[doc = "1: Operation enabled"]
    SsiEnabled = 1,
    #[doc = "0: Operation disabled"]
    SsiDisabled = 0,
}
impl From<Sse> for bool {
    #[inline(always)]
    fn from(variant: Sse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSE` reader - 1:1\\]
Synchronous serial interface enable."]
pub type SseR = crate::BitReader<Sse>;
impl SseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sse {
        match self.bits {
            true => Sse::SsiEnabled,
            false => Sse::SsiDisabled,
        }
    }
    #[doc = "Operation enabled"]
    #[inline(always)]
    pub fn is_ssi_enabled(&self) -> bool {
        *self == Sse::SsiEnabled
    }
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn is_ssi_disabled(&self) -> bool {
        *self == Sse::SsiDisabled
    }
}
#[doc = "Field `SSE` writer - 1:1\\]
Synchronous serial interface enable."]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG, Sse>;
impl<'a, REG> SseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation enabled"]
    #[inline(always)]
    pub fn ssi_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::SsiEnabled)
    }
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn ssi_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::SsiDisabled)
    }
}
#[doc = "2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ms {
    #[doc = "1: Device configured as slave"]
    Slave = 1,
    #[doc = "0: Device configured as master"]
    Master = 0,
}
impl From<Ms> for bool {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
pub type MsR = crate::BitReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ms {
        match self.bits {
            true => Ms::Slave,
            false => Ms::Master,
        }
    }
    #[doc = "Device configured as slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ms::Slave
    }
    #[doc = "Device configured as master"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ms::Master
    }
}
#[doc = "Field `MS` writer - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device configured as slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Slave)
    }
    #[doc = "Device configured as master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Master)
    }
}
#[doc = "Field `SOD` reader - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Synchronous serial interface enable."]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<Cr1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Synchronous serial interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SseW<Cr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<Cr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SodW<Cr1Spec> {
        SodW::new(self, 3)
    }
}
#[doc = "Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
