#[doc = "Register `TARGET` reader"]
pub type R = crate::R<TargetSpec>;
#[doc = "Register `TARGET` writer"]
pub type W = crate::W<TargetSpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
16 bit user defined counter target value, which is used when selected by CTL.TARGET_EN."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
16 bit user defined counter target value, which is used when selected by CTL.TARGET_EN."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
16 bit user defined counter target value, which is used when selected by CTL.TARGET_EN."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
16 bit user defined counter target value, which is used when selected by CTL.TARGET_EN."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<TargetSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TargetSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Target User defined counter target.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetSpec;
impl crate::RegisterSpec for TargetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target::R`](R) reader structure"]
impl crate::Readable for TargetSpec {}
#[doc = "`write(|w| ..)` method takes [`target::W`](W) writer structure"]
impl crate::Writable for TargetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET to value 0"]
impl crate::Resettable for TargetSpec {
    const RESET_VALUE: u32 = 0;
}
