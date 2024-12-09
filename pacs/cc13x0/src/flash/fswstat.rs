#[doc = "Register `FSWSTAT` reader"]
pub type R = crate::R<FswstatSpec>;
#[doc = "Register `FSWSTAT` writer"]
pub type W = crate::W<FswstatSpec>;
#[doc = "Field `SAFELV` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SafelvR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn safelv(&self) -> SafelvR {
        SafelvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fswstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fswstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FswstatSpec;
impl crate::RegisterSpec for FswstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fswstat::R`](R) reader structure"]
impl crate::Readable for FswstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fswstat::W`](W) writer structure"]
impl crate::Writable for FswstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSWSTAT to value 0x01"]
impl crate::Resettable for FswstatSpec {
    const RESET_VALUE: u32 = 0x01;
}
