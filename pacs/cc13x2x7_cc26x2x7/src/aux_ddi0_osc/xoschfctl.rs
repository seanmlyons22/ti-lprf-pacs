#[doc = "Register `XOSCHFCTL` reader"]
pub type R = crate::R<XoschfctlSpec>;
#[doc = "Register `XOSCHFCTL` writer"]
pub type W = crate::W<XoschfctlSpec>;
#[doc = "Field `LP_BUF_ITRIM` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type LpBufItrimR = crate::FieldReader;
#[doc = "Field `LP_BUF_ITRIM` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type LpBufItrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_BUF_ITRIM` reader - 4:2\\]
Internal. Only to be used through TI provided API."]
pub type HpBufItrimR = crate::FieldReader;
#[doc = "Field `HP_BUF_ITRIM` writer - 4:2\\]
Internal. Only to be used through TI provided API."]
pub type HpBufItrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED5` reader - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `RESERVED5` writer - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAK_DET_ITRIM` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type PeakDetItrimR = crate::FieldReader;
#[doc = "Field `PEAK_DET_ITRIM` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type PeakDetItrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCXO_MODE` reader - 12:12\\]
If this register is 1 when BYPASS is 1, this will enable clock qualification on the TCXO clock on external input. This register has no effect when BYPASS is 0."]
pub type TcxoModeR = crate::BitReader;
#[doc = "Field `TCXO_MODE` writer - 12:12\\]
If this register is 1 when BYPASS is 1, this will enable clock qualification on the TCXO clock on external input. This register has no effect when BYPASS is 0."]
pub type TcxoModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCXO_MODE_XOSC_HF_EN` reader - 13:13\\]
If this register is 1 when TCXO_MODE is 1, then the XOSC_HF is enabled, turning on the XOSC_HF bias current allowing a DC bias point to be provided to the clipped-sine wave clock signal on external input."]
pub type TcxoModeXoscHfEnR = crate::BitReader;
#[doc = "Field `TCXO_MODE_XOSC_HF_EN` writer - 13:13\\]
If this register is 1 when TCXO_MODE is 1, then the XOSC_HF is enabled, turning on the XOSC_HF bias current allowing a DC bias point to be provided to the clipped-sine wave clock signal on external input."]
pub type TcxoModeXoscHfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare14R = crate::FieldReader<u32>;
#[doc = "Field `SPARE14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LpBufItrimR {
        LpBufItrimR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HpBufItrimR {
        HpBufItrimR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PeakDetItrimR {
        PeakDetItrimR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
If this register is 1 when BYPASS is 1, this will enable clock qualification on the TCXO clock on external input. This register has no effect when BYPASS is 0."]
    #[inline(always)]
    pub fn tcxo_mode(&self) -> TcxoModeR {
        TcxoModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
If this register is 1 when TCXO_MODE is 1, then the XOSC_HF is enabled, turning on the XOSC_HF bias current allowing a DC bias point to be provided to the clipped-sine wave clock signal on external input."]
    #[inline(always)]
    pub fn tcxo_mode_xosc_hf_en(&self) -> TcxoModeXoscHfEnR {
        TcxoModeXoscHfEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare14(&self) -> Spare14R {
        Spare14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lp_buf_itrim(&mut self) -> LpBufItrimW<XoschfctlSpec> {
        LpBufItrimW::new(self, 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hp_buf_itrim(&mut self) -> HpBufItrimW<XoschfctlSpec> {
        HpBufItrimW::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<XoschfctlSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<XoschfctlSpec> {
        BypassW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<XoschfctlSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn peak_det_itrim(&mut self) -> PeakDetItrimW<XoschfctlSpec> {
        PeakDetItrimW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<XoschfctlSpec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
If this register is 1 when BYPASS is 1, this will enable clock qualification on the TCXO clock on external input. This register has no effect when BYPASS is 0."]
    #[inline(always)]
    #[must_use]
    pub fn tcxo_mode(&mut self) -> TcxoModeW<XoschfctlSpec> {
        TcxoModeW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
If this register is 1 when TCXO_MODE is 1, then the XOSC_HF is enabled, turning on the XOSC_HF bias current allowing a DC bias point to be provided to the clipped-sine wave clock signal on external input."]
    #[inline(always)]
    #[must_use]
    pub fn tcxo_mode_xosc_hf_en(&mut self) -> TcxoModeXoscHfEnW<XoschfctlSpec> {
        TcxoModeXoscHfEnW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare14(&mut self) -> Spare14W<XoschfctlSpec> {
        Spare14W::new(self, 14)
    }
}
#[doc = "XOSCHF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xoschfctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xoschfctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XoschfctlSpec;
impl crate::RegisterSpec for XoschfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xoschfctl::R`](R) reader structure"]
impl crate::Readable for XoschfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`xoschfctl::W`](W) writer structure"]
impl crate::Writable for XoschfctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XOSCHFCTL to value 0"]
impl crate::Resettable for XoschfctlSpec {
    const RESET_VALUE: u32 = 0;
}
