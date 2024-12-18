#[doc = "Register `OSCICR` reader"]
pub type R = crate::R<OscicrSpec>;
#[doc = "Register `OSCICR` writer"]
pub type W = crate::W<OscicrSpec>;
#[doc = "Field `RCOSCHFC` writer - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
pub type RcoschfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCHFC` writer - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
pub type XoschfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCLFC` writer - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
pub type RcosclfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCDLFC` writer - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
pub type RcoscdlfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCLFC` writer - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
pub type XosclfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCDLFC` writer - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
pub type XoscdlfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSRCDONEC` writer - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
pub type LfsrcdonecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFSRCPENDC` writer - 7:7\\]
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
pub type HfsrcpendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfc(&mut self) -> RcoschfcW<OscicrSpec> {
        RcoschfcW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn xoschfc(&mut self) -> XoschfcW<OscicrSpec> {
        XoschfcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rcosclfc(&mut self) -> RcosclfcW<OscicrSpec> {
        RcosclfcW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rcoscdlfc(&mut self) -> RcoscdlfcW<OscicrSpec> {
        RcoscdlfcW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn xosclfc(&mut self) -> XosclfcW<OscicrSpec> {
        XosclfcW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn xoscdlfc(&mut self) -> XoscdlfcW<OscicrSpec> {
        XoscdlfcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn lfsrcdonec(&mut self) -> LfsrcdonecW<OscicrSpec> {
        LfsrcdonecW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn hfsrcpendc(&mut self) -> HfsrcpendcW<OscicrSpec> {
        HfsrcpendcW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<OscicrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Oscillator Raw Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscicrSpec;
impl crate::RegisterSpec for OscicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscicr::R`](R) reader structure"]
impl crate::Readable for OscicrSpec {}
#[doc = "`write(|w| ..)` method takes [`oscicr::W`](W) writer structure"]
impl crate::Writable for OscicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCICR to value 0"]
impl crate::Resettable for OscicrSpec {
    const RESET_VALUE: u32 = 0;
}
