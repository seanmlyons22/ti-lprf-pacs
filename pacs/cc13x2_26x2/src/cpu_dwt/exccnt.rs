#[doc = "Register `EXCCNT` reader"]
pub type R = crate::R<ExccntSpec>;
#[doc = "Register `EXCCNT` writer"]
pub type W = crate::W<ExccntSpec>;
#[doc = "Field `EXCCNT` reader - 7:0\\]
Current interrupt overhead counter value. Counts the total cycles spent in interrupt processing (for example entry stacking, return unstacking, pre-emption). An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.EXCEVTENA."]
pub type ExccntR = crate::FieldReader;
#[doc = "Field `EXCCNT` writer - 7:0\\]
Current interrupt overhead counter value. Counts the total cycles spent in interrupt processing (for example entry stacking, return unstacking, pre-emption). An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.EXCEVTENA."]
pub type ExccntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Current interrupt overhead counter value. Counts the total cycles spent in interrupt processing (for example entry stacking, return unstacking, pre-emption). An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.EXCEVTENA."]
    #[inline(always)]
    pub fn exccnt(&self) -> ExccntR {
        ExccntR::new((self.bits & 0xff) as u8)
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
Current interrupt overhead counter value. Counts the total cycles spent in interrupt processing (for example entry stacking, return unstacking, pre-emption). An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.EXCEVTENA."]
    #[inline(always)]
    #[must_use]
    pub fn exccnt(&mut self) -> ExccntW<ExccntSpec> {
        ExccntW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<ExccntSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exccnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exccnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExccntSpec;
impl crate::RegisterSpec for ExccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exccnt::R`](R) reader structure"]
impl crate::Readable for ExccntSpec {}
#[doc = "`write(|w| ..)` method takes [`exccnt::W`](W) writer structure"]
impl crate::Writable for ExccntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXCCNT to value 0"]
impl crate::Resettable for ExccntSpec {
    const RESET_VALUE: u32 = 0;
}
