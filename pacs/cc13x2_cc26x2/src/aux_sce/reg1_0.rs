#[doc = "Register `REG1_0` reader"]
pub struct R(crate::R<REG1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1_0` writer"]
pub struct W(crate::W<REG1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_0_SPEC>;
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
impl From<crate::W<REG1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type REG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG1_0_SPEC, u16, u16, 16, O>;
#[doc = "Field `REG1` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG1` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type REG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG1_0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg0(&self) -> REG0_R {
        REG0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg1(&self) -> REG1_R {
        REG1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg0(&mut self) -> REG0_W<0> {
        REG0_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg1(&mut self) -> REG1_W<16> {
        REG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1_0](index.html) module"]
pub struct REG1_0_SPEC;
impl crate::RegisterSpec for REG1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1_0::R](R) reader structure"]
impl crate::Readable for REG1_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1_0::W](W) writer structure"]
impl crate::Writable for REG1_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG1_0 to value 0"]
impl crate::Resettable for REG1_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
