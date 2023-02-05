#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBM` reader - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
pub type LBM_R = crate::BitReader<bool>;
#[doc = "Field `LBM` writer - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
pub type LBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SSE` reader - 1:1\\]
Synchronous serial interface enable."]
pub type SSE_R = crate::BitReader<SSE_A>;
#[doc = "1:1\\]
Synchronous serial interface enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSE_A {
    #[doc = "1: Operation enabled"]
    SSI_ENABLED = 1,
    #[doc = "0: Operation disabled"]
    SSI_DISABLED = 0,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSE_A {
        match self.bits {
            true => SSE_A::SSI_ENABLED,
            false => SSE_A::SSI_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_ENABLED`"]
    #[inline(always)]
    pub fn is_ssi_enabled(&self) -> bool {
        *self == SSE_A::SSI_ENABLED
    }
    #[doc = "Checks if the value of the field is `SSI_DISABLED`"]
    #[inline(always)]
    pub fn is_ssi_disabled(&self) -> bool {
        *self == SSE_A::SSI_DISABLED
    }
}
#[doc = "Field `SSE` writer - 1:1\\]
Synchronous serial interface enable."]
pub type SSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SSE_A, O>;
impl<'a, const O: u8> SSE_W<'a, O> {
    #[doc = "Operation enabled"]
    #[inline(always)]
    pub fn ssi_enabled(self) -> &'a mut W {
        self.variant(SSE_A::SSI_ENABLED)
    }
    #[doc = "Operation disabled"]
    #[inline(always)]
    pub fn ssi_disabled(self) -> &'a mut W {
        self.variant(SSE_A::SSI_DISABLED)
    }
}
#[doc = "Field `MS` reader - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
pub type MS_R = crate::BitReader<MS_A>;
#[doc = "2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MS_A {
    #[doc = "1: Device configured as slave"]
    SLAVE = 1,
    #[doc = "0: Device configured as master"]
    MASTER = 0,
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
            true => MS_A::SLAVE,
            false => MS_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
}
#[doc = "Field `MS` writer - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
pub type MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MS_A, O>;
impl<'a, const O: u8> MS_W<'a, O> {
    #[doc = "Device configured as slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MS_A::SLAVE)
    }
    #[doc = "Device configured as master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
}
#[doc = "Field `SOD` reader - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
pub type SOD_R = crate::BitReader<bool>;
#[doc = "Field `SOD` writer - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
pub type SOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Synchronous serial interface enable."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<0> {
        LBM_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Synchronous serial interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<1> {
        SSE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<2> {
        MS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SOD_W<3> {
        SOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
