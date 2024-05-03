#[doc = "Register `ACC` reader"]
pub type R = crate::R<AccSpec>;
#[doc = "Register `ACC` writer"]
pub type W = crate::W<AccSpec>;
#[doc = "Field `ACCUMULATOR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type AccumulatorR = crate::FieldReader<u32>;
#[doc = "Field `ACCUMULATOR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type AccumulatorW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn accumulator(&self) -> AccumulatorR {
        AccumulatorR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn accumulator(&mut self) -> AccumulatorW<AccSpec> {
        AccumulatorW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<AccSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccSpec;
impl crate::RegisterSpec for AccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acc::R`](R) reader structure"]
impl crate::Readable for AccSpec {}
#[doc = "`write(|w| ..)` method takes [`acc::W`](W) writer structure"]
impl crate::Writable for AccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACC to value 0"]
impl crate::Resettable for AccSpec {
    const RESET_VALUE: u32 = 0;
}
