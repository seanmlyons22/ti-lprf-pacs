#[doc = "Register `FLOCK` reader"]
pub type R = crate::R<FlockSpec>;
#[doc = "Register `FLOCK` writer"]
pub type W = crate::W<FlockSpec>;
#[doc = "Field `ENCOM` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type EncomR = crate::FieldReader<u16>;
#[doc = "Field `ENCOM` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type EncomW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn encom(&self) -> EncomR {
        EncomR::new((self.bits & 0xffff) as u16)
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
    pub fn encom(&mut self) -> EncomW<FlockSpec> {
        EncomW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FlockSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlockSpec;
impl crate::RegisterSpec for FlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flock::R`](R) reader structure"]
impl crate::Readable for FlockSpec {}
#[doc = "`write(|w| ..)` method takes [`flock::W`](W) writer structure"]
impl crate::Writable for FlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOCK to value 0x55aa"]
impl crate::Resettable for FlockSpec {
    const RESET_VALUE: u32 = 0x55aa;
}
