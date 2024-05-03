#[doc = "Register `SHIFT` reader"]
pub type R = crate::R<ShiftSpec>;
#[doc = "Register `SHIFT` writer"]
pub type W = crate::W<ShiftSpec>;
#[doc = "Field `NUM_BITS_TO_SHIFT` reader - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub type NumBitsToShiftR = crate::FieldReader;
#[doc = "Field `NUM_BITS_TO_SHIFT` writer - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub type NumBitsToShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED11` reader - 31:5\\]
Set to zero on write, ignore on read"]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:5\\]
Set to zero on write, ignore on read"]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&self) -> NumBitsToShiftR {
        NumBitsToShiftR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    #[must_use]
    pub fn num_bits_to_shift(&mut self) -> NumBitsToShiftW<ShiftSpec> {
        NumBitsToShiftW::new(self, 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<ShiftSpec> {
        Reserved11W::new(self, 5)
    }
}
#[doc = "PKA Bit Shift Value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftSpec;
impl crate::RegisterSpec for ShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shift::R`](R) reader structure"]
impl crate::Readable for ShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`shift::W`](W) writer structure"]
impl crate::Writable for ShiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFT to value 0"]
impl crate::Resettable for ShiftSpec {
    const RESET_VALUE: u32 = 0;
}
