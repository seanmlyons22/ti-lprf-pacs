#[doc = "Register `RCOSCHFCTL` reader"]
pub type R = crate::R<RcoschfctlSpec>;
#[doc = "Register `RCOSCHFCTL` writer"]
pub type W = crate::W<RcoschfctlSpec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RCOSCHF_CTRIM` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfCtrimR = crate::FieldReader;
#[doc = "Field `RCOSCHF_CTRIM` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfCtrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RcoschfCtrimR {
        RcoschfCtrimR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RcoschfctlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschf_ctrim(&mut self) -> RcoschfCtrimW<RcoschfctlSpec> {
        RcoschfCtrimW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<RcoschfctlSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "RCOSCHF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcoschfctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcoschfctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcoschfctlSpec;
impl crate::RegisterSpec for RcoschfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcoschfctl::R`](R) reader structure"]
impl crate::Readable for RcoschfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rcoschfctl::W`](W) writer structure"]
impl crate::Writable for RcoschfctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCOSCHFCTL to value 0"]
impl crate::Resettable for RcoschfctlSpec {
    const RESET_VALUE: u32 = 0;
}
