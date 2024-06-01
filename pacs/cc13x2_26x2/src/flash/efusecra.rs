#[doc = "Register `EFUSECRA` reader"]
pub type R = crate::R<EfusecraSpec>;
#[doc = "Register `EFUSECRA` writer"]
pub type W = crate::W<EfusecraSpec>;
#[doc = "Field `DATA` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<EfusecraSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<EfusecraSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusecra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusecra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusecraSpec;
impl crate::RegisterSpec for EfusecraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efusecra::R`](R) reader structure"]
impl crate::Readable for EfusecraSpec {}
#[doc = "`write(|w| ..)` method takes [`efusecra::W`](W) writer structure"]
impl crate::Writable for EfusecraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSECRA to value 0"]
impl crate::Resettable for EfusecraSpec {
    const RESET_VALUE: u32 = 0;
}
