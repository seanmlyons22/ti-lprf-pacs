#[doc = "Register `FLASHPUMPP0` reader"]
pub struct R(crate::R<FLASHPUMPP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHPUMPP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHPUMPP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHPUMPP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHPUMPP0` writer"]
pub struct W(crate::W<FLASHPUMPP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHPUMPP0_SPEC>;
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
impl From<crate::W<FLASHPUMPP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHPUMPP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASHPUMPP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `OVR` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASHPUMPP0_SPEC, bool, O>;
#[doc = "Field `LOWLIM` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type LOWLIM_R = crate::BitReader<bool>;
#[doc = "Field `LOWLIM` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type LOWLIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASHPUMPP0_SPEC, bool, O>;
#[doc = "Field `HIGHLIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type HIGHLIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIGHLIM` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type HIGHLIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASHPUMPP0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FALLB` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FALLB_R = crate::BitReader<bool>;
#[doc = "Field `FALLB` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FALLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASHPUMPP0_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHPUMPP0_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lowlim(&self) -> LOWLIM_R {
        LOWLIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn highlim(&self) -> HIGHLIM_R {
        HIGHLIM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fallb(&self) -> FALLB_R {
        FALLB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<0> {
        CFG_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lowlim(&mut self) -> LOWLIM_W<5> {
        LOWLIM_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn highlim(&mut self) -> HIGHLIM_W<6> {
        HIGHLIM_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fallb(&mut self) -> FALLB_W<8> {
        FALLB_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashpumpp0](index.html) module"]
pub struct FLASHPUMPP0_SPEC;
impl crate::RegisterSpec for FLASHPUMPP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashpumpp0::R](R) reader structure"]
impl crate::Readable for FLASHPUMPP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashpumpp0::W](W) writer structure"]
impl crate::Writable for FLASHPUMPP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHPUMPP0 to value 0"]
impl crate::Resettable for FLASHPUMPP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
