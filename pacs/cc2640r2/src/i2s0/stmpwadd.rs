#[doc = "Register `STMPWADD` reader"]
pub type R = crate::R<StmpwaddSpec>;
#[doc = "Register `STMPWADD` writer"]
pub type W = crate::W<StmpwaddSpec>;
#[doc = "Field `VALUE_INC` reader - 15:0\\]
WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
pub type ValueIncR = crate::FieldReader<u16>;
#[doc = "Field `VALUE_INC` writer - 15:0\\]
WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
pub type ValueIncW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
    #[inline(always)]
    pub fn value_inc(&self) -> ValueIncR {
        ValueIncR::new((self.bits & 0xffff) as u16)
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
WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
    #[inline(always)]
    #[must_use]
    pub fn value_inc(&mut self) -> ValueIncW<StmpwaddSpec> {
        ValueIncW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<StmpwaddSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "WCLK Counter Add Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwadd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwadd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpwaddSpec;
impl crate::RegisterSpec for StmpwaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpwadd::R`](R) reader structure"]
impl crate::Readable for StmpwaddSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpwadd::W`](W) writer structure"]
impl crate::Writable for StmpwaddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPWADD to value 0"]
impl crate::Resettable for StmpwaddSpec {
    const RESET_VALUE: u32 = 0;
}
