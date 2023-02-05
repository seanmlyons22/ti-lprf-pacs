#[doc = "Register `FSM_ADDR` reader"]
pub struct R(crate::R<FSM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ADDR` writer"]
pub struct W(crate::W<FSM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ADDR_SPEC>;
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
impl From<crate::W<FSM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUR_ADDR` reader - 27:0\\]
Internal. Only to be used through TI provided API."]
pub type CUR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CUR_ADDR` writer - 27:0\\]
Internal. Only to be used through TI provided API."]
pub type CUR_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_ADDR_SPEC, u32, u32, 28, O>;
#[doc = "Field `BANK` reader - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANK` writer - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type BANK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_ADDR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED31_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_ADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_addr(&self) -> CUR_ADDR_R {
        CUR_ADDR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cur_addr(&mut self) -> CUR_ADDR_W<0> {
        CUR_ADDR_W::new(self)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bank(&mut self) -> BANK_W<28> {
        BANK_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> RESERVED31_W<31> {
        RESERVED31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_addr](index.html) module"]
pub struct FSM_ADDR_SPEC;
impl crate::RegisterSpec for FSM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_addr::R](R) reader structure"]
impl crate::Readable for FSM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_addr::W](W) writer structure"]
impl crate::Writable for FSM_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_ADDR to value 0"]
impl crate::Resettable for FSM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
