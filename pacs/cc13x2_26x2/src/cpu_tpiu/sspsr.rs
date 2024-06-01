#[doc = "Register `SSPSR` reader"]
pub type R = crate::R<SspsrSpec>;
#[doc = "Register `SSPSR` writer"]
pub type W = crate::W<SspsrSpec>;
#[doc = "Field `ONE` reader - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWO` reader - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
pub type TwoR = crate::BitReader;
#[doc = "Field `TWO` writer - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
pub type TwoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THREE` reader - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
pub type ThreeR = crate::BitReader;
#[doc = "Field `THREE` writer - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
pub type ThreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUR` reader - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
pub type FourR = crate::BitReader;
#[doc = "Field `FOUR` writer - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
pub type FourW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn two(&self) -> TwoR {
        TwoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn three(&self) -> ThreeR {
        ThreeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
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
1-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> OneW<SspsrSpec> {
        OneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    #[must_use]
    pub fn two(&mut self) -> TwoW<SspsrSpec> {
        TwoW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    #[must_use]
    pub fn three(&mut self) -> ThreeW<SspsrSpec> {
        ThreeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    #[must_use]
    pub fn four(&mut self) -> FourW<SspsrSpec> {
        FourW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SspsrSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspsrSpec;
impl crate::RegisterSpec for SspsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspsr::R`](R) reader structure"]
impl crate::Readable for SspsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sspsr::W`](W) writer structure"]
impl crate::Writable for SspsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPSR to value 0x0b"]
impl crate::Resettable for SspsrSpec {
    const RESET_VALUE: u32 = 0x0b;
}
