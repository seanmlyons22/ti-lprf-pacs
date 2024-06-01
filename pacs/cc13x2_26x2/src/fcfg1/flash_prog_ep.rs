#[doc = "Register `FLASH_PROG_EP` reader"]
pub type R = crate::R<FlashProgEpSpec>;
#[doc = "Register `FLASH_PROG_EP` writer"]
pub type W = crate::W<FlashProgEpSpec>;
#[doc = "Field `PROGRAM_PW` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ProgramPwR = crate::FieldReader<u16>;
#[doc = "Field `PROGRAM_PW` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ProgramPwW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAX_EP` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type MaxEpR = crate::FieldReader<u16>;
#[doc = "Field `MAX_EP` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type MaxEpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn program_pw(&self) -> ProgramPwR {
        ProgramPwR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ep(&self) -> MaxEpR {
        MaxEpR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn program_pw(&mut self) -> ProgramPwW<FlashProgEpSpec> {
        ProgramPwW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_ep(&mut self) -> MaxEpW<FlashProgEpSpec> {
        MaxEpW::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_prog_ep::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_prog_ep::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashProgEpSpec;
impl crate::RegisterSpec for FlashProgEpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_prog_ep::R`](R) reader structure"]
impl crate::Readable for FlashProgEpSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_prog_ep::W`](W) writer structure"]
impl crate::Writable for FlashProgEpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PROG_EP to value 0x0fa0_0010"]
impl crate::Resettable for FlashProgEpSpec {
    const RESET_VALUE: u32 = 0x0fa0_0010;
}
