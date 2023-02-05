#[doc = "Register `REG3_2` reader"]
pub struct R(crate::R<REG3_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG3_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG3_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG3_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG3_2` writer"]
pub struct W(crate::W<REG3_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG3_2_SPEC>;
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
impl From<crate::W<REG3_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG3_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG2` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG2` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG3_2_SPEC, u16, u16, 16, O>;
#[doc = "Field `REG3` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG3` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG3_2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg2(&self) -> REG2_R {
        REG2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg3(&self) -> REG3_R {
        REG3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg2(&mut self) -> REG2_W<0> {
        REG2_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg3(&mut self) -> REG3_W<16> {
        REG3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3_2](index.html) module"]
pub struct REG3_2_SPEC;
impl crate::RegisterSpec for REG3_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg3_2::R](R) reader structure"]
impl crate::Readable for REG3_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg3_2::W](W) writer structure"]
impl crate::Writable for REG3_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG3_2 to value 0"]
impl crate::Resettable for REG3_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
