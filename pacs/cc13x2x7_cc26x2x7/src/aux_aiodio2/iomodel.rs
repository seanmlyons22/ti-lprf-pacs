#[doc = "Register `IOMODEL` reader"]
pub struct R(crate::R<IOMODEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMODEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMODEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMODEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOMODEL` writer"]
pub struct W(crate::W<IOMODEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMODEL_SPEC>;
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
impl From<crate::W<IOMODEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMODEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO0` reader - 1:0\\]
See IOMODE.IO0."]
pub type IO0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO0` writer - 1:0\\]
See IOMODE.IO0."]
pub type IO0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `IO1` reader - 3:2\\]
See IOMODE.IO1."]
pub type IO1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO1` writer - 3:2\\]
See IOMODE.IO1."]
pub type IO1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `IO2` reader - 5:4\\]
See IOMODE.IO2."]
pub type IO2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO2` writer - 5:4\\]
See IOMODE.IO2."]
pub type IO2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `IO3` reader - 7:6\\]
See IOMODE.IO3."]
pub type IO3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO3` writer - 7:6\\]
See IOMODE.IO3."]
pub type IO3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMODEL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
See IOMODE.IO0."]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO1."]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO2."]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO3."]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 6) & 3) as u8)
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
See IOMODE.IO0."]
    #[inline(always)]
    #[must_use]
    pub fn io0(&mut self) -> IO0_W<0> {
        IO0_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO1."]
    #[inline(always)]
    #[must_use]
    pub fn io1(&mut self) -> IO1_W<2> {
        IO1_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO2."]
    #[inline(always)]
    #[must_use]
    pub fn io2(&mut self) -> IO2_W<4> {
        IO2_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO3."]
    #[inline(always)]
    #[must_use]
    pub fn io3(&mut self) -> IO3_W<6> {
        IO3_W::new(self)
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
#[doc = "Input Output Mode Low This is an alias register for IOMODE.IO0 thru IOMODE.IO3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomodel](index.html) module"]
pub struct IOMODEL_SPEC;
impl crate::RegisterSpec for IOMODEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iomodel::R](R) reader structure"]
impl crate::Readable for IOMODEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iomodel::W](W) writer structure"]
impl crate::Writable for IOMODEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOMODEL to value 0"]
impl crate::Resettable for IOMODEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
