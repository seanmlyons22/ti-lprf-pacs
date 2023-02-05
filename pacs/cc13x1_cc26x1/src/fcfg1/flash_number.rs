#[doc = "Register `FLASH_NUMBER` reader"]
pub struct R(crate::R<FLASH_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_NUMBER` writer"]
pub struct W(crate::W<FLASH_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_NUMBER_SPEC>;
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
impl From<crate::W<FLASH_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOT_NUMBER` reader - 31:0\\]
Number of the manufacturing lot that produced this unit."]
pub type LOT_NUMBER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOT_NUMBER` writer - 31:0\\]
Number of the manufacturing lot that produced this unit."]
pub type LOT_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_NUMBER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub fn lot_number(&self) -> LOT_NUMBER_R {
        LOT_NUMBER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    #[must_use]
    pub fn lot_number(&mut self) -> LOT_NUMBER_W<0> {
        LOT_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_number](index.html) module"]
pub struct FLASH_NUMBER_SPEC;
impl crate::RegisterSpec for FLASH_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_number::R](R) reader structure"]
impl crate::Readable for FLASH_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_number::W](W) writer structure"]
impl crate::Writable for FLASH_NUMBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_NUMBER to value 0"]
impl crate::Resettable for FLASH_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
