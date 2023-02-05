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
#[doc = "Field `INTLINESNUM` reader - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
pub type INTLINESNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTLINESNUM` writer - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
pub type INTLINESNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICTR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
    #[inline(always)]
    #[must_use]
    pub fn intlinesnum(&mut self) -> INTLINESNUM_W<0> {
        INTLINESNUM_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictr](index.html) module"]
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
#[doc = "`reset()` method sets ICTR to value 0x01"]
impl crate::Resettable for ICTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
