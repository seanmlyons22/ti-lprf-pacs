#[doc = "Register `BOUNDARY` reader"]
pub type R = crate::R<BoundarySpec>;
#[doc = "Register `BOUNDARY` writer"]
pub type W = crate::W<BoundarySpec>;
#[doc = "Field `INPUTENABLE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type InputenableR = crate::FieldReader;
#[doc = "Field `INPUTENABLE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type InputenableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYS_WS_READ_STATES` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SysWsReadStatesR = crate::FieldReader;
#[doc = "Field `SYS_WS_READ_STATES` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SysWsReadStatesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYS_REPAIR_EN` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type SysRepairEnR = crate::FieldReader;
#[doc = "Field `SYS_REPAIR_EN` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type SysRepairEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SysDieidAutoloadEnR = crate::BitReader;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SysDieidAutoloadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC_FDI` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EfcFdiR = crate::BitReader;
#[doc = "Field `EFC_FDI` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EfcFdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type SysEccOverrideEnR = crate::BitReader;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type SysEccOverrideEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SysEccSelfTestEnR = crate::BitReader;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SysEccSelfTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUTENABLE` reader - 17:14\\]
Internal. Only to be used through TI provided API."]
pub type OutputenableR = crate::FieldReader;
#[doc = "Field `OUTPUTENABLE` writer - 17:14\\]
Internal. Only to be used through TI provided API."]
pub type OutputenableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EFC_AUTOLOAD_ERROR` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type EfcAutoloadErrorR = crate::BitReader;
#[doc = "Field `EFC_AUTOLOAD_ERROR` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type EfcAutoloadErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC_INSTRUCTION_ERROR` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type EfcInstructionErrorR = crate::BitReader;
#[doc = "Field `EFC_INSTRUCTION_ERROR` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type EfcInstructionErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC_INSTRUCTION_INFO` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type EfcInstructionInfoR = crate::BitReader;
#[doc = "Field `EFC_INSTRUCTION_INFO` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type EfcInstructionInfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC_SELF_TEST_ERROR` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type EfcSelfTestErrorR = crate::BitReader;
#[doc = "Field `EFC_SELF_TEST_ERROR` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type EfcSelfTestErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISROW0` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type Disrow0R = crate::BitReader;
#[doc = "Field `DISROW0` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type Disrow0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inputenable(&self) -> InputenableR {
        InputenableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SysWsReadStatesR {
        SysWsReadStatesR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SysRepairEnR {
        SysRepairEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SysDieidAutoloadEnR {
        SysDieidAutoloadEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fdi(&self) -> EfcFdiR {
        EfcFdiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SysEccOverrideEnR {
        SysEccOverrideEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SysEccSelfTestEnR {
        SysEccSelfTestEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn outputenable(&self) -> OutputenableR {
        OutputenableR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EfcAutoloadErrorR {
        EfcAutoloadErrorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EfcInstructionErrorR {
        EfcInstructionErrorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EfcInstructionInfoR {
        EfcInstructionInfoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EfcSelfTestErrorR {
        EfcSelfTestErrorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disrow0(&self) -> Disrow0R {
        Disrow0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn inputenable(&mut self) -> InputenableW<BoundarySpec> {
        InputenableW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ws_read_states(&mut self) -> SysWsReadStatesW<BoundarySpec> {
        SysWsReadStatesW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_repair_en(&mut self) -> SysRepairEnW<BoundarySpec> {
        SysRepairEnW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_dieid_autoload_en(&mut self) -> SysDieidAutoloadEnW<BoundarySpec> {
        SysDieidAutoloadEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_fdi(&mut self) -> EfcFdiW<BoundarySpec> {
        EfcFdiW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ecc_override_en(&mut self) -> SysEccOverrideEnW<BoundarySpec> {
        SysEccOverrideEnW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ecc_self_test_en(&mut self) -> SysEccSelfTestEnW<BoundarySpec> {
        SysEccSelfTestEnW::new(self, 13)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn outputenable(&mut self) -> OutputenableW<BoundarySpec> {
        OutputenableW::new(self, 14)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_autoload_error(&mut self) -> EfcAutoloadErrorW<BoundarySpec> {
        EfcAutoloadErrorW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_instruction_error(&mut self) -> EfcInstructionErrorW<BoundarySpec> {
        EfcInstructionErrorW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_instruction_info(&mut self) -> EfcInstructionInfoW<BoundarySpec> {
        EfcInstructionInfoW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_self_test_error(&mut self) -> EfcSelfTestErrorW<BoundarySpec> {
        EfcSelfTestErrorW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<BoundarySpec> {
        SpareW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn disrow0(&mut self) -> Disrow0W<BoundarySpec> {
        Disrow0W::new(self, 23)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundary::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundary::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BoundarySpec;
impl crate::RegisterSpec for BoundarySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boundary::R`](R) reader structure"]
impl crate::Readable for BoundarySpec {}
#[doc = "`write(|w| ..)` method takes [`boundary::W`](W) writer structure"]
impl crate::Writable for BoundarySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOUNDARY to value 0"]
impl crate::Resettable for BoundarySpec {
    const RESET_VALUE: u32 = 0;
}
