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
#[doc = "Field `MCU_WARM_RESET` reader - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
pub type MCU_WARM_RESET_R = crate::BitReader<bool>;
#[doc = "Field `MCU_WARM_RESET` writer - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
pub type MCU_WARM_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `MCU_RESET_SRC` reader - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
pub type MCU_RESET_SRC_R = crate::BitReader<bool>;
#[doc = "Field `MCU_RESET_SRC` writer - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
pub type MCU_RESET_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    pub fn mcu_warm_reset(&self) -> MCU_WARM_RESET_R {
        MCU_WARM_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    pub fn mcu_reset_src(&self) -> MCU_RESET_SRC_R {
        MCU_RESET_SRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_warm_reset(&mut self) -> MCU_WARM_RESET_W<0> {
        MCU_WARM_RESET_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_reset_src(&mut self) -> MCU_RESET_SRC_W<1> {
        MCU_RESET_SRC_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 This register contains various chip level control and debug bitfields.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
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
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
