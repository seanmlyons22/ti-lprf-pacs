#[doc = "Register `EFUSEADDR` reader"]
pub type R = crate::R<EfuseaddrSpec>;
#[doc = "Register `EFUSEADDR` writer"]
pub type W = crate::W<EfuseaddrSpec>;
#[doc = "Field `ROW` reader - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type RowR = crate::FieldReader<u16>;
#[doc = "Field `ROW` writer - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type RowW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BLOCK` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type BlockR = crate::FieldReader;
#[doc = "Field `BLOCK` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type BlockW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn row(&self) -> RowR {
        RowR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn row(&mut self) -> RowW<EfuseaddrSpec> {
        RowW::new(self, 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BlockW<EfuseaddrSpec> {
        BlockW::new(self, 11)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseaddrSpec;
impl crate::RegisterSpec for EfuseaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuseaddr::R`](R) reader structure"]
impl crate::Readable for EfuseaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`efuseaddr::W`](W) writer structure"]
impl crate::Writable for EfuseaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSEADDR to value 0"]
impl crate::Resettable for EfuseaddrSpec {
    const RESET_VALUE: u32 = 0;
}
