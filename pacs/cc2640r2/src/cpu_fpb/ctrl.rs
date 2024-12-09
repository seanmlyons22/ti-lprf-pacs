#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `NUM_CODE1` reader - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
pub type NumCode1R = crate::FieldReader;
#[doc = "Field `NUM_LIT` reader - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
pub type NumLitR = crate::FieldReader;
#[doc = "Field `NUM_CODE2` reader - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
pub type NumCode2R = crate::FieldReader;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    pub fn num_code1(&self) -> NumCode1R {
        NumCode1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    pub fn num_lit(&self) -> NumLitR {
        NumLitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub fn num_code2(&self) -> NumCode2R {
        NumCode2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CtrlSpec> {
        KeyW::new(self, 1)
    }
}
#[doc = "Control This register is used to enable the flash patch block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0260"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0260;
}
