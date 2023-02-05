#[doc = "Register `ICTR` reader"]
pub struct R(crate::R<ICTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICTR` writer"]
pub struct W(crate::W<ICTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICTR_SPEC>;
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
impl From<crate::W<ICTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTLINESNUM` reader - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
pub type INTLINESNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTLINESNUM` writer - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
pub type INTLINESNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICTR_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
    #[inline(always)]
    #[must_use]
    pub fn intlinesnum(&mut self) -> INTLINESNUM_W<0> {
        INTLINESNUM_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides information about the interrupt controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictr](index.html) module"]
pub struct ICTR_SPEC;
impl crate::RegisterSpec for ICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ictr::R](R) reader structure"]
impl crate::Readable for ICTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ictr::W](W) writer structure"]
impl crate::Writable for ICTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICTR to value 0"]
impl crate::Resettable for ICTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
