#[doc = "Register `REG5_4` reader"]
pub struct R(crate::R<REG5_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG5_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG5_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG5_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG5_4` writer"]
pub struct W(crate::W<REG5_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG5_4_SPEC>;
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
impl From<crate::W<REG5_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG5_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG4` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG4` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG5_4_SPEC, u16, u16, 16, O>;
#[doc = "Field `REG5` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG5` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG5_4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg4(&self) -> REG4_R {
        REG4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg5(&self) -> REG5_R {
        REG5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg4(&mut self) -> REG4_W<0> {
        REG4_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg5(&mut self) -> REG5_W<16> {
        REG5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg5_4](index.html) module"]
pub struct REG5_4_SPEC;
impl crate::RegisterSpec for REG5_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg5_4::R](R) reader structure"]
impl crate::Readable for REG5_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg5_4::W](W) writer structure"]
impl crate::Writable for REG5_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG5_4 to value 0"]
impl crate::Resettable for REG5_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
