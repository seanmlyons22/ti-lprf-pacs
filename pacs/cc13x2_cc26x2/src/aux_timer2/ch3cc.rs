#[doc = "Register `CH3CC` reader"]
pub struct R(crate::R<CH3CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3CC` writer"]
pub struct W(crate::W<CH3CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3CC_SPEC>;
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
impl From<crate::W<CH3CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH3EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH3EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH3EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH3EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3CC_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3CC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH3EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH3EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH3EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH3EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 3 Capture Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cc](index.html) module"]
pub struct CH3CC_SPEC;
impl crate::RegisterSpec for CH3CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3cc::R](R) reader structure"]
impl crate::Readable for CH3CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3cc::W](W) writer structure"]
impl crate::Writable for CH3CC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3CC to value 0"]
impl crate::Resettable for CH3CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
