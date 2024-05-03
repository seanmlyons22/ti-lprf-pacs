#[doc = "Register `SHPR1` reader"]
pub type R = crate::R<Shpr1Spec>;
#[doc = "Register `SHPR1` writer"]
pub type W = crate::W<Shpr1Spec>;
#[doc = "Field `PRI_4` reader - 7:0\\]
Priority of system handler 4: MemManage"]
pub type Pri4R = crate::FieldReader;
#[doc = "Field `PRI_4` writer - 7:0\\]
Priority of system handler 4: MemManage"]
pub type Pri4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_5` reader - 15:8\\]
Priority of system handler 5: BusFault"]
pub type Pri5R = crate::FieldReader;
#[doc = "Field `PRI_5` writer - 15:8\\]
Priority of system handler 5: BusFault"]
pub type Pri5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_6` reader - 23:16\\]
Priority of system handler 6. UsageFault"]
pub type Pri6R = crate::FieldReader;
#[doc = "Field `PRI_6` writer - 23:16\\]
Priority of system handler 6. UsageFault"]
pub type Pri6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_7` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Pri7R = crate::FieldReader;
#[doc = "Field `PRI_7` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Pri7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 4: MemManage"]
    #[inline(always)]
    pub fn pri_4(&self) -> Pri4R {
        Pri4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of system handler 5: BusFault"]
    #[inline(always)]
    pub fn pri_5(&self) -> Pri5R {
        Pri5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 6. UsageFault"]
    #[inline(always)]
    pub fn pri_6(&self) -> Pri6R {
        Pri6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn pri_7(&self) -> Pri7R {
        Pri7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 4: MemManage"]
    #[inline(always)]
    #[must_use]
    pub fn pri_4(&mut self) -> Pri4W<Shpr1Spec> {
        Pri4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of system handler 5: BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_5(&mut self) -> Pri5W<Shpr1Spec> {
        Pri5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 6. UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_6(&mut self) -> Pri6W<Shpr1Spec> {
        Pri6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pri_7(&mut self) -> Pri7W<Shpr1Spec> {
        Pri7W::new(self, 24)
    }
}
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shpr1Spec;
impl crate::RegisterSpec for Shpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr1::R`](R) reader structure"]
impl crate::Readable for Shpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`shpr1::W`](W) writer structure"]
impl crate::Writable for Shpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR1 to value 0"]
impl crate::Resettable for Shpr1Spec {
    const RESET_VALUE: u32 = 0;
}
