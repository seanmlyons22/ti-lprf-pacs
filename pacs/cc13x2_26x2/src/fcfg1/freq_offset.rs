#[doc = "Register `FREQ_OFFSET` reader"]
pub type R = crate::R<FreqOffsetSpec>;
#[doc = "Register `FREQ_OFFSET` writer"]
pub type W = crate::W<FreqOffsetSpec>;
#[doc = "Field `HPOSC_COMP_P2` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP2R = crate::FieldReader;
#[doc = "Field `HPOSC_COMP_P2` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPOSC_COMP_P1` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP1R = crate::FieldReader;
#[doc = "Field `HPOSC_COMP_P1` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPOSC_COMP_P0` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP0R = crate::FieldReader<u16>;
#[doc = "Field `HPOSC_COMP_P0` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p2(&self) -> HposcCompP2R {
        HposcCompP2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p1(&self) -> HposcCompP1R {
        HposcCompP1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p0(&self) -> HposcCompP0R {
        HposcCompP0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_comp_p2(&mut self) -> HposcCompP2W<FreqOffsetSpec> {
        HposcCompP2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_comp_p1(&mut self) -> HposcCompP1W<FreqOffsetSpec> {
        HposcCompP1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_comp_p0(&mut self) -> HposcCompP0W<FreqOffsetSpec> {
        HposcCompP0W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freq_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freq_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqOffsetSpec;
impl crate::RegisterSpec for FreqOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freq_offset::R`](R) reader structure"]
impl crate::Readable for FreqOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`freq_offset::W`](W) writer structure"]
impl crate::Writable for FreqOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQ_OFFSET to value 0"]
impl crate::Resettable for FreqOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
