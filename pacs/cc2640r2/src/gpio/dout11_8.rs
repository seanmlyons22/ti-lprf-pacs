#[doc = "Register `DOUT11_8` reader"]
pub struct R(crate::R<DOUT11_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT11_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT11_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT11_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT11_8` writer"]
pub struct W(crate::W<DOUT11_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT11_8_SPEC>;
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
impl From<crate::W<DOUT11_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT11_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIO8` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#8, if the corresponding DOE31_0 bitfield is set."]
pub type DIO8_R = crate::BitReader<bool>;
#[doc = "Field `DIO8` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#8, if the corresponding DOE31_0 bitfield is set."]
pub type DIO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT11_8_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT11_8_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO9` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#9, if the corresponding DOE31_0 bitfield is set."]
pub type DIO9_R = crate::BitReader<bool>;
#[doc = "Field `DIO9` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#9, if the corresponding DOE31_0 bitfield is set."]
pub type DIO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT11_8_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT11_8_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO10` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#10, if the corresponding DOE31_0 bitfield is set."]
pub type DIO10_R = crate::BitReader<bool>;
#[doc = "Field `DIO10` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#10, if the corresponding DOE31_0 bitfield is set."]
pub type DIO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT11_8_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT11_8_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO11` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#11, if the corresponding DOE31_0 bitfield is set."]
pub type DIO11_R = crate::BitReader<bool>;
#[doc = "Field `DIO11` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#11, if the corresponding DOE31_0 bitfield is set."]
pub type DIO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT11_8_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT11_8_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#8, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio8(&self) -> DIO8_R {
        DIO8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#9, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio9(&self) -> DIO9_R {
        DIO9_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#10, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio10(&self) -> DIO10_R {
        DIO10_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#11, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio11(&self) -> DIO11_R {
        DIO11_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#8, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio8(&mut self) -> DIO8_W<0> {
        DIO8_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#9, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio9(&mut self) -> DIO9_W<8> {
        DIO9_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#10, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio10(&mut self) -> DIO10_W<16> {
        DIO10_W::new(self)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#11, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio11(&mut self) -> DIO11_W<24> {
        DIO11_W::new(self)
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
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout11_8](index.html) module"]
pub struct DOUT11_8_SPEC;
impl crate::RegisterSpec for DOUT11_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout11_8::R](R) reader structure"]
impl crate::Readable for DOUT11_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout11_8::W](W) writer structure"]
impl crate::Writable for DOUT11_8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUT11_8 to value 0"]
impl crate::Resettable for DOUT11_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
