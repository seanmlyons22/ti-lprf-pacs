#[doc = "Register `WUFLAGSCLR` reader"]
pub type R = crate::R<WuflagsclrSpec>;
#[doc = "Register `WUFLAGSCLR` writer"]
pub type W = crate::W<WuflagsclrSpec>;
#[doc = "Field `PROG_WU0` reader - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
pub type ProgWu0R = crate::BitReader;
#[doc = "Field `PROG_WU0` writer - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
pub type ProgWu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_WU1` reader - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
pub type ProgWu1R = crate::BitReader;
#[doc = "Field `PROG_WU1` writer - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
pub type ProgWu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_WU2` reader - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
pub type ProgWu2R = crate::BitReader;
#[doc = "Field `PROG_WU2` writer - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
pub type ProgWu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_WU3` reader - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
pub type ProgWu3R = crate::BitReader;
#[doc = "Field `PROG_WU3` writer - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
pub type ProgWu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU0` reader - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
pub type SwWu0R = crate::BitReader;
#[doc = "Field `SW_WU0` writer - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
pub type SwWu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU1` reader - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
pub type SwWu1R = crate::BitReader;
#[doc = "Field `SW_WU1` writer - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
pub type SwWu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU2` reader - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
pub type SwWu2R = crate::BitReader;
#[doc = "Field `SW_WU2` writer - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
pub type SwWu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU3` reader - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
pub type SwWu3R = crate::BitReader;
#[doc = "Field `SW_WU3` writer - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
pub type SwWu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
    #[inline(always)]
    pub fn prog_wu0(&self) -> ProgWu0R {
        ProgWu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
    #[inline(always)]
    pub fn prog_wu1(&self) -> ProgWu1R {
        ProgWu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
    #[inline(always)]
    pub fn prog_wu2(&self) -> ProgWu2R {
        ProgWu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
    #[inline(always)]
    pub fn prog_wu3(&self) -> ProgWu3R {
        ProgWu3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SwWu0R {
        SwWu0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SwWu1R {
        SwWu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SwWu2R {
        SwWu2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
    #[inline(always)]
    pub fn sw_wu3(&self) -> SwWu3R {
        SwWu3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Programmable wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.PROG_WU0. Keep high until WUFLAGS.PROG_WU0 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU0 to 0 when PROGWU0CFG.EN is 0, then set PROGWU0CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu0(&mut self) -> ProgWu0W<WuflagsclrSpec> {
        ProgWu0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.PROG_WU1. Keep high until WUFLAGS.PROG_WU1 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU1 to 0 when PROGWU1CFG.EN is 0, then set PROGWU1CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu1(&mut self) -> ProgWu1W<WuflagsclrSpec> {
        ProgWu1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.PROG_WU2. Keep high until WUFLAGS.PROG_WU2 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU2 to 0 when PROGWU2CFG.EN is 0, then set PROGWU2CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu2(&mut self) -> ProgWu2W<WuflagsclrSpec> {
        ProgWu2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.PROG_WU3. Keep high until WUFLAGS.PROG_WU3 is 0. The wakeup flag becomes edge sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 1. The wakeup flag becomes level sensitive if you write PROG_WU3 to 0 when PROGWU3CFG.EN is 0, then set PROGWU3CFG.EN."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu3(&mut self) -> ProgWu3W<WuflagsclrSpec> {
        ProgWu3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear software wakeup flag 0. 0: No effect. 1: Clear WUFLAGS.SW_WU0. Keep high until WUFLAGS.SW_WU0 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu0(&mut self) -> SwWu0W<WuflagsclrSpec> {
        SwWu0W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear software wakeup flag 1. 0: No effect. 1: Clear WUFLAGS.SW_WU1. Keep high until WUFLAGS.SW_WU1 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu1(&mut self) -> SwWu1W<WuflagsclrSpec> {
        SwWu1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear software wakeup flag 2. 0: No effect. 1: Clear WUFLAGS.SW_WU2. Keep high until WUFLAGS.SW_WU2 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu2(&mut self) -> SwWu2W<WuflagsclrSpec> {
        SwWu2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear software wakeup flag 3. 0: No effect. 1: Clear WUFLAGS.SW_WU3. Keep high until WUFLAGS.SW_WU3 is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu3(&mut self) -> SwWu3W<WuflagsclrSpec> {
        SwWu3W::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<WuflagsclrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuflagsclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuflagsclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuflagsclrSpec;
impl crate::RegisterSpec for WuflagsclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuflagsclr::R`](R) reader structure"]
impl crate::Readable for WuflagsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`wuflagsclr::W`](W) writer structure"]
impl crate::Writable for WuflagsclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUFLAGSCLR to value 0x0f"]
impl crate::Resettable for WuflagsclrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
