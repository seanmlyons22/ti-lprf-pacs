#[doc = "Register `FSPRD` reader"]
pub struct R(crate::R<FSPRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSPRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSPRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSPRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSPRD` writer"]
pub struct W(crate::W<FSPRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSPRD_SPEC>;
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
impl From<crate::W<FSPRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSPRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RM0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RM0_R = crate::BitReader<bool>;
#[doc = "Field `RM0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSPRD_SPEC, bool, O>;
#[doc = "Field `RM1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type RM1_R = crate::BitReader<bool>;
#[doc = "Field `RM1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type RM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSPRD_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSPRD_SPEC, u8, u8, 6, O>;
#[doc = "Field `RMBSEM` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RMBSEM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMBSEM` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RMBSEM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSPRD_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIS_PREEMPT` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type DIS_PREEMPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIS_PREEMPT` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type DIS_PREEMPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSPRD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm0(&self) -> RM0_R {
        RM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm1(&self) -> RM1_R {
        RM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rmbsem(&self) -> RMBSEM_R {
        RMBSEM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_preempt(&self) -> DIS_PREEMPT_R {
        DIS_PREEMPT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rm0(&mut self) -> RM0_W<0> {
        RM0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rm1(&mut self) -> RM1_W<1> {
        RM1_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rmbsem(&mut self) -> RMBSEM_W<8> {
        RMBSEM_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_preempt(&mut self) -> DIS_PREEMPT_W<16> {
        DIS_PREEMPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsprd](index.html) module"]
pub struct FSPRD_SPEC;
impl crate::RegisterSpec for FSPRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsprd::R](R) reader structure"]
impl crate::Readable for FSPRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsprd::W](W) writer structure"]
impl crate::Writable for FSPRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSPRD to value 0"]
impl crate::Resettable for FSPRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
