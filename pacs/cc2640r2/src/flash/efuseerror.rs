#[doc = "Register `EFUSEERROR` reader"]
pub type R = crate::R<EfuseerrorSpec>;
#[doc = "Register `EFUSEERROR` writer"]
pub type W = crate::W<EfuseerrorSpec>;
#[doc = "Field `CODE` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type CodeR = crate::FieldReader;
#[doc = "Field `CODE` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DONE` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CodeW<EfuseerrorSpec> {
        CodeW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<EfuseerrorSpec> {
        DoneW::new(self, 5)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseerror::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseerror::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseerrorSpec;
impl crate::RegisterSpec for EfuseerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuseerror::R`](R) reader structure"]
impl crate::Readable for EfuseerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`efuseerror::W`](W) writer structure"]
impl crate::Writable for EfuseerrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSEERROR to value 0"]
impl crate::Resettable for EfuseerrorSpec {
    const RESET_VALUE: u32 = 0;
}
