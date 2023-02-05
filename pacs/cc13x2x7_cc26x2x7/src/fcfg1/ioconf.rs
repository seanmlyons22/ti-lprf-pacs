#[doc = "Register `IOCONF` reader"]
pub struct R(crate::R<IOCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCONF` writer"]
pub struct W(crate::W<IOCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCONF_SPEC>;
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
impl From<crate::W<IOCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_CNT` reader - 6:0\\]
Number of available DIOs."]
pub type GPIO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_CNT` writer - 6:0\\]
Number of available DIOs."]
pub type GPIO_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCONF_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCONF_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Number of available DIOs."]
    #[inline(always)]
    pub fn gpio_cnt(&self) -> GPIO_CNT_R {
        GPIO_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Number of available DIOs."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_cnt(&mut self) -> GPIO_CNT_W<0> {
        GPIO_CNT_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconf](index.html) module"]
pub struct IOCONF_SPEC;
impl crate::RegisterSpec for IOCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioconf::R](R) reader structure"]
impl crate::Readable for IOCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioconf::W](W) writer structure"]
impl crate::Writable for IOCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCONF to value 0xffff_ff00"]
impl crate::Resettable for IOCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff00;
}
