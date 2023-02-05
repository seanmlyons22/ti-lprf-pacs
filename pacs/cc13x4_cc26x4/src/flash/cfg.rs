#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BP_TRIMCFG_EN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BP_TRIMCFG_EN_R = crate::BitReader<bool>;
#[doc = "Field `BP_TRIMCFG_EN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BP_TRIMCFG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIS_READACCESS` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DIS_READACCESS_R = crate::BitReader<bool>;
#[doc = "Field `DIS_READACCESS` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DIS_READACCESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DIS_EFUSECLK` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DIS_EFUSECLK_R = crate::BitReader<bool>;
#[doc = "Field `DIS_EFUSECLK` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DIS_EFUSECLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENGR_TRIM_STICKY_EN` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type ENGR_TRIM_STICKY_EN_R = crate::BitReader<bool>;
#[doc = "Field `ENGR_TRIM_STICKY_EN` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type ENGR_TRIM_STICKY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FCFG_STICKY_EN` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type FCFG_STICKY_EN_R = crate::BitReader<bool>;
#[doc = "Field `FCFG_STICKY_EN` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type FCFG_STICKY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CCFG_STICKY_EN` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type CCFG_STICKY_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCFG_STICKY_EN` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type CCFG_STICKY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `MAIN_STICKY_EN` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type MAIN_STICKY_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAIN_STICKY_EN` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type MAIN_STICKY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 29:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 29:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u32, u32, 18, O>;
#[doc = "Field `DIS_FWTEST` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type DIS_FWTEST_R = crate::BitReader<bool>;
#[doc = "Field `DIS_FWTEST` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type DIS_FWTEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED31_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_trimcfg_en(&self) -> BP_TRIMCFG_EN_R {
        BP_TRIMCFG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_readaccess(&self) -> DIS_READACCESS_R {
        DIS_READACCESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_efuseclk(&self) -> DIS_EFUSECLK_R {
        DIS_EFUSECLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn engr_trim_sticky_en(&self) -> ENGR_TRIM_STICKY_EN_R {
        ENGR_TRIM_STICKY_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fcfg_sticky_en(&self) -> FCFG_STICKY_EN_R {
        FCFG_STICKY_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ccfg_sticky_en(&self) -> CCFG_STICKY_EN_R {
        CCFG_STICKY_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_sticky_en(&self) -> MAIN_STICKY_EN_R {
        MAIN_STICKY_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:29 - 29:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x0003_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_fwtest(&self) -> DIS_FWTEST_R {
        DIS_FWTEST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bp_trimcfg_en(&mut self) -> BP_TRIMCFG_EN_W<0> {
        BP_TRIMCFG_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_readaccess(&mut self) -> DIS_READACCESS_W<4> {
        DIS_READACCESS_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_efuseclk(&mut self) -> DIS_EFUSECLK_W<5> {
        DIS_EFUSECLK_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn engr_trim_sticky_en(&mut self) -> ENGR_TRIM_STICKY_EN_W<8> {
        ENGR_TRIM_STICKY_EN_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fcfg_sticky_en(&mut self) -> FCFG_STICKY_EN_W<9> {
        FCFG_STICKY_EN_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ccfg_sticky_en(&mut self) -> CCFG_STICKY_EN_W<10> {
        CCFG_STICKY_EN_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn main_sticky_en(&mut self) -> MAIN_STICKY_EN_W<11> {
        MAIN_STICKY_EN_W::new(self)
    }
    #[doc = "Bits 12:29 - 29:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_fwtest(&mut self) -> DIS_FWTEST_W<30> {
        DIS_FWTEST_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> RESERVED31_W<31> {
        RESERVED31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
