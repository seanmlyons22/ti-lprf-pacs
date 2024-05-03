#[doc = "Register `SHDW_SCAN_DATA1_CRC` reader"]
pub type R = crate::R<ShdwScanData1CrcSpec>;
#[doc = "Register `SHDW_SCAN_DATA1_CRC` writer"]
pub type W = crate::W<ShdwScanData1CrcSpec>;
#[doc = "Field `TAP_DAP_LOCK_N` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TapDapLockNR = crate::BitReader;
#[doc = "Field `TAP_DAP_LOCK_N` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TapDapLockNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - 8:1\\]
Internal. Only to be used through TI provided API."]
pub type CrcR = crate::FieldReader;
#[doc = "Field `CRC` writer - 8:1\\]
Internal. Only to be used through TI provided API."]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLASH_RDY` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type FlashRdyR = crate::BitReader;
#[doc = "Field `FLASH_RDY` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type FlashRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tap_dap_lock_n(&self) -> TapDapLockNR {
        TapDapLockNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - 8:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FlashRdyR {
        FlashRdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tap_dap_lock_n(&mut self) -> TapDapLockNW<ShdwScanData1CrcSpec> {
        TapDapLockNW::new(self, 0)
    }
    #[doc = "Bits 1:8 - 8:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<ShdwScanData1CrcSpec> {
        CrcW::new(self, 1)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flash_rdy(&mut self) -> FlashRdyW<ShdwScanData1CrcSpec> {
        FlashRdyW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_scan_data1_crc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_scan_data1_crc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwScanData1CrcSpec;
impl crate::RegisterSpec for ShdwScanData1CrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_scan_data1_crc::R`](R) reader structure"]
impl crate::Readable for ShdwScanData1CrcSpec {}
#[doc = "`write(|w| ..)` method takes [`shdw_scan_data1_crc::W`](W) writer structure"]
impl crate::Writable for ShdwScanData1CrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_SCAN_DATA1_CRC to value 0"]
impl crate::Resettable for ShdwScanData1CrcSpec {
    const RESET_VALUE: u32 = 0;
}
