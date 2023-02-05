#[doc = "Register `FTCTL` reader"]
pub struct R(crate::R<FTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTCTL` writer"]
pub struct W(crate::W<FTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTCTL_SPEC>;
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
impl From<crate::W<FTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTCTL_SPEC, bool, O>;
#[doc = "Field `TEST_EN` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TEST_EN` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 15:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED2` writer - 15:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTCTL_SPEC, u16, u16, 14, O>;
#[doc = "Field `WDATA_BLK_CLR` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type WDATA_BLK_CLR_R = crate::BitReader<bool>;
#[doc = "Field `WDATA_BLK_CLR` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type WDATA_BLK_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTCTL_SPEC, u16, u16, 15, O>;
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
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wdata_blk_clr(&self) -> WDATA_BLK_CLR_R {
        WDATA_BLK_CLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
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
    pub fn test_en(&mut self) -> TEST_EN_W<1> {
        TEST_EN_W::new(self)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wdata_blk_clr(&mut self) -> WDATA_BLK_CLR_W<16> {
        WDATA_BLK_CLR_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftctl](index.html) module"]
pub struct FTCTL_SPEC;
impl crate::RegisterSpec for FTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftctl::R](R) reader structure"]
impl crate::Readable for FTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftctl::W](W) writer structure"]
impl crate::Writable for FTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTCTL to value 0"]
impl crate::Resettable for FTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
