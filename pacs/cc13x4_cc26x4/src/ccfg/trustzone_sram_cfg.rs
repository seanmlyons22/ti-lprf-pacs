#[doc = "Register `TRUSTZONE_SRAM_CFG` reader"]
pub type R = crate::R<TrustzoneSramCfgSpec>;
#[doc = "Register `TRUSTZONE_SRAM_CFG` writer"]
pub type W = crate::W<TrustzoneSramCfgSpec>;
#[doc = "Field `NSCADDR_BOUNDARY` reader - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NscaddrBoundaryR = crate::FieldReader<u16>;
#[doc = "Field `NSCADDR_BOUNDARY` writer - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NscaddrBoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSADDR_BOUNDARY` reader - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NsaddrBoundaryR = crate::FieldReader<u16>;
#[doc = "Field `NSADDR_BOUNDARY` writer - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NsaddrBoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    pub fn nscaddr_boundary(&self) -> NscaddrBoundaryR {
        NscaddrBoundaryR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    pub fn nsaddr_boundary(&self) -> NsaddrBoundaryR {
        NsaddrBoundaryR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn nscaddr_boundary(&mut self) -> NscaddrBoundaryW<TrustzoneSramCfgSpec> {
        NscaddrBoundaryW::new(self, 0)
    }
    #[doc = "Bits 9:17 - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn nsaddr_boundary(&mut self) -> NsaddrBoundaryW<TrustzoneSramCfgSpec> {
        NsaddrBoundaryW::new(self, 9)
    }
}
#[doc = "Trustzone configuration register for MCU SRAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trustzone_sram_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trustzone_sram_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrustzoneSramCfgSpec;
impl crate::RegisterSpec for TrustzoneSramCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trustzone_sram_cfg::R`](R) reader structure"]
impl crate::Readable for TrustzoneSramCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`trustzone_sram_cfg::W`](W) writer structure"]
impl crate::Writable for TrustzoneSramCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRUSTZONE_SRAM_CFG to value 0xffff_ffff"]
impl crate::Resettable for TrustzoneSramCfgSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
