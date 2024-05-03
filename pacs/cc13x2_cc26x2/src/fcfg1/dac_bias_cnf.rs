#[doc = "Register `DAC_BIAS_CNF` reader"]
pub type R = crate::R<DacBiasCnfSpec>;
#[doc = "Register `DAC_BIAS_CNF` writer"]
pub type W = crate::W<DacBiasCnfSpec>;
#[doc = "Field `RESERVED1` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPM_BIAS_BACKUP_EN` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type LpmBiasBackupEnR = crate::BitReader;
#[doc = "Field `LPM_BIAS_BACKUP_EN` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type LpmBiasBackupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_BIAS_WIDTH_TRIM` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type LpmBiasWidthTrimR = crate::FieldReader;
#[doc = "Field `LPM_BIAS_WIDTH_TRIM` writer - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type LpmBiasWidthTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LPM_TRIM_IOUT` reader - 17:12\\]
Internal. Only to be used through TI provided API."]
pub type LpmTrimIoutR = crate::FieldReader;
#[doc = "Field `LPM_TRIM_IOUT` writer - 17:12\\]
Internal. Only to be used through TI provided API."]
pub type LpmTrimIoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_backup_en(&self) -> LpmBiasBackupEnR {
        LpmBiasBackupEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_width_trim(&self) -> LpmBiasWidthTrimR {
        LpmBiasWidthTrimR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LpmTrimIoutR {
        LpmTrimIoutR::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DacBiasCnfSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_bias_backup_en(&mut self) -> LpmBiasBackupEnW<DacBiasCnfSpec> {
        LpmBiasBackupEnW::new(self, 8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_bias_width_trim(&mut self) -> LpmBiasWidthTrimW<DacBiasCnfSpec> {
        LpmBiasWidthTrimW::new(self, 9)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_trim_iout(&mut self) -> LpmTrimIoutW<DacBiasCnfSpec> {
        LpmTrimIoutW::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_bias_cnf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_bias_cnf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacBiasCnfSpec;
impl crate::RegisterSpec for DacBiasCnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_bias_cnf::R`](R) reader structure"]
impl crate::Readable for DacBiasCnfSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_bias_cnf::W`](W) writer structure"]
impl crate::Writable for DacBiasCnfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_BIAS_CNF to value 0xfffc_00ff"]
impl crate::Resettable for DacBiasCnfSpec {
    const RESET_VALUE: u32 = 0xfffc_00ff;
}
