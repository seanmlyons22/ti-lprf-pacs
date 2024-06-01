#[doc = "Register `SHPR2` reader"]
pub type R = crate::R<Shpr2Spec>;
#[doc = "Register `SHPR2` writer"]
pub type W = crate::W<Shpr2Spec>;
#[doc = "Field `RESERVED0` reader - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PRI_11` reader - 31:24\\]
Priority of system handler 11. SVCall"]
pub type Pri11R = crate::FieldReader;
#[doc = "Field `PRI_11` writer - 31:24\\]
Priority of system handler 11. SVCall"]
pub type Pri11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 11. SVCall"]
    #[inline(always)]
    pub fn pri_11(&self) -> Pri11R {
        Pri11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Shpr2Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 11. SVCall"]
    #[inline(always)]
    #[must_use]
    pub fn pri_11(&mut self) -> Pri11W<Shpr2Spec> {
        Pri11W::new(self, 24)
    }
}
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shpr2Spec;
impl crate::RegisterSpec for Shpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr2::R`](R) reader structure"]
impl crate::Readable for Shpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`shpr2::W`](W) writer structure"]
impl crate::Writable for Shpr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR2 to value 0"]
impl crate::Resettable for Shpr2Spec {
    const RESET_VALUE: u32 = 0;
}
