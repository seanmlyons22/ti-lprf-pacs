#[doc = "Register `FPSTROBES` reader"]
pub struct R(crate::R<FPSTROBES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPSTROBES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPSTROBES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPSTROBES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPSTROBES` writer"]
pub struct W(crate::W<FPSTROBES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPSTROBES_SPEC>;
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
impl From<crate::W<FPSTROBES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPSTROBES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `V5PWRDNZ` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type V5PWRDNZ_R = crate::BitReader<bool>;
#[doc = "Field `V5PWRDNZ` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type V5PWRDNZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSTROBES_SPEC, bool, O>;
#[doc = "Field `V3PWRDNZ` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type V3PWRDNZ_R = crate::BitReader<bool>;
#[doc = "Field `V3PWRDNZ` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type V3PWRDNZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSTROBES_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPSTROBES_SPEC, u8, u8, 6, O>;
#[doc = "Field `EXECUTEZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EXECUTEZ_R = crate::BitReader<bool>;
#[doc = "Field `EXECUTEZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EXECUTEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSTROBES_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FPSTROBES_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v5pwrdnz(&self) -> V5PWRDNZ_R {
        V5PWRDNZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v3pwrdnz(&self) -> V3PWRDNZ_R {
        V3PWRDNZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&self) -> EXECUTEZ_R {
        EXECUTEZ_R::new(((self.bits >> 8) & 1) != 0)
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
    pub fn v5pwrdnz(&mut self) -> V5PWRDNZ_W<0> {
        V5PWRDNZ_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn v3pwrdnz(&mut self) -> V3PWRDNZ_W<1> {
        V3PWRDNZ_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn executez(&mut self) -> EXECUTEZ_W<8> {
        EXECUTEZ_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpstrobes](index.html) module"]
pub struct FPSTROBES_SPEC;
impl crate::RegisterSpec for FPSTROBES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpstrobes::R](R) reader structure"]
impl crate::Readable for FPSTROBES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpstrobes::W](W) writer structure"]
impl crate::Writable for FPSTROBES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPSTROBES to value 0x0103"]
impl crate::Resettable for FPSTROBES_SPEC {
    const RESET_VALUE: Self::Ux = 0x0103;
}
