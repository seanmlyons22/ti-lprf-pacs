#[doc = "Register `REG7_6` reader"]
pub struct R(crate::R<REG7_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG7_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG7_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG7_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG7_6` writer"]
pub struct W(crate::W<REG7_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG7_6_SPEC>;
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
impl From<crate::W<REG7_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG7_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG6` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG6` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG7_6_SPEC, u16, u16, 16, O>;
#[doc = "Field `REG7` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG7` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG7_6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg6(&self) -> REG6_R {
        REG6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg7(&self) -> REG7_R {
        REG7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg6(&mut self) -> REG6_W<0> {
        REG6_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg7(&mut self) -> REG7_W<16> {
        REG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg7_6](index.html) module"]
pub struct REG7_6_SPEC;
impl crate::RegisterSpec for REG7_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg7_6::R](R) reader structure"]
impl crate::Readable for REG7_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg7_6::W](W) writer structure"]
impl crate::Writable for REG7_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG7_6 to value 0"]
impl crate::Resettable for REG7_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
