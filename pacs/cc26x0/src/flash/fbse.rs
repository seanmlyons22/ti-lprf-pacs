#[doc = "Register `FBSE` reader"]
pub type R = crate::R<FbseSpec>;
#[doc = "Register `FBSE` writer"]
pub type W = crate::W<FbseSpec>;
#[doc = "Field `BSE` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type BseR = crate::FieldReader<u16>;
#[doc = "Field `BSE` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type BseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bse(&self) -> BseR {
        BseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bse(&mut self) -> BseW<FbseSpec> {
        BseW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FbseSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbseSpec;
impl crate::RegisterSpec for FbseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbse::R`](R) reader structure"]
impl crate::Readable for FbseSpec {}
#[doc = "`write(|w| ..)` method takes [`fbse::W`](W) writer structure"]
impl crate::Writable for FbseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBSE to value 0"]
impl crate::Resettable for FbseSpec {
    const RESET_VALUE: u32 = 0;
}
