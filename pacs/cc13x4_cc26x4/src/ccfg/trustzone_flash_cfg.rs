#[doc = "Register `TRUSTZONE_FLASH_CFG` reader"]
pub type R = crate::R<TrustzoneFlashCfgSpec>;
#[doc = "Register `TRUSTZONE_FLASH_CFG` writer"]
pub type W = crate::W<TrustzoneFlashCfgSpec>;
#[doc = "Field `NSCADDR_BOUNDARY` reader - 9:0\\]
Value will be written to PRCM:NVMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NscaddrBoundaryR = crate::FieldReader<u16>;
#[doc = "Field `NSCADDR_BOUNDARY` writer - 9:0\\]
Value will be written to PRCM:NVMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NscaddrBoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NSADDR_BOUNDARY` reader - 16:10\\]
Value will be written to PRCM:NVMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NsaddrBoundaryR = crate::FieldReader;
#[doc = "Field `NSADDR_BOUNDARY` writer - 16:10\\]
Value will be written to PRCM:NVMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NsaddrBoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Value will be written to PRCM:NVMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    pub fn nscaddr_boundary(&self) -> NscaddrBoundaryR {
        NscaddrBoundaryR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Value will be written to PRCM:NVMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    pub fn nsaddr_boundary(&self) -> NsaddrBoundaryR {
        NsaddrBoundaryR::new(((self.bits >> 10) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Value will be written to PRCM:NVMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn nscaddr_boundary(&mut self) -> NscaddrBoundaryW<TrustzoneFlashCfgSpec> {
        NscaddrBoundaryW::new(self, 0)
    }
    #[doc = "Bits 10:16 - 16:10\\]
Value will be written to PRCM:NVMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn nsaddr_boundary(&mut self) -> NsaddrBoundaryW<TrustzoneFlashCfgSpec> {
        NsaddrBoundaryW::new(self, 10)
    }
}
#[doc = "Trustzone configuration register for flash\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trustzone_flash_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trustzone_flash_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrustzoneFlashCfgSpec;
impl crate::RegisterSpec for TrustzoneFlashCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trustzone_flash_cfg::R`](R) reader structure"]
impl crate::Readable for TrustzoneFlashCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`trustzone_flash_cfg::W`](W) writer structure"]
impl crate::Writable for TrustzoneFlashCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRUSTZONE_FLASH_CFG to value 0xffff_ffff"]
impl crate::Resettable for TrustzoneFlashCfgSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
