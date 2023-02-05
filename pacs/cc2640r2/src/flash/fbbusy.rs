#[doc = "Register `FBBUSY` reader"]
pub struct R(crate::R<FBBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBBUSY` writer"]
pub struct W(crate::W<FBBUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBBUSY_SPEC>;
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
impl From<crate::W<FBBUSY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBBUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type BUSY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type BUSY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBBUSY_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBBUSY_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbbusy](index.html) module"]
pub struct FBBUSY_SPEC;
impl crate::RegisterSpec for FBBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbbusy::R](R) reader structure"]
impl crate::Readable for FBBUSY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbbusy::W](W) writer structure"]
impl crate::Writable for FBBUSY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBBUSY to value 0xfe"]
impl crate::Resettable for FBBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0xfe;
}
