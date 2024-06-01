#[doc = "Register `FVSLP` reader"]
pub type R = crate::R<FvslpSpec>;
#[doc = "Register `FVSLP` writer"]
pub type W = crate::W<FvslpSpec>;
#[doc = "Field `RESERVED0` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED0` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VSL_P` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VslPR = crate::FieldReader;
#[doc = "Field `VSL_P` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VslPW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&self) -> VslPR {
        VslPR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<FvslpSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vsl_p(&mut self) -> VslPW<FvslpSpec> {
        VslPW::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FvslpSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvslp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvslp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FvslpSpec;
impl crate::RegisterSpec for FvslpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvslp::R`](R) reader structure"]
impl crate::Readable for FvslpSpec {}
#[doc = "`write(|w| ..)` method takes [`fvslp::W`](W) writer structure"]
impl crate::Writable for FvslpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVSLP to value 0x8000"]
impl crate::Resettable for FvslpSpec {
    const RESET_VALUE: u32 = 0x8000;
}
