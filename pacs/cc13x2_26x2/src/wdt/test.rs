#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestEn {
    #[doc = "1: Test mode Enabled"]
    En = 1,
    #[doc = "0: Test mode Disabled"]
    Dis = 0,
}
impl From<TestEn> for bool {
    #[inline(always)]
    fn from(variant: TestEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST_EN` reader - 0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
pub type TestEnR = crate::BitReader<TestEn>;
impl TestEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestEn {
        match self.bits {
            true => TestEn::En,
            false => TestEn::Dis,
        }
    }
    #[doc = "Test mode Enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TestEn::En
    }
    #[doc = "Test mode Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TestEn::Dis
    }
}
#[doc = "Field `TEST_EN` writer - 0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
pub type TestEnW<'a, REG> = crate::BitWriter<'a, REG, TestEn>;
impl<'a, REG> TestEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test mode Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TestEn::En)
    }
    #[doc = "Test mode Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TestEn::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stall {
    #[doc = "1: Enable STALL"]
    En = 1,
    #[doc = "0: Disable STALL"]
    Dis = 0,
}
impl From<Stall> for bool {
    #[inline(always)]
    fn from(variant: Stall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL` reader - 8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
pub type StallR = crate::BitReader<Stall>;
impl StallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stall {
        match self.bits {
            true => Stall::En,
            false => Stall::Dis,
        }
    }
    #[doc = "Enable STALL"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Stall::En
    }
    #[doc = "Disable STALL"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Stall::Dis
    }
}
#[doc = "Field `STALL` writer - 8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG, Stall>;
impl<'a, REG> StallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable STALL"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Stall::En)
    }
    #[doc = "Disable STALL"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Stall::Dis)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
    #[inline(always)]
    pub fn test_en(&self) -> TestEnR {
        TestEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn test_en(&mut self) -> TestEnW<TestSpec> {
        TestEnW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TestSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<TestSpec> {
        StallW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<TestSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Test Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
