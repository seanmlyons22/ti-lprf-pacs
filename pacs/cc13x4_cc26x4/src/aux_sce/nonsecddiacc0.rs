#[doc = "Register `NONSECDDIACC0` reader"]
pub struct R(crate::R<NONSECDDIACC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSECDDIACC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSECDDIACC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSECDDIACC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NONSECDDIACC0` writer"]
pub struct W(crate::W<NONSECDDIACC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NONSECDDIACC0_SPEC>;
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
impl From<crate::W<NONSECDDIACC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NONSECDDIACC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_MASK` reader - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
pub type WR_MASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WR_MASK` writer - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
pub type WR_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NONSECDDIACC0_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADDR` reader - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NONSECDDIACC0_SPEC, u8, u8, 6, O>;
#[doc = "Field `RD_EN` reader - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
pub type RD_EN_R = crate::BitReader<bool>;
#[doc = "Field `RD_EN` writer - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
pub type RD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NONSECDDIACC0_SPEC, bool, O>;
#[doc = "Field `RESERVED23` reader - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED23` writer - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED23_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NONSECDDIACC0_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
    #[inline(always)]
    pub fn wr_mask(&self) -> WR_MASK_R {
        WR_MASK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Mask AUX_SCE is allowed to update bits in half-word given by ADDR according to this bit mask."]
    #[inline(always)]
    #[must_use]
    pub fn wr_mask(&mut self) -> WR_MASK_W<0> {
        WR_MASK_W::new(self)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Address AUX_SCE is allowed to update this DDI half-word using SET or CLR access."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<16> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Read Enable 0: AUX_SCE is not allowed to read DDI half-word given by ADDR. 1: AUX_SCE is allowed to read DDI half-word given by ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RD_EN_W<22> {
        RD_EN_W::new(self)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> RESERVED23_W<23> {
        RESERVED23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Secure DDI Access 0 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonsecddiacc0](index.html) module"]
pub struct NONSECDDIACC0_SPEC;
impl crate::RegisterSpec for NONSECDDIACC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonsecddiacc0::R](R) reader structure"]
impl crate::Readable for NONSECDDIACC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nonsecddiacc0::W](W) writer structure"]
impl crate::Writable for NONSECDDIACC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NONSECDDIACC0 to value 0"]
impl crate::Resettable for NONSECDDIACC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
