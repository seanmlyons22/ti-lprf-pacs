#[doc = "Register `MEASCFG` reader"]
pub struct R(crate::R<MEASCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEASCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEASCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEASCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEASCFG` writer"]
pub struct W(crate::W<MEASCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEASCFG_SPEC>;
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
impl From<crate::W<MEASCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEASCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PER_R = crate::FieldReader<u8, PER_A>;
#[doc = "1:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PER_A {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _32CYC = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _16CYC = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _8CYC = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    CONT = 0,
}
impl From<PER_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as _
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            3 => PER_A::_32CYC,
            2 => PER_A::_16CYC,
            1 => PER_A::_8CYC,
            0 => PER_A::CONT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32CYC`"]
    #[inline(always)]
    pub fn is_32cyc(&self) -> bool {
        *self == PER_A::_32CYC
    }
    #[doc = "Checks if the value of the field is `_16CYC`"]
    #[inline(always)]
    pub fn is_16cyc(&self) -> bool {
        *self == PER_A::_16CYC
    }
    #[doc = "Checks if the value of the field is `_8CYC`"]
    #[inline(always)]
    pub fn is_8cyc(&self) -> bool {
        *self == PER_A::_8CYC
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == PER_A::CONT
    }
}
#[doc = "Field `PER` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MEASCFG_SPEC, u8, PER_A, 2, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _32cyc(self) -> &'a mut W {
        self.variant(PER_A::_32CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _16cyc(self) -> &'a mut W {
        self.variant(PER_A::_16CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _8cyc(self) -> &'a mut W {
        self.variant(PER_A::_8CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(PER_A::CONT)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEASCFG_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<0> {
        PER_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [meascfg](index.html) module"]
pub struct MEASCFG_SPEC;
impl crate::RegisterSpec for MEASCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [meascfg::R](R) reader structure"]
impl crate::Readable for MEASCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [meascfg::W](W) writer structure"]
impl crate::Writable for MEASCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEASCFG to value 0"]
impl crate::Resettable for MEASCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
