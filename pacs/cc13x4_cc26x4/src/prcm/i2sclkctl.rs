#[doc = "Register `I2SCLKCTL` reader"]
pub struct R(crate::R<I2SCLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCLKCTL` writer"]
pub struct W(crate::W<I2SCLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCLKCTL_SPEC>;
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
impl From<crate::W<I2SCLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCLKCTL_SPEC, bool, O>;
#[doc = "Field `WCLK_PHASE` reader - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type WCLK_PHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WCLK_PHASE` writer - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type WCLK_PHASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCLKCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SMPL_ON_POSEDGE` reader - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SMPL_ON_POSEDGE_R = crate::BitReader<bool>;
#[doc = "Field `SMPL_ON_POSEDGE` writer - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SMPL_ON_POSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCLKCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2SCLKCTL_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn wclk_phase(&self) -> WCLK_PHASE_R {
        WCLK_PHASE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn smpl_on_posedge(&self) -> SMPL_ON_POSEDGE_R {
        SMPL_ON_POSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_phase(&mut self) -> WCLK_PHASE_W<1> {
        WCLK_PHASE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_on_posedge(&mut self) -> SMPL_ON_POSEDGE_W<3> {
        SMPL_ON_POSEDGE_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclkctl](index.html) module"]
pub struct I2SCLKCTL_SPEC;
impl crate::RegisterSpec for I2SCLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sclkctl::R](R) reader structure"]
impl crate::Readable for I2SCLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sclkctl::W](W) writer structure"]
impl crate::Writable for I2SCLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCLKCTL to value 0"]
impl crate::Resettable for I2SCLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
