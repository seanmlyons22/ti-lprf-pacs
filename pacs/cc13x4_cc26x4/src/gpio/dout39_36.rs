#[doc = "Register `DOUT39_36` reader"]
pub struct R(crate::R<DOUT39_36_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT39_36_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT39_36_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT39_36_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT39_36` writer"]
pub struct W(crate::W<DOUT39_36_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT39_36_SPEC>;
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
impl From<crate::W<DOUT39_36_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT39_36_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIO36` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
pub type DIO36_R = crate::BitReader<bool>;
#[doc = "Field `DIO36` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
pub type DIO36_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT39_36_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT39_36_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO37` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
pub type DIO37_R = crate::BitReader<bool>;
#[doc = "Field `DIO37` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
pub type DIO37_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT39_36_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT39_36_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO38` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
pub type DIO38_R = crate::BitReader<bool>;
#[doc = "Field `DIO38` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
pub type DIO38_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT39_36_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT39_36_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIO39` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
pub type DIO39_R = crate::BitReader<bool>;
#[doc = "Field `DIO39` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
pub type DIO39_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUT39_36_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT39_36_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio36(&self) -> DIO36_R {
        DIO36_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio37(&self) -> DIO37_R {
        DIO37_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio38(&self) -> DIO38_R {
        DIO38_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio39(&self) -> DIO39_R {
        DIO39_R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio36(&mut self) -> DIO36_W<0> {
        DIO36_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio37(&mut self) -> DIO37_W<8> {
        DIO37_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio38(&mut self) -> DIO38_W<16> {
        DIO38_W::new(self)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio39(&mut self) -> DIO39_W<24> {
        DIO39_W::new(self)
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
#[doc = "Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout39_36](index.html) module"]
pub struct DOUT39_36_SPEC;
impl crate::RegisterSpec for DOUT39_36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout39_36::R](R) reader structure"]
impl crate::Readable for DOUT39_36_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout39_36::W](W) writer structure"]
impl crate::Writable for DOUT39_36_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUT39_36 to value 0"]
impl crate::Resettable for DOUT39_36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
