#[doc = "Register `ATESTCTL` reader"]
pub type R = crate::R<AtestctlSpec>;
#[doc = "Register `ATESTCTL` writer"]
pub type W = crate::W<AtestctlSpec>;
#[doc = "Field `RESERVED0` reader - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED0` writer - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ATEST_RCOSCMF` reader - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
pub type AtestRcoscmfR = crate::FieldReader;
#[doc = "Field `ATEST_RCOSCMF` writer - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
pub type AtestRcoscmfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TEST_RCOSCMF` reader - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
pub type TestRcoscmfR = crate::FieldReader;
#[doc = "Field `TEST_RCOSCMF` writer - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
pub type TestRcoscmfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED16` reader - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SCLK_LF_AUX_EN` reader - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SclkLfAuxEnR = crate::BitReader;
#[doc = "Field `SCLK_LF_AUX_EN` writer - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
pub type SclkLfAuxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
    #[inline(always)]
    pub fn atest_rcoscmf(&self) -> AtestRcoscmfR {
        AtestRcoscmfR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
    #[inline(always)]
    pub fn test_rcoscmf(&self) -> TestRcoscmfR {
        TestRcoscmfR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SclkLfAuxEnR {
        SclkLfAuxEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AtestctlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
    #[inline(always)]
    #[must_use]
    pub fn atest_rcoscmf(&mut self) -> AtestRcoscmfW<AtestctlSpec> {
        AtestRcoscmfW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
    #[inline(always)]
    #[must_use]
    pub fn test_rcoscmf(&mut self) -> TestRcoscmfW<AtestctlSpec> {
        TestRcoscmfW::new(self, 14)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<AtestctlSpec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_aux_en(&mut self) -> SclkLfAuxEnW<AtestctlSpec> {
        SclkLfAuxEnW::new(self, 31)
    }
}
#[doc = "Analog Test Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atestctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atestctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtestctlSpec;
impl crate::RegisterSpec for AtestctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atestctl::R`](R) reader structure"]
impl crate::Readable for AtestctlSpec {}
#[doc = "`write(|w| ..)` method takes [`atestctl::W`](W) writer structure"]
impl crate::Writable for AtestctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATESTCTL to value 0"]
impl crate::Resettable for AtestctlSpec {
    const RESET_VALUE: u32 = 0;
}
