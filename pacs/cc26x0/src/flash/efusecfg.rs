#[doc = "Register `EFUSECFG` reader"]
pub struct R(crate::R<EFUSECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSECFG` writer"]
pub struct W(crate::W<EFUSECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSECFG_SPEC>;
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
impl From<crate::W<EFUSECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GATING` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type GATING_R = crate::BitReader<bool>;
#[doc = "Field `GATING` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSECFG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSECFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SLAVEPOWER` reader - 4:3\\]
Internal. Only to be used through TI provided API."]
pub type SLAVEPOWER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVEPOWER` writer - 4:3\\]
Internal. Only to be used through TI provided API."]
pub type SLAVEPOWER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSECFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSECFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IDLEGATING` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type IDLEGATING_R = crate::BitReader<bool>;
#[doc = "Field `IDLEGATING` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type IDLEGATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSECFG_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSECFG_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn slavepower(&self) -> SLAVEPOWER_R {
        SLAVEPOWER_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idlegating(&self) -> IDLEGATING_R {
        IDLEGATING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<0> {
        GATING_W::new(self)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn slavepower(&mut self) -> SLAVEPOWER_W<3> {
        SLAVEPOWER_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn idlegating(&mut self) -> IDLEGATING_W<8> {
        IDLEGATING_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusecfg](index.html) module"]
pub struct EFUSECFG_SPEC;
impl crate::RegisterSpec for EFUSECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efusecfg::R](R) reader structure"]
impl crate::Readable for EFUSECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efusecfg::W](W) writer structure"]
impl crate::Writable for EFUSECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSECFG to value 0x01"]
impl crate::Resettable for EFUSECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
