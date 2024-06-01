#[doc = "Register `TAMATCHR` reader"]
pub type R = crate::R<TamatchrSpec>;
#[doc = "Register `TAMATCHR` writer"]
pub type W = crate::W<TamatchrSpec>;
#[doc = "Field `TAMATCHR` reader - 31:0\\]
GPT Timer A Match Register"]
pub type TamatchrR = crate::FieldReader<u32>;
#[doc = "Field `TAMATCHR` writer - 31:0\\]
GPT Timer A Match Register"]
pub type TamatchrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Match Register"]
    #[inline(always)]
    pub fn tamatchr(&self) -> TamatchrR {
        TamatchrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Match Register"]
    #[inline(always)]
    #[must_use]
    pub fn tamatchr(&mut self) -> TamatchrW<TamatchrSpec> {
        TamatchrW::new(self, 0)
    }
}
#[doc = "Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamatchr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamatchr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamatchrSpec;
impl crate::RegisterSpec for TamatchrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamatchr::R`](R) reader structure"]
impl crate::Readable for TamatchrSpec {}
#[doc = "`write(|w| ..)` method takes [`tamatchr::W`](W) writer structure"]
impl crate::Writable for TamatchrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMATCHR to value 0xffff_ffff"]
impl crate::Resettable for TamatchrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
