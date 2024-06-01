#[doc = "Register `TBPMR` reader"]
pub type R = crate::R<TbpmrSpec>;
#[doc = "Register `TBPMR` writer"]
pub type W = crate::W<TbpmrSpec>;
#[doc = "Field `TBPSMR` reader - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
pub type TbpsmrR = crate::FieldReader;
#[doc = "Field `TBPSMR` writer - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
pub type TbpsmrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub fn tbpsmr(&self) -> TbpsmrR {
        TbpsmrR::new((self.bits & 0xff) as u8)
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
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    #[must_use]
    pub fn tbpsmr(&mut self) -> TbpsmrW<TbpmrSpec> {
        TbpsmrW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<TbpmrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbpmrSpec;
impl crate::RegisterSpec for TbpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbpmr::R`](R) reader structure"]
impl crate::Readable for TbpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbpmr::W`](W) writer structure"]
impl crate::Writable for TbpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TbpmrSpec {
    const RESET_VALUE: u32 = 0;
}
