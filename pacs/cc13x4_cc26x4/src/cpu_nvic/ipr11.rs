#[doc = "Register `IPR11` reader"]
pub struct R(crate::R<IPR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPR11` writer"]
pub struct W(crate::W<IPR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPR11_SPEC>;
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
impl From<crate::W<IPR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_N0` reader - 7:0\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+0, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_N0` writer - 7:0\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+0, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_N1` reader - 15:8\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+1, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_N1` writer - 15:8\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+1, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_N2` reader - 23:16\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+2, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_N2` writer - 23:16\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+2, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_N3` reader - 31:24\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+3, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_N3` writer - 31:24\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+3, or is RES0 if the PE does not implement this interrupt"]
pub type PRI_N3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPR11_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+0, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n0(&self) -> PRI_N0_R {
        PRI_N0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+1, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n1(&self) -> PRI_N1_R {
        PRI_N1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+2, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n2(&self) -> PRI_N2_R {
        PRI_N2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+3, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n3(&self) -> PRI_N3_R {
        PRI_N3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+0, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n0(&mut self) -> PRI_N0_W<0> {
        PRI_N0_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+1, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n1(&mut self) -> PRI_N1_W<8> {
        PRI_N1_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+2, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n2(&mut self) -> PRI_N2_W<16> {
        PRI_N2_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
For register NVIC_IPR*11, the priority of interrupt number 4*11+3, or is RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n3(&mut self) -> PRI_N3_W<24> {
        PRI_N3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sets or reads interrupt priorities\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipr11](index.html) module"]
pub struct IPR11_SPEC;
impl crate::RegisterSpec for IPR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipr11::R](R) reader structure"]
impl crate::Readable for IPR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipr11::W](W) writer structure"]
impl crate::Writable for IPR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPR11 to value 0"]
impl crate::Resettable for IPR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
