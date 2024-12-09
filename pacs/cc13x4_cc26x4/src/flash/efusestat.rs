#[doc = "Register `EFUSESTAT` reader"]
pub type R = crate::R<EfusestatSpec>;
#[doc = "Register `EFUSESTAT` writer"]
pub type W = crate::W<EfusestatSpec>;
#[doc = "Field `RESETDONE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ResetdoneR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn resetdone(&self) -> ResetdoneR {
        ResetdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusestat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusestat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusestatSpec;
impl crate::RegisterSpec for EfusestatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efusestat::R`](R) reader structure"]
impl crate::Readable for EfusestatSpec {}
#[doc = "`write(|w| ..)` method takes [`efusestat::W`](W) writer structure"]
impl crate::Writable for EfusestatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSESTAT to value 0x01"]
impl crate::Resettable for EfusestatSpec {
    const RESET_VALUE: u32 = 0x01;
}
