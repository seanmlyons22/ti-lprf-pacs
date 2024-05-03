#[doc = "Register `FFCR` reader"]
pub type R = crate::R<FfcrSpec>;
#[doc = "Register `FFCR` writer"]
pub type W = crate::W<FfcrSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENFCONT` reader - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
pub type EnfcontR = crate::BitReader;
#[doc = "Field `ENFCONT` writer - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
pub type EnfcontW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRIGIN` reader - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
pub type TriginR = crate::BitReader;
#[doc = "Field `TRIGIN` writer - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
pub type TriginW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
    #[inline(always)]
    pub fn enfcont(&self) -> EnfcontR {
        EnfcontR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
    #[inline(always)]
    pub fn trigin(&self) -> TriginR {
        TriginR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<FfcrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enfcont(&mut self) -> EnfcontW<FfcrSpec> {
        EnfcontW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FfcrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn trigin(&mut self) -> TriginW<FfcrSpec> {
        TriginW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<FfcrSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfcrSpec;
impl crate::RegisterSpec for FfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffcr::R`](R) reader structure"]
impl crate::Readable for FfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffcr::W`](W) writer structure"]
impl crate::Writable for FfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFCR to value 0x0102"]
impl crate::Resettable for FfcrSpec {
    const RESET_VALUE: u32 = 0x0102;
}
