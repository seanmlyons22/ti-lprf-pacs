#[doc = "Register `FVWLCT` reader"]
pub type R = crate::R<FvwlctSpec>;
#[doc = "Register `FVWLCT` writer"]
pub type W = crate::W<FvwlctSpec>;
#[doc = "Field `VWLCT_P` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VwlctPR = crate::FieldReader;
#[doc = "Field `VWLCT_P` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VwlctPW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct_p(&self) -> VwlctPR {
        VwlctPR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vwlct_p(&mut self) -> VwlctPW<FvwlctSpec> {
        VwlctPW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvwlct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvwlct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FvwlctSpec;
impl crate::RegisterSpec for FvwlctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvwlct::R`](R) reader structure"]
impl crate::Readable for FvwlctSpec {}
#[doc = "`write(|w| ..)` method takes [`fvwlct::W`](W) writer structure"]
impl crate::Writable for FvwlctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVWLCT to value 0x08"]
impl crate::Resettable for FvwlctSpec {
    const RESET_VALUE: u32 = 0x08;
}
