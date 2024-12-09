#[doc = "Register `EFUSE` reader"]
pub type R = crate::R<EfuseSpec>;
#[doc = "Register `EFUSE` writer"]
pub type W = crate::W<EfuseSpec>;
#[doc = "Field `DUMPWORD` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type DumpwordR = crate::FieldReader<u16>;
#[doc = "Field `DUMPWORD` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type DumpwordW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader;
#[doc = "Field `INSTRUCTION` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type InstructionR = crate::FieldReader;
#[doc = "Field `INSTRUCTION` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type InstructionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type Reserved29R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dumpword(&self) -> DumpwordR {
        DumpwordR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn instruction(&self) -> InstructionR {
        InstructionR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dumpword(&mut self) -> DumpwordW<EfuseSpec> {
        DumpwordW::new(self, 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> InstructionW<EfuseSpec> {
        InstructionW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseSpec;
impl crate::RegisterSpec for EfuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse::R`](R) reader structure"]
impl crate::Readable for EfuseSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse::W`](W) writer structure"]
impl crate::Writable for EfuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE to value 0"]
impl crate::Resettable for EfuseSpec {
    const RESET_VALUE: u32 = 0;
}
