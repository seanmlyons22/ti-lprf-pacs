#[doc = "Register `WUFLAGS` reader"]
pub type R = crate::R<WuflagsSpec>;
#[doc = "Register `WUFLAGS` writer"]
pub type W = crate::W<WuflagsSpec>;
#[doc = "Field `PROG_WU0` reader - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
pub type ProgWu0R = crate::BitReader;
#[doc = "Field `PROG_WU0` writer - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
pub type ProgWu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_WU1` reader - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
pub type ProgWu1R = crate::BitReader;
#[doc = "Field `PROG_WU1` writer - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
pub type ProgWu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_WU2` reader - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
pub type ProgWu2R = crate::BitReader;
#[doc = "Field `PROG_WU2` writer - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
pub type ProgWu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_WU3` reader - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
pub type ProgWu3R = crate::BitReader;
#[doc = "Field `PROG_WU3` writer - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
pub type ProgWu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU0` reader - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
pub type SwWu0R = crate::BitReader;
#[doc = "Field `SW_WU0` writer - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
pub type SwWu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU1` reader - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
pub type SwWu1R = crate::BitReader;
#[doc = "Field `SW_WU1` writer - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
pub type SwWu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU2` reader - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
pub type SwWu2R = crate::BitReader;
#[doc = "Field `SW_WU2` writer - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
pub type SwWu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_WU3` reader - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
pub type SwWu3R = crate::BitReader;
#[doc = "Field `SW_WU3` writer - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
pub type SwWu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
    #[inline(always)]
    pub fn prog_wu0(&self) -> ProgWu0R {
        ProgWu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
    #[inline(always)]
    pub fn prog_wu1(&self) -> ProgWu1R {
        ProgWu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
    #[inline(always)]
    pub fn prog_wu2(&self) -> ProgWu2R {
        ProgWu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
    #[inline(always)]
    pub fn prog_wu3(&self) -> ProgWu3R {
        ProgWu3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SwWu0R {
        SwWu0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SwWu1R {
        SwWu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SwWu2R {
        SwWu2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
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
Programmable wakeup 0. 0: Programmable wakeup 0 not triggered. 1: Programmable wakeup 0 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu0(&mut self) -> ProgWu0W<WuflagsSpec> {
        ProgWu0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmable wakeup 1. 0: Programmable wakeup 1 not triggered. 1: Programmable wakeup 1 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu1(&mut self) -> ProgWu1W<WuflagsSpec> {
        ProgWu1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Programmable wakeup 2. 0: Programmable wakeup 2 not triggered. 1: Programmable wakeup 2 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu2(&mut self) -> ProgWu2W<WuflagsSpec> {
        ProgWu2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Programmable wakeup 3. 0: Programmable wakeup 3 not triggered. 1: Programmable wakeup 3 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn prog_wu3(&mut self) -> ProgWu3W<WuflagsSpec> {
        ProgWu3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Software wakeup 0 flag. 0: Software wakeup 0 not triggered. 1: Software wakeup 0 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu0(&mut self) -> SwWu0W<WuflagsSpec> {
        SwWu0W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Software wakeup 1 flag. 0: Software wakeup 1 not triggered. 1: Software wakeup 1 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu1(&mut self) -> SwWu1W<WuflagsSpec> {
        SwWu1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Software wakeup 2 flag. 0: Software wakeup 2 not triggered. 1: Software wakeup 2 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu2(&mut self) -> SwWu2W<WuflagsSpec> {
        SwWu2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software wakeup 3 flag. 0: Software wakeup 3 not triggered. 1: Software wakeup 3 triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu3(&mut self) -> SwWu3W<WuflagsSpec> {
        SwWu3W::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<WuflagsSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuflagsSpec;
impl crate::RegisterSpec for WuflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuflags::R`](R) reader structure"]
impl crate::Readable for WuflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`wuflags::W`](W) writer structure"]
impl crate::Writable for WuflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUFLAGS to value 0"]
impl crate::Resettable for WuflagsSpec {
    const RESET_VALUE: u32 = 0;
}
