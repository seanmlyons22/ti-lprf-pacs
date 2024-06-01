#[doc = "Register `CSPSR` reader"]
pub type R = crate::R<CspsrSpec>;
#[doc = "Register `CSPSR` writer"]
pub type W = crate::W<CspsrSpec>;
#[doc = "Field `ONE` reader - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWO` reader - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type TwoR = crate::BitReader;
#[doc = "Field `TWO` writer - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type TwoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THREE` reader - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type ThreeR = crate::BitReader;
#[doc = "Field `THREE` writer - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type ThreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUR` reader - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type FourR = crate::BitReader;
#[doc = "Field `FOUR` writer - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type FourW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn two(&self) -> TwoR {
        TwoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn three(&self) -> ThreeR {
        ThreeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn four(&self) -> FourR {
        FourR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> OneW<CspsrSpec> {
        OneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn two(&mut self) -> TwoW<CspsrSpec> {
        TwoW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn three(&mut self) -> ThreeW<CspsrSpec> {
        ThreeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn four(&mut self) -> FourW<CspsrSpec> {
        FourW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CspsrSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cspsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspsrSpec;
impl crate::RegisterSpec for CspsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspsr::R`](R) reader structure"]
impl crate::Readable for CspsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cspsr::W`](W) writer structure"]
impl crate::Writable for CspsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSPSR to value 0x01"]
impl crate::Resettable for CspsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
