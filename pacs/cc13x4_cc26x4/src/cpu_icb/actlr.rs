#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ActlrSpec>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ActlrSpec>;
#[doc = "Field `DISMCYCINT` reader - 0:0\\]
Disable dual-issue."]
pub type DismcycintR = crate::BitReader;
#[doc = "Field `DISMCYCINT` writer - 0:0\\]
Disable dual-issue."]
pub type DismcycintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFOLD` reader - 2:2\\]
Disable dual-issue."]
pub type DisfoldR = crate::BitReader;
#[doc = "Field `DISFOLD` writer - 2:2\\]
Disable dual-issue."]
pub type DisfoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DISOOFP` reader - 9:9\\]
Disable out-of-order FP instruction completion"]
pub type DisoofpR = crate::BitReader;
#[doc = "Field `DISOOFP` writer - 9:9\\]
Disable out-of-order FP instruction completion"]
pub type DisoofpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPEXCODIS` reader - 10:10\\]
Disable FPU exception outputs"]
pub type FpexcodisR = crate::BitReader;
#[doc = "Field `FPEXCODIS` writer - 10:10\\]
Disable FPU exception outputs"]
pub type FpexcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `RESERVED11` writer - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISITMATBFLUSH` reader - 12:12\\]
Disable ATB Flush"]
pub type DisitmatbflushR = crate::BitReader;
#[doc = "Field `DISITMATBFLUSH` writer - 12:12\\]
Disable ATB Flush"]
pub type DisitmatbflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED13` reader - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED13` writer - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EXTEXCLALL` reader - 29:29\\]
External Exclusives Allowed with no MPU"]
pub type ExtexclallR = crate::BitReader;
#[doc = "Field `EXTEXCLALL` writer - 29:29\\]
External Exclusives Allowed with no MPU"]
pub type ExtexclallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::FieldReader;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable dual-issue."]
    #[inline(always)]
    pub fn dismcycint(&self) -> DismcycintR {
        DismcycintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable dual-issue."]
    #[inline(always)]
    pub fn disfold(&self) -> DisfoldR {
        DisfoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Disable out-of-order FP instruction completion"]
    #[inline(always)]
    pub fn disoofp(&self) -> DisoofpR {
        DisoofpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Disable FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FpexcodisR {
        FpexcodisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Disable ATB Flush"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DisitmatbflushR {
        DisitmatbflushR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28 - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
External Exclusives Allowed with no MPU"]
    #[inline(always)]
    pub fn extexclall(&self) -> ExtexclallR {
        ExtexclallR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable dual-issue."]
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DismcycintW<ActlrSpec> {
        DismcycintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ActlrSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable dual-issue."]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DisfoldW<ActlrSpec> {
        DisfoldW::new(self, 2)
    }
    #[doc = "Bits 3:8 - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<ActlrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 9 - 9:9\\]
Disable out-of-order FP instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn disoofp(&mut self) -> DisoofpW<ActlrSpec> {
        DisoofpW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Disable FPU exception outputs"]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FpexcodisW<ActlrSpec> {
        FpexcodisW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<ActlrSpec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Disable ATB Flush"]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DisitmatbflushW<ActlrSpec> {
        DisitmatbflushW::new(self, 12)
    }
    #[doc = "Bits 13:28 - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<ActlrSpec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bit 29 - 29:29\\]
External Exclusives Allowed with no MPU"]
    #[inline(always)]
    #[must_use]
    pub fn extexclall(&mut self) -> ExtexclallW<ActlrSpec> {
        ExtexclallW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> Reserved30W<ActlrSpec> {
        Reserved30W::new(self, 30)
    }
}
#[doc = "Provides IMPLEMENTATION DEFINED configuration and control options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActlrSpec;
impl crate::RegisterSpec for ActlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ActlrSpec {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ActlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ActlrSpec {
    const RESET_VALUE: u32 = 0;
}
