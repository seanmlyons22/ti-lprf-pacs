#[doc = "Register `LOAD` reader"]
pub struct R(crate::R<LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD` writer"]
pub struct W(crate::W<LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_SPEC>;
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
impl From<crate::W<LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTLOAD` reader - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
pub type WDTLOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDTLOAD` writer - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
pub type WDTLOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOAD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
    #[inline(always)]
    pub fn wdtload(&self) -> WDTLOAD_R {
        WDTLOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
    #[inline(always)]
    #[must_use]
    pub fn wdtload(&mut self) -> WDTLOAD_W<0> {
        WDTLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](index.html) module"]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load::R](R) reader structure"]
impl crate::Readable for LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load::W](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOAD to value 0xffff_ffff"]
impl crate::Resettable for LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
