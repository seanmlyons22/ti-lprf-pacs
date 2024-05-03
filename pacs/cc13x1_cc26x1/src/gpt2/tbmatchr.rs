#[doc = "Register `TBMATCHR` reader"]
pub type R = crate::R<TbmatchrSpec>;
#[doc = "Register `TBMATCHR` writer"]
pub type W = crate::W<TbmatchrSpec>;
#[doc = "Field `TBMATCHR` reader - 15:0\\]
GPT Timer B Match Register"]
pub type TbmatchrR = crate::FieldReader<u16>;
#[doc = "Field `TBMATCHR` writer - 15:0\\]
GPT Timer B Match Register"]
pub type TbmatchrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
GPT Timer B Match Register"]
    #[inline(always)]
    pub fn tbmatchr(&self) -> TbmatchrR {
        TbmatchrR::new((self.bits & 0xffff) as u16)
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
GPT Timer B Match Register"]
    #[inline(always)]
    #[must_use]
    pub fn tbmatchr(&mut self) -> TbmatchrW<TbmatchrSpec> {
        TbmatchrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TbmatchrSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmatchr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmatchr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbmatchrSpec;
impl crate::RegisterSpec for TbmatchrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbmatchr::R`](R) reader structure"]
impl crate::Readable for TbmatchrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbmatchr::W`](W) writer structure"]
impl crate::Writable for TbmatchrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBMATCHR to value 0xffff"]
impl crate::Resettable for TbmatchrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
