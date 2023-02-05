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
#[doc = "Field `RESERVED0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DIS_STANDBY` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_R = crate::BitReader<bool>;
#[doc = "Field `DIS_STANDBY` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `ENABLE_SWINTF` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type ENABLE_SWINTF_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_SWINTF` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type ENABLE_SWINTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
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
#[doc = "Field `STANDBY_PW_SEL` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STANDBY_PW_SEL` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `STANDBY_MODE_SEL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `STANDBY_MODE_SEL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `FORCE_STANDBY` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_STANDBY_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_STANDBY` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type FORCE_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED21` reader - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED21_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED21` writer - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby(&self) -> DIS_STANDBY_R {
        DIS_STANDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn enable_swintf(&self) -> ENABLE_SWINTF_R {
        ENABLE_SWINTF_R::new(((self.bits >> 3) & 1) != 0)
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
    pub fn standby_pw_sel(&self) -> STANDBY_PW_SEL_R {
        STANDBY_PW_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel(&self) -> STANDBY_MODE_SEL_R {
        STANDBY_MODE_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_standby(&self) -> FORCE_STANDBY_R {
        FORCE_STANDBY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby(&mut self) -> DIS_STANDBY_W<1> {
        DIS_STANDBY_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn enable_swintf(&mut self) -> ENABLE_SWINTF_W<3> {
        ENABLE_SWINTF_W::new(self)
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
    pub fn standby_pw_sel(&mut self) -> STANDBY_PW_SEL_W<6> {
        STANDBY_PW_SEL_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel(&mut self) -> STANDBY_MODE_SEL_W<8> {
        STANDBY_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_standby(&mut self) -> FORCE_STANDBY_W<9> {
        FORCE_STANDBY_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> RESERVED21_W<10> {
        RESERVED21_W::new(self)
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
