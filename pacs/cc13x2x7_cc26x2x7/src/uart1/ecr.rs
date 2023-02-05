#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FE` reader - 0:0\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `FE` writer - 0:0\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `PE` reader - 1:1\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - 1:1\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `BE` reader - 2:2\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - 2:2\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `OE` reader - 3:3\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `OE` writer - 3:3\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<0> {
        FE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<2> {
        BE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<3> {
        OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
