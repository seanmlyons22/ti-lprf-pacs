#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<Pidr2Spec>;
#[doc = "Register `PIDR2` writer"]
pub type W = crate::W<Pidr2Spec>;
#[doc = "Field `DES_1` reader - 2:0\\]
Bits \\[6:4\\]
of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des1R = crate::FieldReader;
#[doc = "Field `DES_1` writer - 2:0\\]
Bits \\[6:4\\]
of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEDEC` reader - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
pub type JedecR = crate::BitReader;
#[doc = "Field `JEDEC` writer - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
pub type JedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REVISION` reader - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Bits \\[6:4\\]
of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Bits \\[6:4\\]
of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    #[must_use]
    pub fn des_1(&mut self) -> Des1W<Pidr2Spec> {
        Des1W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Always set. Indicates that a JEDEC assigned value is used"]
    #[inline(always)]
    #[must_use]
    pub fn jedec(&mut self) -> JedecW<Pidr2Spec> {
        JedecW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<Pidr2Spec> {
        RevisionW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Pidr2Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr2Spec;
impl crate::RegisterSpec for Pidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for Pidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr2::W`](W) writer structure"]
impl crate::Writable for Pidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for Pidr2Spec {
    const RESET_VALUE: u32 = 0x0b;
}
