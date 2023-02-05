#[doc = "Register `DOUT31_28` reader"]
pub struct R(crate::R<DOUT31_28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT31_28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT31_28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT31_28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT31_28` writer"]
pub struct W(crate::W<DOUT31_28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT31_28_SPEC>;
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
impl From<crate::W<DOUT31_28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT31_28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIO28` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE31_0 bitfield is set."]
pub type DIO28_R = crate::BitReader<bool>;
#[doc = "Field `DIO28` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE31_0 bitfield is set."]
pub type DIO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT31_28_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT31_28_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO29` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE31_0 bitfield is set."]
pub type DIO29_R = crate::BitReader<bool>;
#[doc = "Field `DIO29` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE31_0 bitfield is set."]
pub type DIO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT31_28_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT31_28_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO30` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE31_0 bitfield is set."]
pub type DIO30_R = crate::BitReader<bool>;
#[doc = "Field `DIO30` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE31_0 bitfield is set."]
pub type DIO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT31_28_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT31_28_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO31` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE31_0 bitfield is set."]
pub type DIO31_R = crate::BitReader<bool>;
#[doc = "Field `DIO31` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE31_0 bitfield is set."]
pub type DIO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT31_28_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT31_28_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio28(&self) -> DIO28_R {
        DIO28_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio29(&self) -> DIO29_R {
        DIO29_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio30(&self) -> DIO30_R {
        DIO30_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio31(&self) -> DIO31_R {
        DIO31_R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#28, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio28(&mut self) -> DIO28_W<0> {
        DIO28_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#29, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio29(&mut self) -> DIO29_W<8> {
        DIO29_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#30, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio30(&mut self) -> DIO30_W<16> {
        DIO30_W::new(self)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#31, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio31(&mut self) -> DIO31_W<24> {
        DIO31_W::new(self)
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
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout31_28](index.html) module"]
pub struct DOUT31_28_SPEC;
impl crate::RegisterSpec for DOUT31_28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout31_28::R](R) reader structure"]
impl crate::Readable for DOUT31_28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout31_28::W](W) writer structure"]
impl crate::Writable for DOUT31_28_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUT31_28 to value 0"]
impl crate::Resettable for DOUT31_28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
