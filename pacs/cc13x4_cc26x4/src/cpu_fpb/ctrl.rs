#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Enables the FPB"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enables the FPB"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
pub type KeyR = crate::BitReader;
#[doc = "Field `KEY` writer - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUM_CODE_7_4_` reader - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NumCode7_4_R = crate::FieldReader;
#[doc = "Field `NUM_CODE_7_4_` writer - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NumCode7_4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUM_LIT` reader - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
pub type NumLitR = crate::FieldReader;
#[doc = "Field `NUM_LIT` writer - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
pub type NumLitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUM_CODE_14_12_` reader - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NumCode14_12_R = crate::FieldReader;
#[doc = "Field `NUM_CODE_14_12_` writer - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NumCode14_12_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED15` reader - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED15` writer - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `REV` reader - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
pub type RevR = crate::FieldReader;
#[doc = "Field `REV` writer - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
pub type RevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the FPB"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    pub fn num_code_7_4_(&self) -> NumCode7_4_R {
        NumCode7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
    #[inline(always)]
    pub fn num_lit(&self) -> NumLitR {
        NumLitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    pub fn num_code_14_12_(&self) -> NumCode14_12_R {
        NumCode14_12_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:27 - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the FPB"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CtrlSpec> {
        KeyW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CtrlSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    #[must_use]
    pub fn num_code_7_4_(&mut self) -> NumCode7_4_W<CtrlSpec> {
        NumCode7_4_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
    #[inline(always)]
    #[must_use]
    pub fn num_lit(&mut self) -> NumLitW<CtrlSpec> {
        NumLitW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    #[must_use]
    pub fn num_code_14_12_(&mut self) -> NumCode14_12_W<CtrlSpec> {
        NumCode14_12_W::new(self, 12)
    }
    #[doc = "Bits 15:27 - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<CtrlSpec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> RevW<CtrlSpec> {
        RevW::new(self, 28)
    }
}
#[doc = "Provides FPB implementation information, and the global enable for the FPB unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
