#[doc = "Register `FBPROT` reader"]
pub struct R(crate::R<FBPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBPROT` writer"]
pub struct W(crate::W<FBPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBPROT_SPEC>;
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
impl From<crate::W<FBPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTL1DIS` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type PROTL1DIS_R = crate::BitReader<bool>;
#[doc = "Field `PROTL1DIS` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type PROTL1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBPROT_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBPROT_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn protl1dis(&self) -> PROTL1DIS_R {
        PROTL1DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn protl1dis(&mut self) -> PROTL1DIS_W<0> {
        PROTL1DIS_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprot](index.html) module"]
pub struct FBPROT_SPEC;
impl crate::RegisterSpec for FBPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbprot::R](R) reader structure"]
impl crate::Readable for FBPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbprot::W](W) writer structure"]
impl crate::Writable for FBPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBPROT to value 0"]
impl crate::Resettable for FBPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
