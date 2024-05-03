#[doc = "Register `SYSCODE_START` reader"]
pub type R = crate::R<SyscodeStartSpec>;
#[doc = "Register `SYSCODE_START` writer"]
pub type W = crate::W<SyscodeStartSpec>;
#[doc = "Field `SYSCODE_START` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SyscodeStartR = crate::FieldReader;
#[doc = "Field `SYSCODE_START` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SyscodeStartW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED5` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn syscode_start(&self) -> SyscodeStartR {
        SyscodeStartR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn syscode_start(&mut self) -> SyscodeStartW<SyscodeStartSpec> {
        SyscodeStartW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<SyscodeStartSpec> {
        Reserved5W::new(self, 6)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscode_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscode_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscodeStartSpec;
impl crate::RegisterSpec for SyscodeStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscode_start::R`](R) reader structure"]
impl crate::Readable for SyscodeStartSpec {}
#[doc = "`write(|w| ..)` method takes [`syscode_start::W`](W) writer structure"]
impl crate::Writable for SyscodeStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCODE_START to value 0"]
impl crate::Resettable for SyscodeStartSpec {
    const RESET_VALUE: u32 = 0;
}
