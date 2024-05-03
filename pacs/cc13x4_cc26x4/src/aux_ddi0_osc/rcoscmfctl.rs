#[doc = "Register `RCOSCMFCTL` reader"]
pub type R = crate::R<RcoscmfctlSpec>;
#[doc = "Register `RCOSCMFCTL` writer"]
pub type W = crate::W<RcoscmfctlSpec>;
#[doc = "Field `RCOSC_MF_BIAS_ADJ` reader - 3:0\\]
Adjusts bias current to RCOSC_MF. The trim has binary encoding with MSB inverted. 0x5 Minimum current 0x6 Default -10 0x7 Default -9 . . . 0xF Default -1 0x0 Default current 0x1 Default +1 0x2 Default +2 0x3 Default +3 0x4 Maximum current"]
pub type RcoscMfBiasAdjR = crate::FieldReader;
#[doc = "Field `RCOSC_MF_BIAS_ADJ` writer - 3:0\\]
Adjusts bias current to RCOSC_MF. The trim has binary encoding with MSB inverted. 0x5 Minimum current 0x6 Default -10 0x7 Default -9 . . . 0xF Default -1 0x0 Default current 0x1 Default +1 0x2 Default +2 0x3 Default +3 0x4 Maximum current"]
pub type RcoscMfBiasAdjW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RCOSC_MF_RES_FINE` reader - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
pub type RcoscMfResFineR = crate::FieldReader;
#[doc = "Field `RCOSC_MF_RES_FINE` writer - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
pub type RcoscMfResFineW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RCOSC_MF_RES_COARSE` reader - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
pub type RcoscMfResCoarseR = crate::FieldReader;
#[doc = "Field `RCOSC_MF_RES_COARSE` writer - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
pub type RcoscMfResCoarseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RCOSC_MF_REG_SEL` reader - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
pub type RcoscMfRegSelR = crate::BitReader;
#[doc = "Field `RCOSC_MF_REG_SEL` writer - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
pub type RcoscMfRegSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSC_MF_CAP_ARRAY` reader - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
pub type RcoscMfCapArrayR = crate::FieldReader;
#[doc = "Field `RCOSC_MF_CAP_ARRAY` writer - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
pub type RcoscMfCapArrayW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPARE16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare16R = crate::FieldReader<u16>;
#[doc = "Field `SPARE16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Adjusts bias current to RCOSC_MF. The trim has binary encoding with MSB inverted. 0x5 Minimum current 0x6 Default -10 0x7 Default -9 . . . 0xF Default -1 0x0 Default current 0x1 Default +1 0x2 Default +2 0x3 Default +3 0x4 Maximum current"]
    #[inline(always)]
    pub fn rcosc_mf_bias_adj(&self) -> RcoscMfBiasAdjR {
        RcoscMfBiasAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline(always)]
    pub fn rcosc_mf_res_fine(&self) -> RcoscMfResFineR {
        RcoscMfResFineR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline(always)]
    pub fn rcosc_mf_res_coarse(&self) -> RcoscMfResCoarseR {
        RcoscMfResCoarseR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
    #[inline(always)]
    pub fn rcosc_mf_reg_sel(&self) -> RcoscMfRegSelR {
        RcoscMfRegSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline(always)]
    pub fn rcosc_mf_cap_array(&self) -> RcoscMfCapArrayR {
        RcoscMfCapArrayR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> Spare16R {
        Spare16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Adjusts bias current to RCOSC_MF. The trim has binary encoding with MSB inverted. 0x5 Minimum current 0x6 Default -10 0x7 Default -9 . . . 0xF Default -1 0x0 Default current 0x1 Default +1 0x2 Default +2 0x3 Default +3 0x4 Maximum current"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_bias_adj(&mut self) -> RcoscMfBiasAdjW<RcoscmfctlSpec> {
        RcoscMfBiasAdjW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_res_fine(&mut self) -> RcoscMfResFineW<RcoscmfctlSpec> {
        RcoscMfResFineW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_res_coarse(&mut self) -> RcoscMfResCoarseW<RcoscmfctlSpec> {
        RcoscMfResCoarseW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_reg_sel(&mut self) -> RcoscMfRegSelW<RcoscmfctlSpec> {
        RcoscMfRegSelW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_cap_array(&mut self) -> RcoscMfCapArrayW<RcoscmfctlSpec> {
        RcoscMfCapArrayW::new(self, 9)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare16(&mut self) -> Spare16W<RcoscmfctlSpec> {
        Spare16W::new(self, 16)
    }
}
#[doc = "RCOSC_MF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcoscmfctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcoscmfctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcoscmfctlSpec;
impl crate::RegisterSpec for RcoscmfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcoscmfctl::R`](R) reader structure"]
impl crate::Readable for RcoscmfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rcoscmfctl::W`](W) writer structure"]
impl crate::Writable for RcoscmfctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCOSCMFCTL to value 0"]
impl crate::Resettable for RcoscmfctlSpec {
    const RESET_VALUE: u32 = 0;
}
