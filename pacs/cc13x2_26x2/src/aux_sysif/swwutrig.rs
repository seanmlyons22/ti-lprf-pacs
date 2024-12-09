#[doc = "Register `SWWUTRIG` reader"]
pub type R = crate::R<SwwutrigSpec>;
#[doc = "Register `SWWUTRIG` writer"]
pub type W = crate::W<SwwutrigSpec>;
#[doc = "Field `SW_WU0` writer - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
pub type SwWu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU1` writer - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
pub type SwWu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU2` writer - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
pub type SwWu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU3` writer - 3:3\\]
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
pub type SwWu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu0(&mut self) -> SwWu0W<SwwutrigSpec> {
        SwWu0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu1(&mut self) -> SwWu1W<SwwutrigSpec> {
        SwWu1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu2(&mut self) -> SwWu2W<SwwutrigSpec> {
        SwWu2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu3(&mut self) -> SwWu3W<SwwutrigSpec> {
        SwWu3W::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SwwutrigSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swwutrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swwutrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwwutrigSpec;
impl crate::RegisterSpec for SwwutrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swwutrig::R`](R) reader structure"]
impl crate::Readable for SwwutrigSpec {}
#[doc = "`write(|w| ..)` method takes [`swwutrig::W`](W) writer structure"]
impl crate::Writable for SwwutrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWWUTRIG to value 0"]
impl crate::Resettable for SwwutrigSpec {
    const RESET_VALUE: u32 = 0;
}
