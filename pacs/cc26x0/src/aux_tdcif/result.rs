#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESULT` writer"]
pub struct W(crate::W<RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESULT_SPEC>;
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
impl From<crate::W<RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 24:0\\]
TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - 24:0\\]
TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESULT_SPEC, u32, u32, 25, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESULT_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:24 - 24:0\\]
TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:24 - 24:0\\]
TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result Result of last TDC conversion\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [result::W](W) writer structure"]
impl crate::Writable for RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESULT to value 0x02"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
