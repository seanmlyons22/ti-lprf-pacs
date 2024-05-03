#[doc = "Register `FUNCTION1` reader"]
pub type R = crate::R<Function1Spec>;
#[doc = "Register `FUNCTION1` writer"]
pub type W = crate::W<Function1Spec>;
#[doc = "Field `MATCH` reader - 3:0\\]
Controls the type of match generated by this comparator"]
pub type MatchR = crate::FieldReader;
#[doc = "Field `MATCH` writer - 3:0\\]
Controls the type of match generated by this comparator"]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACTION` reader - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
pub type ActionR = crate::FieldReader;
#[doc = "Field `ACTION` writer - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
pub type ActionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED6` reader - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAVSIZE` reader - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
pub type DatavsizeR = crate::FieldReader;
#[doc = "Field `DATAVSIZE` writer - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
pub type DatavsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED12` reader - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED12` writer - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MATCHED` reader - 24:24\\]
Set to 1 when the comparator matches"]
pub type MatchedR = crate::BitReader;
#[doc = "Field `MATCHED` writer - 24:24\\]
Set to 1 when the comparator matches"]
pub type MatchedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ID` reader - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Controls the type of match generated by this comparator"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
    #[inline(always)]
    pub fn datavsize(&self) -> DatavsizeR {
        DatavsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 when the comparator matches"]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Controls the type of match generated by this comparator"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MatchW<Function1Spec> {
        MatchW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ActionW<Function1Spec> {
        ActionW::new(self, 4)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Function1Spec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the object being watched for by Data Value and Data Address comparators"]
    #[inline(always)]
    #[must_use]
    pub fn datavsize(&mut self) -> DatavsizeW<Function1Spec> {
        DatavsizeW::new(self, 10)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<Function1Spec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 24 - 24:24\\]
Set to 1 when the comparator matches"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MatchedW<Function1Spec> {
        MatchedW::new(self, 24)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Function1Spec> {
        Reserved25W::new(self, 25)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Identifies the capabilities for MATCH for comparator *n"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<Function1Spec> {
        IdW::new(self, 27)
    }
}
#[doc = "Controls the operation of watchpoint comparator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Function1Spec;
impl crate::RegisterSpec for Function1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function1::R`](R) reader structure"]
impl crate::Readable for Function1Spec {}
#[doc = "`write(|w| ..)` method takes [`function1::W`](W) writer structure"]
impl crate::Writable for Function1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNCTION1 to value 0"]
impl crate::Resettable for Function1Spec {
    const RESET_VALUE: u32 = 0;
}
