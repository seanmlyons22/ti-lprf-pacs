#[doc = "Register `ICTR` reader"]
pub type R = crate::R<IctrSpec>;
#[doc = "Register `ICTR` writer"]
pub type W = crate::W<IctrSpec>;
#[doc = "Field `INTLINESNUM` reader - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
pub type IntlinesnumR = crate::FieldReader;
#[doc = "Field `INTLINESNUM` writer - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
pub type IntlinesnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> IntlinesnumR {
        IntlinesnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the number of the highest implemented register in each of the NVIC control register sets, or in the case of NVIC_IPR*n, 4xINTLINESNUM"]
    #[inline(always)]
    #[must_use]
    pub fn intlinesnum(&mut self) -> IntlinesnumW<IctrSpec> {
        IntlinesnumW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<IctrSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Provides information about the interrupt controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ictr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IctrSpec;
impl crate::RegisterSpec for IctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictr::R`](R) reader structure"]
impl crate::Readable for IctrSpec {}
#[doc = "`write(|w| ..)` method takes [`ictr::W`](W) writer structure"]
impl crate::Writable for IctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICTR to value 0"]
impl crate::Resettable for IctrSpec {
    const RESET_VALUE: u32 = 0;
}
