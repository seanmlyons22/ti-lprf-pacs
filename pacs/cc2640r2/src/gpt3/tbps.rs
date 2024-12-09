#[doc = "Register `TBPS` reader"]
pub type R = crate::R<TbpsSpec>;
#[doc = "Register `TBPS` writer"]
pub type W = crate::W<TbpsSpec>;
#[doc = "Field `PSS` reader - 7:0\\]
GPT Timer B Pre-scaler"]
pub type PssR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scaler"]
    #[inline(always)]
    pub fn pss(&self) -> PssR {
        PssR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbpsSpec;
impl crate::RegisterSpec for TbpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbps::R`](R) reader structure"]
impl crate::Readable for TbpsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbps::W`](W) writer structure"]
impl crate::Writable for TbpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPS to value 0"]
impl crate::Resettable for TbpsSpec {
    const RESET_VALUE: u32 = 0;
}
