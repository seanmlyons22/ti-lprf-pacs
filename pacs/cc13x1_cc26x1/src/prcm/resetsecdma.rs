#[doc = "Register `RESETSECDMA` reader"]
pub struct R(crate::R<RESETSECDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETSECDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETSECDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETSECDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETSECDMA` writer"]
pub struct W(crate::W<RESETSECDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETSECDMA_SPEC>;
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
impl From<crate::W<RESETSECDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETSECDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO` reader - 0:0\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type CRYPTO_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO` writer - 0:0\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type CRYPTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSECDMA_SPEC, bool, O>;
#[doc = "Field `TRNG` reader - 1:1\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type TRNG_R = crate::BitReader<bool>;
#[doc = "Field `TRNG` writer - 1:1\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type TRNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSECDMA_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETSECDMA_SPEC, u8, u8, 6, O>;
#[doc = "Field `DMA` reader - 8:8\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - 8:8\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSECDMA_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESETSECDMA_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn crypto(&self) -> CRYPTO_R {
        CRYPTO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn crypto(&mut self) -> CRYPTO_W<0> {
        CRYPTO_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn trng(&mut self) -> TRNG_W<1> {
        TRNG_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESET For SEC (TRNG And CRYPTO) And UDMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetsecdma](index.html) module"]
pub struct RESETSECDMA_SPEC;
impl crate::RegisterSpec for RESETSECDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetsecdma::R](R) reader structure"]
impl crate::Readable for RESETSECDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetsecdma::W](W) writer structure"]
impl crate::Writable for RESETSECDMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETSECDMA to value 0"]
impl crate::Resettable for RESETSECDMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
