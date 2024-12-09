#[doc = "Register `SHDW_SCAN_MCU3_SEC` reader"]
pub type R = crate::R<ShdwScanMcu3SecSpec>;
#[doc = "Register `SHDW_SCAN_MCU3_SEC` writer"]
pub type W = crate::W<ShdwScanMcu3SecSpec>;
#[doc = "Field `ULL_MCU_RAM_1_REP_1` reader - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type UllMcuRam1Rep1R = crate::FieldReader<u16>;
#[doc = "Field `ULL_MCU_RAM_0_REP` reader - 22:11\\]
Internal. Only to be used through TI provided API."]
pub type UllMcuRam0RepR = crate::FieldReader<u16>;
#[doc = "Field `SECURITY` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type SecurityR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ull_mcu_ram_1_rep_1(&self) -> UllMcuRam1Rep1R {
        UllMcuRam1Rep1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:22 - 22:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ull_mcu_ram_0_rep(&self) -> UllMcuRam0RepR {
        UllMcuRam0RepR::new(((self.bits >> 11) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn security(&self) -> SecurityR {
        SecurityR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_scan_mcu3_sec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_scan_mcu3_sec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwScanMcu3SecSpec;
impl crate::RegisterSpec for ShdwScanMcu3SecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_scan_mcu3_sec::R`](R) reader structure"]
impl crate::Readable for ShdwScanMcu3SecSpec {}
#[doc = "`write(|w| ..)` method takes [`shdw_scan_mcu3_sec::W`](W) writer structure"]
impl crate::Writable for ShdwScanMcu3SecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_SCAN_MCU3_SEC to value 0"]
impl crate::Resettable for ShdwScanMcu3SecSpec {
    const RESET_VALUE: u32 = 0;
}
