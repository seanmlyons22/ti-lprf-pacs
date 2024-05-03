#[doc = "Register `FREQ_OFFSET` reader"]
pub type R = crate::R<FreqOffsetSpec>;
#[doc = "Register `FREQ_OFFSET` writer"]
pub type W = crate::W<FreqOffsetSpec>;
#[doc = "Field `HF_COMP_P2` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HfCompP2R = crate::FieldReader;
#[doc = "Field `HF_COMP_P2` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HfCompP2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HF_COMP_P1` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HfCompP1R = crate::FieldReader;
#[doc = "Field `HF_COMP_P1` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HfCompP1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HF_COMP_P0` reader - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HfCompP0R = crate::FieldReader<u16>;
#[doc = "Field `HF_COMP_P0` writer - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HfCompP0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p2(&self) -> HfCompP2R {
        HfCompP2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p1(&self) -> HfCompP1R {
        HfCompP1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p0(&self) -> HfCompP0R {
        HfCompP0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp_p2(&mut self) -> HfCompP2W<FreqOffsetSpec> {
        HfCompP2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp_p1(&mut self) -> HfCompP1W<FreqOffsetSpec> {
        HfCompP1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp_p0(&mut self) -> HfCompP0W<FreqOffsetSpec> {
        HfCompP0W::new(self, 16)
    }
}
#[doc = "Frequency Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freq_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freq_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets FREQ_OFFSET to value 0xffff_ffff"]
impl crate::Resettable for FreqOffsetSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
