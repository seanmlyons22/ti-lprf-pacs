#[doc = "Register `OSCIMSC` reader"]
pub type R = crate::R<OscimscSpec>;
#[doc = "Register `OSCIMSC` writer"]
pub type W = crate::W<OscimscSpec>;
#[doc = "Field `RCOSCHFIM` reader - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
pub type RcoschfimR = crate::BitReader;
#[doc = "Field `RCOSCHFIM` writer - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
pub type RcoschfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCHFIM` reader - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
pub type XoschfimR = crate::BitReader;
#[doc = "Field `XOSCHFIM` writer - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
pub type XoschfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCLFIM` reader - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
pub type RcosclfimR = crate::BitReader;
#[doc = "Field `RCOSCLFIM` writer - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
pub type RcosclfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCDLFIM` reader - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
pub type RcoscdlfimR = crate::BitReader;
#[doc = "Field `RCOSCDLFIM` writer - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
pub type RcoscdlfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCLFIM` reader - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
pub type XosclfimR = crate::BitReader;
#[doc = "Field `XOSCLFIM` writer - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
pub type XosclfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCDLFIM` reader - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
pub type XoscdlfimR = crate::BitReader;
#[doc = "Field `XOSCDLFIM` writer - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
pub type XoscdlfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSRCDONEIM` reader - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
pub type LfsrcdoneimR = crate::BitReader;
#[doc = "Field `LFSRCDONEIM` writer - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
pub type LfsrcdoneimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFSRCPENDIM` reader - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
pub type HfsrcpendimR = crate::BitReader;
#[doc = "Field `HFSRCPENDIM` writer - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
pub type HfsrcpendimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
    #[inline(always)]
    pub fn rcoschfim(&self) -> RcoschfimR {
        RcoschfimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
    #[inline(always)]
    pub fn xoschfim(&self) -> XoschfimR {
        XoschfimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
    #[inline(always)]
    pub fn rcosclfim(&self) -> RcosclfimR {
        RcosclfimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
    #[inline(always)]
    pub fn rcoscdlfim(&self) -> RcoscdlfimR {
        RcoscdlfimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
    #[inline(always)]
    pub fn xosclfim(&self) -> XosclfimR {
        XosclfimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
    #[inline(always)]
    pub fn xoscdlfim(&self) -> XoscdlfimR {
        XoscdlfimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
    #[inline(always)]
    pub fn lfsrcdoneim(&self) -> LfsrcdoneimR {
        LfsrcdoneimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
    #[inline(always)]
    pub fn hfsrcpendim(&self) -> HfsrcpendimR {
        HfsrcpendimR::new(((self.bits >> 7) & 1) != 0)
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
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfim(&mut self) -> RcoschfimW<OscimscSpec> {
        RcoschfimW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn xoschfim(&mut self) -> XoschfimW<OscimscSpec> {
        XoschfimW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn rcosclfim(&mut self) -> RcosclfimW<OscimscSpec> {
        RcosclfimW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn rcoscdlfim(&mut self) -> RcoscdlfimW<OscimscSpec> {
        RcoscdlfimW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn xosclfim(&mut self) -> XosclfimW<OscimscSpec> {
        XosclfimW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn xoscdlfim(&mut self) -> XoscdlfimW<OscimscSpec> {
        XoscdlfimW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn lfsrcdoneim(&mut self) -> LfsrcdoneimW<OscimscSpec> {
        LfsrcdoneimW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn hfsrcpendim(&mut self) -> HfsrcpendimW<OscimscSpec> {
        HfsrcpendimW::new(self, 7)
    }
}
#[doc = "Oscillator Interrupt Mask Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscimsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscimsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscimscSpec;
impl crate::RegisterSpec for OscimscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscimsc::R`](R) reader structure"]
impl crate::Readable for OscimscSpec {}
#[doc = "`write(|w| ..)` method takes [`oscimsc::W`](W) writer structure"]
impl crate::Writable for OscimscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCIMSC to value 0x36"]
impl crate::Resettable for OscimscSpec {
    const RESET_VALUE: u32 = 0x36;
}
