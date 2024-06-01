#[doc = "Register `SPIMCFG` reader"]
pub type R = crate::R<SpimcfgSpec>;
#[doc = "Register `SPIMCFG` writer"]
pub type W = crate::W<SpimcfgSpec>;
#[doc = "Field `POL` reader - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHA` reader - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
pub type PhaR = crate::BitReader;
#[doc = "Field `PHA` writer - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
pub type PhaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
    #[inline(always)]
    pub fn pha(&self) -> PhaR {
        PhaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 2) & 0x3f) as u8)
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
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<SpimcfgSpec> {
        PolW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
    #[inline(always)]
    #[must_use]
    pub fn pha(&mut self) -> PhaW<SpimcfgSpec> {
        PhaW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<SpimcfgSpec> {
        DivW::new(self, 2)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SpimcfgSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spimcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spimcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpimcfgSpec;
impl crate::RegisterSpec for SpimcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spimcfg::R`](R) reader structure"]
impl crate::Readable for SpimcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spimcfg::W`](W) writer structure"]
impl crate::Writable for SpimcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIMCFG to value 0"]
impl crate::Resettable for SpimcfgSpec {
    const RESET_VALUE: u32 = 0;
}
