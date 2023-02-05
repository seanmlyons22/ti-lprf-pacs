#[doc = "Register `CONFIG_IF_ADC` reader"]
pub struct R(crate::R<CONFIG_IF_ADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_IF_ADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_IF_ADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_IF_ADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_IF_ADC` writer"]
pub struct W(crate::W<CONFIG_IF_ADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_IF_ADC_SPEC>;
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
impl From<crate::W<CONFIG_IF_ADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_IF_ADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFANALDO_TRIM_OUTPUT` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type IFANALDO_TRIM_OUTPUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFANALDO_TRIM_OUTPUT` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type IFANALDO_TRIM_OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 5, O>;
#[doc = "Field `IFDIGLDO_TRIM_OUTPUT` reader - 9:5\\]
Internal. Only to be used through TI provided API."]
pub type IFDIGLDO_TRIM_OUTPUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFDIGLDO_TRIM_OUTPUT` writer - 9:5\\]
Internal. Only to be used through TI provided API."]
pub type IFDIGLDO_TRIM_OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 5, O>;
#[doc = "Field `INT2ADJ` reader - 13:10\\]
Internal. Only to be used through TI provided API."]
pub type INT2ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT2ADJ` writer - 13:10\\]
Internal. Only to be used through TI provided API."]
pub type INT2ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 4, O>;
#[doc = "Field `AAFCAP` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type AAFCAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AAFCAP` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type AAFCAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 2, O>;
#[doc = "Field `FF1ADJ` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type FF1ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FF1ADJ` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type FF1ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 4, O>;
#[doc = "Field `INT3ADJ` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type INT3ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT3ADJ` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type INT3ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 4, O>;
#[doc = "Field `FF3ADJ` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type FF3ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FF3ADJ` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type FF3ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 4, O>;
#[doc = "Field `FF2ADJ` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type FF2ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FF2ADJ` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type FF2ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_IF_ADC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifanaldo_trim_output(&self) -> IFANALDO_TRIM_OUTPUT_R {
        IFANALDO_TRIM_OUTPUT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifdigldo_trim_output(&self) -> IFDIGLDO_TRIM_OUTPUT_R {
        IFDIGLDO_TRIM_OUTPUT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int2adj(&self) -> INT2ADJ_R {
        INT2ADJ_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aafcap(&self) -> AAFCAP_R {
        AAFCAP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff1adj(&self) -> FF1ADJ_R {
        FF1ADJ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int3adj(&self) -> INT3ADJ_R {
        INT3ADJ_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff3adj(&self) -> FF3ADJ_R {
        FF3ADJ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff2adj(&self) -> FF2ADJ_R {
        FF2ADJ_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ifanaldo_trim_output(&mut self) -> IFANALDO_TRIM_OUTPUT_W<0> {
        IFANALDO_TRIM_OUTPUT_W::new(self)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ifdigldo_trim_output(&mut self) -> IFDIGLDO_TRIM_OUTPUT_W<5> {
        IFDIGLDO_TRIM_OUTPUT_W::new(self)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn int2adj(&mut self) -> INT2ADJ_W<10> {
        INT2ADJ_W::new(self)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aafcap(&mut self) -> AAFCAP_W<14> {
        AAFCAP_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ff1adj(&mut self) -> FF1ADJ_W<16> {
        FF1ADJ_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn int3adj(&mut self) -> INT3ADJ_W<20> {
        INT3ADJ_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ff3adj(&mut self) -> FF3ADJ_W<24> {
        FF3ADJ_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ff2adj(&mut self) -> FF2ADJ_W<28> {
        FF2ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_if_adc](index.html) module"]
pub struct CONFIG_IF_ADC_SPEC;
impl crate::RegisterSpec for CONFIG_IF_ADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_if_adc::R](R) reader structure"]
impl crate::Readable for CONFIG_IF_ADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_if_adc::W](W) writer structure"]
impl crate::Writable for CONFIG_IF_ADC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_IF_ADC to value 0x3460_f400"]
impl crate::Resettable for CONFIG_IF_ADC_SPEC {
    const RESET_VALUE: Self::Ux = 0x3460_f400;
}
