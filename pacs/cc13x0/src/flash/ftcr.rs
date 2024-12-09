#[doc = "Register `FTCR` reader"]
pub type R = crate::R<FtcrSpec>;
#[doc = "Register `FTCR` writer"]
pub type W = crate::W<FtcrSpec>;
#[doc = "Field `TCR` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type TcrR = crate::FieldReader;
#[doc = "Field `TCR` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type TcrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Internal. Only to be used through TI provided API."]
pub type Reserved7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TcrW<FtcrSpec> {
        TcrW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtcrSpec;
impl crate::RegisterSpec for FtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftcr::R`](R) reader structure"]
impl crate::Readable for FtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftcr::W`](W) writer structure"]
impl crate::Writable for FtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTCR to value 0"]
impl crate::Resettable for FtcrSpec {
    const RESET_VALUE: u32 = 0;
}
