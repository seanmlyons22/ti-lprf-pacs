#[doc = "Register `DEB_AUTH_CFG` reader"]
pub struct R(crate::R<DEB_AUTH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEB_AUTH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEB_AUTH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEB_AUTH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEB_AUTH_CFG` writer"]
pub struct W(crate::W<DEB_AUTH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEB_AUTH_CFG_SPEC>;
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
impl From<crate::W<DEB_AUTH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEB_AUTH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIDENSEL` reader - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
pub type SPIDENSEL_R = crate::BitReader<bool>;
#[doc = "Field `SPIDENSEL` writer - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
pub type SPIDENSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEB_AUTH_CFG_SPEC, bool, O>;
#[doc = "Field `INTSPIDEN` reader - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
pub type INTSPIDEN_R = crate::BitReader<bool>;
#[doc = "Field `INTSPIDEN` writer - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
pub type INTSPIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEB_AUTH_CFG_SPEC, bool, O>;
#[doc = "Field `SPNIDENSEL` reader - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
pub type SPNIDENSEL_R = crate::BitReader<bool>;
#[doc = "Field `SPNIDENSEL` writer - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
pub type SPNIDENSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEB_AUTH_CFG_SPEC, bool, O>;
#[doc = "Field `INTSPNIDEN` reader - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
pub type INTSPNIDEN_R = crate::BitReader<bool>;
#[doc = "Field `INTSPNIDEN` writer - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
pub type INTSPNIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEB_AUTH_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
    #[inline(always)]
    pub fn spidensel(&self) -> SPIDENSEL_R {
        SPIDENSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
    #[inline(always)]
    pub fn intspiden(&self) -> INTSPIDEN_R {
        INTSPIDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
    #[inline(always)]
    pub fn spnidensel(&self) -> SPNIDENSEL_R {
        SPNIDENSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
    #[inline(always)]
    pub fn intspniden(&self) -> INTSPNIDEN_R {
        INTSPNIDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn spidensel(&mut self) -> SPIDENSEL_W<0> {
        SPIDENSEL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn intspiden(&mut self) -> INTSPIDEN_W<1> {
        INTSPIDEN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn spnidensel(&mut self) -> SPNIDENSEL_W<2> {
        SPNIDENSEL_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn intspniden(&mut self) -> INTSPNIDEN_W<3> {
        INTSPNIDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for debug authentication\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deb_auth_cfg](index.html) module"]
pub struct DEB_AUTH_CFG_SPEC;
impl crate::RegisterSpec for DEB_AUTH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deb_auth_cfg::R](R) reader structure"]
impl crate::Readable for DEB_AUTH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deb_auth_cfg::W](W) writer structure"]
impl crate::Writable for DEB_AUTH_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEB_AUTH_CFG to value 0xffff_ffff"]
impl crate::Resettable for DEB_AUTH_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
