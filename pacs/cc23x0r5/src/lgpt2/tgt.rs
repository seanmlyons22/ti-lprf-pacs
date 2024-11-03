#[doc = "Register `TGT` reader"]
pub type R = crate::R<TgtSpec>;
#[doc = "Register `TGT` writer"]
pub type W = crate::W<TgtSpec>;
#[doc = "Field `VAL` reader - 15:0\\]
User defined counter target value."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - 15:0\\]
User defined counter target value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
User defined counter target value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
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
User defined counter target value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<TgtSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TgtSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Target User defined counter target. A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tgt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tgt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgtSpec;
impl crate::RegisterSpec for TgtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgt::R`](R) reader structure"]
impl crate::Readable for TgtSpec {}
#[doc = "`write(|w| ..)` method takes [`tgt::W`](W) writer structure"]
impl crate::Writable for TgtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGT to value 0xffff"]
impl crate::Resettable for TgtSpec {
    const RESET_VALUE: u32 = 0xffff;
}
