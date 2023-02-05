#[doc = "Register `TWOBIT` reader"]
pub struct R(crate::R<TWOBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWOBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWOBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWOBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWOBIT` writer"]
pub struct W(crate::W<TWOBIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWOBIT_SPEC>;
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
impl From<crate::W<TWOBIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWOBIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FROM0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type FROM0_R = crate::BitReader<bool>;
#[doc = "Field `FROM0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type FROM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWOBIT_SPEC, bool, O>;
#[doc = "Field `FROMN` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type FROMN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FROMN` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type FROMN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TWOBIT_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn from0(&self) -> FROM0_R {
        FROM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fromn(&self) -> FROMN_R {
        FROMN_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn from0(&mut self) -> FROM0_W<0> {
        FROM0_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fromn(&mut self) -> FROMN_W<1> {
        FROMN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twobit](index.html) module"]
pub struct TWOBIT_SPEC;
impl crate::RegisterSpec for TWOBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twobit::R](R) reader structure"]
impl crate::Readable for TWOBIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twobit::W](W) writer structure"]
impl crate::Writable for TWOBIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWOBIT to value 0"]
impl crate::Resettable for TWOBIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
