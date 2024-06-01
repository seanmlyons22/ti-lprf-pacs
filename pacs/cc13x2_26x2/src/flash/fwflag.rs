#[doc = "Register `FWFLAG` reader"]
pub type R = crate::R<FwflagSpec>;
#[doc = "Register `FWFLAG` writer"]
pub type W = crate::W<FwflagSpec>;
#[doc = "Field `FWFLAG` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FwflagR = crate::FieldReader;
#[doc = "Field `FWFLAG` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FwflagW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwflag(&self) -> FwflagR {
        FwflagR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwflag(&mut self) -> FwflagW<FwflagSpec> {
        FwflagW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<FwflagSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwflagSpec;
impl crate::RegisterSpec for FwflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwflag::R`](R) reader structure"]
impl crate::Readable for FwflagSpec {}
#[doc = "`write(|w| ..)` method takes [`fwflag::W`](W) writer structure"]
impl crate::Writable for FwflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWFLAG to value 0"]
impl crate::Resettable for FwflagSpec {
    const RESET_VALUE: u32 = 0;
}
