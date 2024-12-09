#[doc = "Register `PWRCTL` reader"]
pub type R = crate::R<PwrctlSpec>;
#[doc = "Register `PWRCTL` writer"]
pub type W = crate::W<PwrctlSpec>;
#[doc = "Field `DCDC_EN` reader - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
pub type DcdcEnR = crate::BitReader;
#[doc = "Field `DCDC_EN` writer - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
pub type DcdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_REG_MODE` reader - 1:1\\]
Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR"]
pub type ExtRegModeR = crate::BitReader;
#[doc = "Field `DCDC_ACTIVE` reader - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
pub type DcdcActiveR = crate::BitReader;
#[doc = "Field `DCDC_ACTIVE` writer - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
pub type DcdcActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_en(&self) -> DcdcEnR {
        DcdcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR"]
    #[inline(always)]
    pub fn ext_reg_mode(&self) -> ExtRegModeR {
        ExtRegModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DcdcActiveR {
        DcdcActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en(&mut self) -> DcdcEnW<PwrctlSpec> {
        DcdcEnW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_active(&mut self) -> DcdcActiveW<PwrctlSpec> {
        DcdcActiveW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<PwrctlSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctlSpec;
impl crate::RegisterSpec for PwrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctl::R`](R) reader structure"]
impl crate::Readable for PwrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctl::W`](W) writer structure"]
impl crate::Writable for PwrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PwrctlSpec {
    const RESET_VALUE: u32 = 0;
}
