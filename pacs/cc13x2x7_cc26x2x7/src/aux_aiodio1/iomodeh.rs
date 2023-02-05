#[doc = "Register `IOMODEH` reader"]
pub struct R(crate::R<IOMODEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMODEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMODEH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMODEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOMODEH` writer"]
pub struct W(crate::W<IOMODEH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMODEH_SPEC>;
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
impl From<crate::W<IOMODEH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMODEH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO4` reader - 1:0\\]
See IOMODE.IO4."]
pub type IO4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO4` writer - 1:0\\]
See IOMODE.IO4."]
pub type IO4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEH_SPEC, u8, u8, 2, O>;
#[doc = "Field `IO5` reader - 3:2\\]
See IOMODE.IO5."]
pub type IO5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO5` writer - 3:2\\]
See IOMODE.IO5."]
pub type IO5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEH_SPEC, u8, u8, 2, O>;
#[doc = "Field `IO6` reader - 5:4\\]
See IOMODE.IO6."]
pub type IO6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO6` writer - 5:4\\]
See IOMODE.IO6."]
pub type IO6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEH_SPEC, u8, u8, 2, O>;
#[doc = "Field `IO7` reader - 7:6\\]
See IOMODE.IO7."]
pub type IO7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO7` writer - 7:6\\]
See IOMODE.IO7."]
pub type IO7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEH_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEH_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
See IOMODE.IO4."]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO5."]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO6."]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO7."]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
See IOMODE.IO4."]
    #[inline(always)]
    #[must_use]
    pub fn io4(&mut self) -> IO4_W<0> {
        IO4_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO5."]
    #[inline(always)]
    #[must_use]
    pub fn io5(&mut self) -> IO5_W<2> {
        IO5_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO6."]
    #[inline(always)]
    #[must_use]
    pub fn io6(&mut self) -> IO6_W<4> {
        IO6_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO7."]
    #[inline(always)]
    #[must_use]
    pub fn io7(&mut self) -> IO7_W<6> {
        IO7_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Output Mode High This is an alias register for IOMODE.IO4 thru IOMODE.IO7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomodeh](index.html) module"]
pub struct IOMODEH_SPEC;
impl crate::RegisterSpec for IOMODEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iomodeh::R](R) reader structure"]
impl crate::Readable for IOMODEH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iomodeh::W](W) writer structure"]
impl crate::Writable for IOMODEH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOMODEH to value 0"]
impl crate::Resettable for IOMODEH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
