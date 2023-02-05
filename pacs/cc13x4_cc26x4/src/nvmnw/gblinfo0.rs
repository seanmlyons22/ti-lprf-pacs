#[doc = "Register `GBLINFO0` reader"]
pub struct R(crate::R<GBLINFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GBLINFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GBLINFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GBLINFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GBLINFO0` writer"]
pub struct W(crate::W<GBLINFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GBLINFO0_SPEC>;
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
impl From<crate::W<GBLINFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GBLINFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECTORSIZE` reader - 15:0\\]
Sector size in bytes"]
pub type SECTORSIZE_R = crate::FieldReader<u16, SECTORSIZE_A>;
#[doc = "15:0\\]
Sector size in bytes\n\nValue on reset: 2048"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SECTORSIZE_A {
    #[doc = "2048: Sector size is TWOKB"]
    TWOKB = 2048,
    #[doc = "1024: Sector size is ONEKB"]
    ONEKB = 1024,
}
impl From<SECTORSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: SECTORSIZE_A) -> Self {
        variant as _
    }
}
impl SECTORSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SECTORSIZE_A> {
        match self.bits {
            2048 => Some(SECTORSIZE_A::TWOKB),
            1024 => Some(SECTORSIZE_A::ONEKB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TWOKB`"]
    #[inline(always)]
    pub fn is_twokb(&self) -> bool {
        *self == SECTORSIZE_A::TWOKB
    }
    #[doc = "Checks if the value of the field is `ONEKB`"]
    #[inline(always)]
    pub fn is_onekb(&self) -> bool {
        *self == SECTORSIZE_A::ONEKB
    }
}
#[doc = "Field `SECTORSIZE` writer - 15:0\\]
Sector size in bytes"]
pub type SECTORSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO0_SPEC, u16, SECTORSIZE_A, 16, O>;
impl<'a, const O: u8> SECTORSIZE_W<'a, O> {
    #[doc = "Sector size is TWOKB"]
    #[inline(always)]
    pub fn twokb(self) -> &'a mut W {
        self.variant(SECTORSIZE_A::TWOKB)
    }
    #[doc = "Sector size is ONEKB"]
    #[inline(always)]
    pub fn onekb(self) -> &'a mut W {
        self.variant(SECTORSIZE_A::ONEKB)
    }
}
#[doc = "Field `NUMBANKS` reader - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
pub type NUMBANKS_R = crate::FieldReader<u8, NUMBANKS_A>;
#[doc = "18:16\\]
Number of banks instantiated Minimum:1 Maximum:5\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUMBANKS_A {
    #[doc = "5: Maximum value"]
    MAXIMUM = 5,
    #[doc = "1: Minimum value"]
    MINIMUM = 1,
}
impl From<NUMBANKS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMBANKS_A) -> Self {
        variant as _
    }
}
impl NUMBANKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUMBANKS_A> {
        match self.bits {
            5 => Some(NUMBANKS_A::MAXIMUM),
            1 => Some(NUMBANKS_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == NUMBANKS_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == NUMBANKS_A::MINIMUM
    }
}
#[doc = "Field `NUMBANKS` writer - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
pub type NUMBANKS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO0_SPEC, u8, NUMBANKS_A, 3, O>;
impl<'a, const O: u8> NUMBANKS_W<'a, O> {
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(NUMBANKS_A::MAXIMUM)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(NUMBANKS_A::MINIMUM)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Sector size in bytes"]
    #[inline(always)]
    pub fn sectorsize(&self) -> SECTORSIZE_R {
        SECTORSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
    #[inline(always)]
    pub fn numbanks(&self) -> NUMBANKS_R {
        NUMBANKS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Sector size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sectorsize(&mut self) -> SECTORSIZE_W<0> {
        SECTORSIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
    #[inline(always)]
    #[must_use]
    pub fn numbanks(&mut self) -> NUMBANKS_W<16> {
        NUMBANKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Info 0 Register Read only register detailing information about sector size and number of banks present.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gblinfo0](index.html) module"]
pub struct GBLINFO0_SPEC;
impl crate::RegisterSpec for GBLINFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gblinfo0::R](R) reader structure"]
impl crate::Readable for GBLINFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gblinfo0::W](W) writer structure"]
impl crate::Writable for GBLINFO0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GBLINFO0 to value 0x0002_0800"]
impl crate::Resettable for GBLINFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0800;
}
