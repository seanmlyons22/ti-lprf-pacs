#[doc = "Register `TBV` reader"]
pub type R = crate::R<TbvSpec>;
#[doc = "Register `TBV` writer"]
pub type W = crate::W<TbvSpec>;
#[doc = "Field `TBV` reader - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
pub type TbvR = crate::FieldReader<u32>;
#[doc = "Field `TBV` writer - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
pub type TbvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
    #[inline(always)]
    pub fn tbv(&self) -> TbvR {
        TbvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn tbv(&mut self) -> TbvW<TbvSpec> {
        TbvW::new(self, 0)
    }
}
#[doc = "Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbvSpec;
impl crate::RegisterSpec for TbvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbv::R`](R) reader structure"]
impl crate::Readable for TbvSpec {}
#[doc = "`write(|w| ..)` method takes [`tbv::W`](W) writer structure"]
impl crate::Writable for TbvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBV to value 0xffff"]
impl crate::Resettable for TbvSpec {
    const RESET_VALUE: u32 = 0xffff;
}
