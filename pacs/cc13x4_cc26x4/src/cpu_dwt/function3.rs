#[doc = "Register `FUNCTION3` reader"]
pub struct R(crate::R<FUNCTION3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCTION3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCTION3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCTION3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNCTION3` writer"]
pub struct W(crate::W<FUNCTION3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNCTION3_SPEC>;
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
impl From<crate::W<FUNCTION3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNCTION3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - 3:0\\]
Controls the type of match generated by this comparator"]
pub type MATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MATCH` writer - 3:0\\]
Controls the type of match generated by this comparator"]
pub type MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACTION` reader - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
pub type ACTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTION` writer - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
pub type ACTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED6` reader - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATAVSIZE` reader - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
pub type DATAVSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAVSIZE` writer - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
pub type DATAVSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED12` reader - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED12` writer - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u16, u16, 12, O>;
#[doc = "Field `MATCHED` reader - 24:24\\]
Set to 1 when the comparator matches"]
pub type MATCHED_R = crate::BitReader<bool>;
#[doc = "Field `MATCHED` writer - 24:24\\]
Set to 1 when the comparator matches"]
pub type MATCHED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION3_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ID` reader - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` writer - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Controls the type of match generated by this comparator"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 when the comparator matches"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Controls the type of match generated by this comparator"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<0> {
        MATCH_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<4> {
        ACTION_W::new(self)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
    #[inline(always)]
    #[must_use]
    pub fn datavsize(&mut self) -> DATAVSIZE_W<10> {
        DATAVSIZE_W::new(self)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 when the comparator matches"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MATCHED_W<24> {
        MATCHED_W::new(self)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<27> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the operation of watchpoint comparator 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function3](index.html) module"]
pub struct FUNCTION3_SPEC;
impl crate::RegisterSpec for FUNCTION3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [function3::R](R) reader structure"]
impl crate::Readable for FUNCTION3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [function3::W](W) writer structure"]
impl crate::Writable for FUNCTION3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNCTION3 to value 0"]
impl crate::Resettable for FUNCTION3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
