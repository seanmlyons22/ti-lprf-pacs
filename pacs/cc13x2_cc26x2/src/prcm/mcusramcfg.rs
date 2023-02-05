#[doc = "Register `MCUSRAMCFG` reader"]
pub struct R(crate::R<MCUSRAMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUSRAMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUSRAMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUSRAMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUSRAMCFG` writer"]
pub struct W(crate::W<MCUSRAMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUSRAMCFG_SPEC>;
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
impl From<crate::W<MCUSRAMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUSRAMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCH_L` reader - 0:0\\]
0: No bitline precharge in first half of cycle 1: Bitline precharge in first half of cycle when in Burst Mode, BM = 1"]
pub type PCH_L_R = crate::BitReader<bool>;
#[doc = "Field `PCH_L` writer - 0:0\\]
0: No bitline precharge in first half of cycle 1: Bitline precharge in first half of cycle when in Burst Mode, BM = 1"]
pub type PCH_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUSRAMCFG_SPEC, bool, O>;
#[doc = "Field `PCH_F` reader - 1:1\\]
0: No bitline precharge in second half of cycle 1: Bitline precharge in second half of cycle when in Burst Mode, BM = 1"]
pub type PCH_F_R = crate::BitReader<bool>;
#[doc = "Field `PCH_F` writer - 1:1\\]
0: No bitline precharge in second half of cycle 1: Bitline precharge in second half of cycle when in Burst Mode, BM = 1"]
pub type PCH_F_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUSRAMCFG_SPEC, bool, O>;
#[doc = "Field `BM` reader - 2:2\\]
Burst Mode Enable 0: Burst Mode Disable. Memory works in standard mode. 1: Burst Mode Enable When in Burst Mode bitline precharge and wordline firing depends on PCH_F and PCH_L. Burst Mode results in reduction in active power."]
pub type BM_R = crate::BitReader<bool>;
#[doc = "Field `BM` writer - 2:2\\]
Burst Mode Enable 0: Burst Mode Disable. Memory works in standard mode. 1: Burst Mode Enable When in Burst Mode bitline precharge and wordline firing depends on PCH_F and PCH_L. Burst Mode results in reduction in active power."]
pub type BM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUSRAMCFG_SPEC, bool, O>;
#[doc = "Field `PGS` reader - 3:3\\]
0: Select LSB half of word during Page Mode, PAGE = 1 1: Select MSB half of word during Page Mode, PAGE = 1"]
pub type PGS_R = crate::BitReader<bool>;
#[doc = "Field `PGS` writer - 3:3\\]
0: Select LSB half of word during Page Mode, PAGE = 1 1: Select MSB half of word during Page Mode, PAGE = 1"]
pub type PGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUSRAMCFG_SPEC, bool, O>;
#[doc = "Field `PAGE` reader - 4:4\\]
Page Mode select 0: Page Mode disabled. Memory works in standard mode 1: Page Mode enabled. Only one half of butterfly array selected. Page Mode will select either LSB half or MSB half of the word based on PGS setting. This mode can be used for additional power saving"]
pub type PAGE_R = crate::BitReader<bool>;
#[doc = "Field `PAGE` writer - 4:4\\]
Page Mode select 0: Page Mode disabled. Memory works in standard mode 1: Page Mode enabled. Only one half of butterfly array selected. Page Mode will select either LSB half or MSB half of the word based on PGS setting. This mode can be used for additional power saving"]
pub type PAGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUSRAMCFG_SPEC, bool, O>;
#[doc = "Field `BM_OFF` reader - 5:5\\]
Burst Mode disable 0: Burst Mode enabled. 1: Burst Mode off."]
pub type BM_OFF_R = crate::BitReader<bool>;
#[doc = "Field `BM_OFF` writer - 5:5\\]
Burst Mode disable 0: Burst Mode enabled. 1: Burst Mode off."]
pub type BM_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCUSRAMCFG_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCUSRAMCFG_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No bitline precharge in first half of cycle 1: Bitline precharge in first half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    pub fn pch_l(&self) -> PCH_L_R {
        PCH_L_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No bitline precharge in second half of cycle 1: Bitline precharge in second half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    pub fn pch_f(&self) -> PCH_F_R {
        PCH_F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Burst Mode Enable 0: Burst Mode Disable. Memory works in standard mode. 1: Burst Mode Enable When in Burst Mode bitline precharge and wordline firing depends on PCH_F and PCH_L. Burst Mode results in reduction in active power."]
    #[inline(always)]
    pub fn bm(&self) -> BM_R {
        BM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Select LSB half of word during Page Mode, PAGE = 1 1: Select MSB half of word during Page Mode, PAGE = 1"]
    #[inline(always)]
    pub fn pgs(&self) -> PGS_R {
        PGS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Page Mode select 0: Page Mode disabled. Memory works in standard mode 1: Page Mode enabled. Only one half of butterfly array selected. Page Mode will select either LSB half or MSB half of the word based on PGS setting. This mode can be used for additional power saving"]
    #[inline(always)]
    pub fn page(&self) -> PAGE_R {
        PAGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Burst Mode disable 0: Burst Mode enabled. 1: Burst Mode off."]
    #[inline(always)]
    pub fn bm_off(&self) -> BM_OFF_R {
        BM_OFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No bitline precharge in first half of cycle 1: Bitline precharge in first half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn pch_l(&mut self) -> PCH_L_W<0> {
        PCH_L_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No bitline precharge in second half of cycle 1: Bitline precharge in second half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    #[must_use]
    pub fn pch_f(&mut self) -> PCH_F_W<1> {
        PCH_F_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Burst Mode Enable 0: Burst Mode Disable. Memory works in standard mode. 1: Burst Mode Enable When in Burst Mode bitline precharge and wordline firing depends on PCH_F and PCH_L. Burst Mode results in reduction in active power."]
    #[inline(always)]
    #[must_use]
    pub fn bm(&mut self) -> BM_W<2> {
        BM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Select LSB half of word during Page Mode, PAGE = 1 1: Select MSB half of word during Page Mode, PAGE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn pgs(&mut self) -> PGS_W<3> {
        PGS_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Page Mode select 0: Page Mode disabled. Memory works in standard mode 1: Page Mode enabled. Only one half of butterfly array selected. Page Mode will select either LSB half or MSB half of the word based on PGS setting. This mode can be used for additional power saving"]
    #[inline(always)]
    #[must_use]
    pub fn page(&mut self) -> PAGE_W<4> {
        PAGE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Burst Mode disable 0: Burst Mode enabled. 1: Burst Mode off."]
    #[inline(always)]
    #[must_use]
    pub fn bm_off(&mut self) -> BM_OFF_W<5> {
        BM_OFF_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU SRAM configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcusramcfg](index.html) module"]
pub struct MCUSRAMCFG_SPEC;
impl crate::RegisterSpec for MCUSRAMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcusramcfg::R](R) reader structure"]
impl crate::Readable for MCUSRAMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcusramcfg::W](W) writer structure"]
impl crate::Writable for MCUSRAMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUSRAMCFG to value 0x20"]
impl crate::Resettable for MCUSRAMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
