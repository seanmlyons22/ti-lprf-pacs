#[doc = "Register `ACCSHIFT` reader"]
pub type R = crate::R<AccshiftSpec>;
#[doc = "Register `ACCSHIFT` writer"]
pub type W = crate::W<AccshiftSpec>;
#[doc = "Field `ASR1` reader - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
pub type Asr1R = crate::BitReader;
#[doc = "Field `ASR1` writer - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
pub type Asr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSR1` reader - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
pub type Lsr1R = crate::BitReader;
#[doc = "Field `LSR1` writer - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
pub type Lsr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSL1` reader - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
pub type Lsl1R = crate::BitReader;
#[doc = "Field `LSL1` writer - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
pub type Lsl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
    #[inline(always)]
    pub fn asr1(&self) -> Asr1R {
        Asr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
    #[inline(always)]
    pub fn lsr1(&self) -> Lsr1R {
        Lsr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
    #[inline(always)]
    pub fn lsl1(&self) -> Lsl1R {
        Lsl1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
    #[inline(always)]
    #[must_use]
    pub fn asr1(&mut self) -> Asr1W<AccshiftSpec> {
        Asr1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
    #[inline(always)]
    #[must_use]
    pub fn lsr1(&mut self) -> Lsr1W<AccshiftSpec> {
        Lsr1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn lsl1(&mut self) -> Lsl1W<AccshiftSpec> {
        Lsl1W::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<AccshiftSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`accshift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`accshift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccshiftSpec;
impl crate::RegisterSpec for AccshiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accshift::R`](R) reader structure"]
impl crate::Readable for AccshiftSpec {}
#[doc = "`write(|w| ..)` method takes [`accshift::W`](W) writer structure"]
impl crate::Writable for AccshiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACCSHIFT to value 0"]
impl crate::Resettable for AccshiftSpec {
    const RESET_VALUE: u32 = 0;
}
