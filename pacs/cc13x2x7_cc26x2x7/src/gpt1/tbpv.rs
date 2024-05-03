#[doc = "Register `TBPV` reader"]
pub type R = crate::R<TbpvSpec>;
#[doc = "Register `TBPV` writer"]
pub type W = crate::W<TbpvSpec>;
#[doc = "Field `PSV` reader - 7:0\\]
GPT Timer B Pre-scaler Value"]
pub type PsvR = crate::FieldReader;
#[doc = "Field `PSV` writer - 7:0\\]
GPT Timer B Pre-scaler Value"]
pub type PsvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scaler Value"]
    #[inline(always)]
    pub fn psv(&self) -> PsvR {
        PsvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn psv(&mut self) -> PsvW<TbpvSpec> {
        PsvW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<TbpvSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbpvSpec;
impl crate::RegisterSpec for TbpvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpv::R`](R) reader structure"]
impl crate::Readable for TbpvSpec {}
#[doc = "`write(|w| ..)` method takes [`tbpv::W`](W) writer structure"]
impl crate::Writable for TbpvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPV to value 0"]
impl crate::Resettable for TbpvSpec {
    const RESET_VALUE: u32 = 0;
}
