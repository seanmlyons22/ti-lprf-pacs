#[doc = "Register `WDTLOAD` reader"]
pub struct R(crate::R<WDTLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTLOAD` writer"]
pub struct W(crate::W<WDTLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTLOAD_SPEC>;
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
impl From<crate::W<WDTLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
pub type LOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOAD` writer - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTLOAD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Load Value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtload](index.html) module"]
pub struct WDTLOAD_SPEC;
impl crate::RegisterSpec for WDTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtload::R](R) reader structure"]
impl crate::Readable for WDTLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtload::W](W) writer structure"]
impl crate::Writable for WDTLOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTLOAD to value 0"]
impl crate::Resettable for WDTLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
