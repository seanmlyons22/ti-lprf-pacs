#[doc = "Register `MASK0` reader"]
pub type R = crate::R<Mask0Spec>;
#[doc = "Register `MASK0` writer"]
pub type W = crate::W<Mask0Spec>;
#[doc = "Field `MASK` reader - 3:0\\]
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - 3:0\\]
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0x0f) as u8)
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
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<Mask0Spec> {
        MaskW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Mask0Spec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mask0Spec;
impl crate::RegisterSpec for Mask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask0::R`](R) reader structure"]
impl crate::Readable for Mask0Spec {}
#[doc = "`write(|w| ..)` method takes [`mask0::W`](W) writer structure"]
impl crate::Writable for Mask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK0 to value 0"]
impl crate::Resettable for Mask0Spec {
    const RESET_VALUE: u32 = 0;
}
