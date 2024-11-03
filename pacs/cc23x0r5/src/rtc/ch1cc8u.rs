#[doc = "Register `CH1CC8U` reader"]
pub type R = crate::R<Ch1cc8uSpec>;
#[doc = "Register `CH1CC8U` writer"]
pub type W = crate::W<Ch1cc8uSpec>;
#[doc = "Field `VAL` reader - 20:0\\]
TIME8U.VAL captured value at the last selected edge of capture event."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 20:0\\]
TIME8U.VAL captured value at the last selected edge of capture event."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `RESERVED21` reader - 31:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED21` writer - 31:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
TIME8U.VAL captured value at the last selected edge of capture event."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
TIME8U.VAL captured value at the last selected edge of capture event."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Ch1cc8uSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<Ch1cc8uSpec> {
        Reserved21W::new(self, 21)
    }
}
#[doc = "Channel 1 capture value. This register captures the RTC time slice \\[34:3\\]
on each selected edge of the capture event when the ARMSET.CH1 = 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cc8u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cc8u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cc8uSpec;
impl crate::RegisterSpec for Ch1cc8uSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cc8u::R`](R) reader structure"]
impl crate::Readable for Ch1cc8uSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1cc8u::W`](W) writer structure"]
impl crate::Writable for Ch1cc8uSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CC8U to value 0"]
impl crate::Resettable for Ch1cc8uSpec {
    const RESET_VALUE: u32 = 0;
}
