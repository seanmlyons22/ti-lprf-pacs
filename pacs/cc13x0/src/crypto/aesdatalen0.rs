#[doc = "Register `AESDATALEN0` reader"]
pub struct R(crate::R<AESDATALEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESDATALEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESDATALEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESDATALEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESDATALEN0` writer"]
pub struct W(crate::W<AESDATALEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESDATALEN0_SPEC>;
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
impl From<crate::W<AESDATALEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESDATALEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN_LSW` reader - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
pub type LEN_LSW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LEN_LSW` writer - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
pub type LEN_LSW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AESDATALEN0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
    #[inline(always)]
    pub fn len_lsw(&self) -> LEN_LSW_R {
        LEN_LSW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\]
of the combined data length."]
    #[inline(always)]
    #[must_use]
    pub fn len_lsw(&mut self) -> LEN_LSW_W<0> {
        LEN_LSW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crypto Data Length LSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatalen0](index.html) module"]
pub struct AESDATALEN0_SPEC;
impl crate::RegisterSpec for AESDATALEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesdatalen0::R](R) reader structure"]
impl crate::Readable for AESDATALEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesdatalen0::W](W) writer structure"]
impl crate::Writable for AESDATALEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESDATALEN0 to value 0"]
impl crate::Resettable for AESDATALEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
