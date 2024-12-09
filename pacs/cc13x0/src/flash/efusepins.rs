#[doc = "Register `EFUSEPINS` reader"]
pub type R = crate::R<EfusepinsSpec>;
#[doc = "Register `EFUSEPINS` writer"]
pub type W = crate::W<EfusepinsSpec>;
#[doc = "Field `SYS_WS_READ_STATES` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SysWsReadStatesR = crate::FieldReader;
#[doc = "Field `SYS_REPAIR_EN` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type SysRepairEnR = crate::FieldReader;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type SysDieidAutoloadEnR = crate::BitReader;
#[doc = "Field `EFC_FCLRZ` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type EfcFclrzR = crate::BitReader;
#[doc = "Field `EFC_READY` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EfcReadyR = crate::BitReader;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type SysEccOverrideEnR = crate::BitReader;
#[doc = "Field `EFC_AUTOLOAD_ERROR` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EfcAutoloadErrorR = crate::BitReader;
#[doc = "Field `EFC_INSTRUCTION_ERROR` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EfcInstructionErrorR = crate::BitReader;
#[doc = "Field `EFC_INSTRUCTION_INFO` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type EfcInstructionInfoR = crate::BitReader;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SysEccSelfTestEnR = crate::BitReader;
#[doc = "Field `EFC_SELF_TEST_ERROR` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type EfcSelfTestErrorR = crate::BitReader;
#[doc = "Field `EFC_SELF_TEST_DONE` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type EfcSelfTestDoneR = crate::BitReader;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SysWsReadStatesR {
        SysWsReadStatesR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SysRepairEnR {
        SysRepairEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SysDieidAutoloadEnR {
        SysDieidAutoloadEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fclrz(&self) -> EfcFclrzR {
        EfcFclrzR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_ready(&self) -> EfcReadyR {
        EfcReadyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SysEccOverrideEnR {
        SysEccOverrideEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EfcAutoloadErrorR {
        EfcAutoloadErrorR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EfcInstructionErrorR {
        EfcInstructionErrorR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EfcInstructionInfoR {
        EfcInstructionInfoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SysEccSelfTestEnR {
        SysEccSelfTestEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EfcSelfTestErrorR {
        EfcSelfTestErrorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_done(&self) -> EfcSelfTestDoneR {
        EfcSelfTestDoneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusepins::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusepins::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusepinsSpec;
impl crate::RegisterSpec for EfusepinsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efusepins::R`](R) reader structure"]
impl crate::Readable for EfusepinsSpec {}
#[doc = "`write(|w| ..)` method takes [`efusepins::W`](W) writer structure"]
impl crate::Writable for EfusepinsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSEPINS to value 0"]
impl crate::Resettable for EfusepinsSpec {
    const RESET_VALUE: u32 = 0;
}
