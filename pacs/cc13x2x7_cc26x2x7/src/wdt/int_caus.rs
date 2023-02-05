#[doc = "Register `INT_CAUS` reader"]
pub struct R(crate::R<INT_CAUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CAUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CAUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CAUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_CAUS` writer"]
pub struct W(crate::W<INT_CAUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CAUS_SPEC>;
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
impl From<crate::W<INT_CAUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CAUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAUSE_INTR` reader - 0:0\\]
Replica of RIS.WDTRIS"]
pub type CAUSE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CAUSE_INTR` writer - 0:0\\]
Replica of RIS.WDTRIS"]
pub type CAUSE_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CAUS_SPEC, bool, O>;
#[doc = "Field `CAUSE_RESET` reader - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
pub type CAUSE_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CAUSE_RESET` writer - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
pub type CAUSE_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CAUS_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT_CAUS_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Replica of RIS.WDTRIS"]
    #[inline(always)]
    pub fn cause_intr(&self) -> CAUSE_INTR_R {
        CAUSE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub fn cause_reset(&self) -> CAUSE_RESET_R {
        CAUSE_RESET_R::new(((self.bits >> 1) & 1) != 0)
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
Replica of RIS.WDTRIS"]
    #[inline(always)]
    #[must_use]
    pub fn cause_intr(&mut self) -> CAUSE_INTR_W<0> {
        CAUSE_INTR_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    #[must_use]
    pub fn cause_reset(&mut self) -> CAUSE_RESET_W<1> {
        CAUSE_RESET_W::new(self)
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
#[doc = "Interrupt Cause Test Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_caus](index.html) module"]
pub struct INT_CAUS_SPEC;
impl crate::RegisterSpec for INT_CAUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_caus::R](R) reader structure"]
impl crate::Readable for INT_CAUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_caus::W](W) writer structure"]
impl crate::Writable for INT_CAUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CAUS to value 0"]
impl crate::Resettable for INT_CAUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
