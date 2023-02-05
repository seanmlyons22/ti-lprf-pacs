#[doc = "Register `MISOCFG` reader"]
pub struct R(crate::R<MISOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISOCFG` writer"]
pub struct W(crate::W<MISOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISOCFG_SPEC>;
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
impl From<crate::W<MISOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXIO` reader - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
pub type AUXIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUXIO` writer - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
pub type AUXIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISOCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISOCFG_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
    #[inline(always)]
    pub fn auxio(&self) -> AUXIO_R {
        AUXIO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
    #[inline(always)]
    #[must_use]
    pub fn auxio(&mut self) -> AUXIO_W<0> {
        AUXIO_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MISO Configuration Write operation stalls until current transfer completes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misocfg](index.html) module"]
pub struct MISOCFG_SPEC;
impl crate::RegisterSpec for MISOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misocfg::R](R) reader structure"]
impl crate::Readable for MISOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misocfg::W](W) writer structure"]
impl crate::Writable for MISOCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISOCFG to value 0"]
impl crate::Resettable for MISOCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
