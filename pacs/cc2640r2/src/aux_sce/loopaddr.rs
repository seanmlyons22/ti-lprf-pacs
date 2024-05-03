#[doc = "Register `LOOPADDR` reader"]
pub type R = crate::R<LoopaddrSpec>;
#[doc = "Register `LOOPADDR` writer"]
pub type W = crate::W<LoopaddrSpec>;
#[doc = "Field `START` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type StartR = crate::FieldReader<u16>;
#[doc = "Field `START` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STOP` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type StopR = crate::FieldReader<u16>;
#[doc = "Field `STOP` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<LoopaddrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<LoopaddrSpec> {
        StopW::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loopaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loopaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoopaddrSpec;
impl crate::RegisterSpec for LoopaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loopaddr::R`](R) reader structure"]
impl crate::Readable for LoopaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`loopaddr::W`](W) writer structure"]
impl crate::Writable for LoopaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOPADDR to value 0"]
impl crate::Resettable for LoopaddrSpec {
    const RESET_VALUE: u32 = 0;
}
