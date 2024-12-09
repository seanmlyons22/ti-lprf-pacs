#[doc = "Register `RESETSECDMA` reader"]
pub type R = crate::R<ResetsecdmaSpec>;
#[doc = "Register `RESETSECDMA` writer"]
pub type W = crate::W<ResetsecdmaSpec>;
#[doc = "Field `CRYPTO` writer - 0:0\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type CryptoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG` writer - 1:1\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type TrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA` writer - 2:2\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type PkaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `DMA` writer - 8:8\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn crypto(&mut self) -> CryptoW<ResetsecdmaSpec> {
        CryptoW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn trng(&mut self) -> TrngW<ResetsecdmaSpec> {
        TrngW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn pka(&mut self) -> PkaW<ResetsecdmaSpec> {
        PkaW::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to reset. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<ResetsecdmaSpec> {
        DmaW::new(self, 8)
    }
}
#[doc = "RESET For SEC (PKA And TRNG And CRYPTO) And UDMA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetsecdma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetsecdma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetsecdmaSpec;
impl crate::RegisterSpec for ResetsecdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetsecdma::R`](R) reader structure"]
impl crate::Readable for ResetsecdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`resetsecdma::W`](W) writer structure"]
impl crate::Writable for ResetsecdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETSECDMA to value 0"]
impl crate::Resettable for ResetsecdmaSpec {
    const RESET_VALUE: u32 = 0;
}
