#[doc = "Register `SHPR3` reader"]
pub type R = crate::R<Shpr3Spec>;
#[doc = "Register `SHPR3` writer"]
pub type W = crate::W<Shpr3Spec>;
#[doc = "Field `PRI_12` reader - 7:0\\]
Priority of system handler 12. Debug Monitor"]
pub type Pri12R = crate::FieldReader;
#[doc = "Field `PRI_12` writer - 7:0\\]
Priority of system handler 12. Debug Monitor"]
pub type Pri12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_14` reader - 23:16\\]
Priority of system handler 14. Pend SV"]
pub type Pri14R = crate::FieldReader;
#[doc = "Field `PRI_14` writer - 23:16\\]
Priority of system handler 14. Pend SV"]
pub type Pri14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_15` reader - 31:24\\]
Priority of system handler 15. SysTick exception"]
pub type Pri15R = crate::FieldReader;
#[doc = "Field `PRI_15` writer - 31:24\\]
Priority of system handler 15. SysTick exception"]
pub type Pri15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 12. Debug Monitor"]
    #[inline(always)]
    pub fn pri_12(&self) -> Pri12R {
        Pri12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 14. Pend SV"]
    #[inline(always)]
    pub fn pri_14(&self) -> Pri14R {
        Pri14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 15. SysTick exception"]
    #[inline(always)]
    pub fn pri_15(&self) -> Pri15R {
        Pri15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 12. Debug Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn pri_12(&mut self) -> Pri12W<Shpr3Spec> {
        Pri12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Shpr3Spec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 14. Pend SV"]
    #[inline(always)]
    #[must_use]
    pub fn pri_14(&mut self) -> Pri14W<Shpr3Spec> {
        Pri14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 15. SysTick exception"]
    #[inline(always)]
    #[must_use]
    pub fn pri_15(&mut self) -> Pri15W<Shpr3Spec> {
        Pri15W::new(self, 24)
    }
}
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shpr3Spec;
impl crate::RegisterSpec for Shpr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr3::R`](R) reader structure"]
impl crate::Readable for Shpr3Spec {}
#[doc = "`write(|w| ..)` method takes [`shpr3::W`](W) writer structure"]
impl crate::Writable for Shpr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for Shpr3Spec {
    const RESET_VALUE: u32 = 0;
}
