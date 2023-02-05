#[doc = "Register `SRAM_CFG` reader"]
pub struct R(crate::R<SRAM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CFG` writer"]
pub struct W(crate::W<SRAM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SRAM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARITY_DIS` reader - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
pub type PARITY_DIS_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_DIS` writer - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
pub type PARITY_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG_SPEC, bool, O>;
#[doc = "Field `MEM_SEL` reader - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
pub type MEM_SEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEM_SEL` writer - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
pub type MEM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAM_CFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
    #[inline(always)]
    pub fn parity_dis(&self) -> PARITY_DIS_R {
        PARITY_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
    #[inline(always)]
    pub fn mem_sel(&self) -> MEM_SEL_R {
        MEM_SEL_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn parity_dis(&mut self) -> PARITY_DIS_W<0> {
        PARITY_DIS_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn mem_sel(&mut self) -> MEM_SEL_W<8> {
        MEM_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for MCU SRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cfg](index.html) module"]
pub struct SRAM_CFG_SPEC;
impl crate::RegisterSpec for SRAM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cfg::R](R) reader structure"]
impl crate::Readable for SRAM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cfg::W](W) writer structure"]
impl crate::Writable for SRAM_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_CFG to value 0xffff_ffff"]
impl crate::Resettable for SRAM_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
