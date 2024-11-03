#[doc = "Register `PREG2` reader"]
pub type R = crate::R<Preg2Spec>;
#[doc = "Register `PREG2` writer"]
pub type W = crate::W<Preg2Spec>;
#[doc = "3:0\\]
PMU REG ATB selection bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuregAtbsel {
    #[doc = "8: Either DCDC/GLDO or Recharge Block output depending on DCDC_RCHG_ATBSEL bit on ATEST0 and Recharge Block output on ATEST1"]
    DcdcAtest0RchgAtest1 = 8,
    #[doc = "4: SOCLDO current on ATEST0"]
    SocldoiA0 = 4,
    #[doc = "1: SOCLDO voltage on ATEST1"]
    SocldovA1 = 1,
    #[doc = "0: No signal connected to ATEST outputs"]
    Nc = 0,
}
impl From<PmuregAtbsel> for u8 {
    #[inline(always)]
    fn from(variant: PmuregAtbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmuregAtbsel {
    type Ux = u8;
}
impl crate::IsEnum for PmuregAtbsel {}
#[doc = "Field `PMUREG_ATBSEL` reader - 3:0\\]
PMU REG ATB selection bits."]
pub type PmuregAtbselR = crate::FieldReader<PmuregAtbsel>;
impl PmuregAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PmuregAtbsel> {
        match self.bits {
            8 => Some(PmuregAtbsel::DcdcAtest0RchgAtest1),
            4 => Some(PmuregAtbsel::SocldoiA0),
            1 => Some(PmuregAtbsel::SocldovA1),
            0 => Some(PmuregAtbsel::Nc),
            _ => None,
        }
    }
    #[doc = "Either DCDC/GLDO or Recharge Block output depending on DCDC_RCHG_ATBSEL bit on ATEST0 and Recharge Block output on ATEST1"]
    #[inline(always)]
    pub fn is_dcdc_atest0_rchg_atest1(&self) -> bool {
        *self == PmuregAtbsel::DcdcAtest0RchgAtest1
    }
    #[doc = "SOCLDO current on ATEST0"]
    #[inline(always)]
    pub fn is_socldoi_a0(&self) -> bool {
        *self == PmuregAtbsel::SocldoiA0
    }
    #[doc = "SOCLDO voltage on ATEST1"]
    #[inline(always)]
    pub fn is_socldov_a1(&self) -> bool {
        *self == PmuregAtbsel::SocldovA1
    }
    #[doc = "No signal connected to ATEST outputs"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == PmuregAtbsel::Nc
    }
}
#[doc = "Field `PMUREG_ATBSEL` writer - 3:0\\]
PMU REG ATB selection bits."]
pub type PmuregAtbselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PmuregAtbsel>;
impl<'a, REG> PmuregAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Either DCDC/GLDO or Recharge Block output depending on DCDC_RCHG_ATBSEL bit on ATEST0 and Recharge Block output on ATEST1"]
    #[inline(always)]
    pub fn dcdc_atest0_rchg_atest1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuregAtbsel::DcdcAtest0RchgAtest1)
    }
    #[doc = "SOCLDO current on ATEST0"]
    #[inline(always)]
    pub fn socldoi_a0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuregAtbsel::SocldoiA0)
    }
    #[doc = "SOCLDO voltage on ATEST1"]
    #[inline(always)]
    pub fn socldov_a1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuregAtbsel::SocldovA1)
    }
    #[doc = "No signal connected to ATEST outputs"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(PmuregAtbsel::Nc)
    }
}
#[doc = "4:4\\]
ATB selection between DCDC/GLDO and Recharge block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcRchgAtbsel {
    #[doc = "1: Enables input trigger level test signal to the recharge block"]
    RchgBlk = 1,
    #[doc = "0: Enables DCDC/GLDO test signals selected by GLDO_ATBSEL"]
    DcdcGldo = 0,
}
impl From<DcdcRchgAtbsel> for bool {
    #[inline(always)]
    fn from(variant: DcdcRchgAtbsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_RCHG_ATBSEL` reader - 4:4\\]
ATB selection between DCDC/GLDO and Recharge block."]
pub type DcdcRchgAtbselR = crate::BitReader<DcdcRchgAtbsel>;
impl DcdcRchgAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcRchgAtbsel {
        match self.bits {
            true => DcdcRchgAtbsel::RchgBlk,
            false => DcdcRchgAtbsel::DcdcGldo,
        }
    }
    #[doc = "Enables input trigger level test signal to the recharge block"]
    #[inline(always)]
    pub fn is_rchg_blk(&self) -> bool {
        *self == DcdcRchgAtbsel::RchgBlk
    }
    #[doc = "Enables DCDC/GLDO test signals selected by GLDO_ATBSEL"]
    #[inline(always)]
    pub fn is_dcdc_gldo(&self) -> bool {
        *self == DcdcRchgAtbsel::DcdcGldo
    }
}
#[doc = "Field `DCDC_RCHG_ATBSEL` writer - 4:4\\]
ATB selection between DCDC/GLDO and Recharge block."]
pub type DcdcRchgAtbselW<'a, REG> = crate::BitWriter<'a, REG, DcdcRchgAtbsel>;
impl<'a, REG> DcdcRchgAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables input trigger level test signal to the recharge block"]
    #[inline(always)]
    pub fn rchg_blk(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcRchgAtbsel::RchgBlk)
    }
    #[doc = "Enables DCDC/GLDO test signals selected by GLDO_ATBSEL"]
    #[inline(always)]
    pub fn dcdc_gldo(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcRchgAtbsel::DcdcGldo)
    }
}
#[doc = "5:5\\]
This bit is used to mask RSTN pin reset during FLTP1 flash test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstnmask {
    #[doc = "1: Reset is masked"]
    Mask = 1,
    #[doc = "0: Reset is unmasked"]
    Unmask = 0,
}
impl From<Rstnmask> for bool {
    #[inline(always)]
    fn from(variant: Rstnmask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTNMASK` reader - 5:5\\]
This bit is used to mask RSTN pin reset during FLTP1 flash test."]
pub type RstnmaskR = crate::BitReader<Rstnmask>;
impl RstnmaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstnmask {
        match self.bits {
            true => Rstnmask::Mask,
            false => Rstnmask::Unmask,
        }
    }
    #[doc = "Reset is masked"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Rstnmask::Mask
    }
    #[doc = "Reset is unmasked"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == Rstnmask::Unmask
    }
}
#[doc = "Field `RSTNMASK` writer - 5:5\\]
This bit is used to mask RSTN pin reset during FLTP1 flash test."]
pub type RstnmaskW<'a, REG> = crate::BitWriter<'a, REG, Rstnmask>;
impl<'a, REG> RstnmaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset is masked"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Rstnmask::Mask)
    }
    #[doc = "Reset is unmasked"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(Rstnmask::Unmask)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
PMU REG ATB selection bits."]
    #[inline(always)]
    pub fn pmureg_atbsel(&self) -> PmuregAtbselR {
        PmuregAtbselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
ATB selection between DCDC/GLDO and Recharge block."]
    #[inline(always)]
    pub fn dcdc_rchg_atbsel(&self) -> DcdcRchgAtbselR {
        DcdcRchgAtbselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is used to mask RSTN pin reset during FLTP1 flash test."]
    #[inline(always)]
    pub fn rstnmask(&self) -> RstnmaskR {
        RstnmaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
PMU REG ATB selection bits."]
    #[inline(always)]
    #[must_use]
    pub fn pmureg_atbsel(&mut self) -> PmuregAtbselW<Preg2Spec> {
        PmuregAtbselW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ATB selection between DCDC/GLDO and Recharge block."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_rchg_atbsel(&mut self) -> DcdcRchgAtbselW<Preg2Spec> {
        DcdcRchgAtbselW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is used to mask RSTN pin reset during FLTP1 flash test."]
    #[inline(always)]
    #[must_use]
    pub fn rstnmask(&mut self) -> RstnmaskW<Preg2Spec> {
        RstnmaskW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Preg2Spec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "PMU REG 2 register. Note: This register is write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preg2Spec;
impl crate::RegisterSpec for Preg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preg2::R`](R) reader structure"]
impl crate::Readable for Preg2Spec {}
#[doc = "`write(|w| ..)` method takes [`preg2::W`](W) writer structure"]
impl crate::Writable for Preg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREG2 to value 0"]
impl crate::Resettable for Preg2Spec {
    const RESET_VALUE: u32 = 0;
}
