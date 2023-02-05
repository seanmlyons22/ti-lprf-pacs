#[doc = "Register `NVIC_ISER1` reader"]
pub struct R(crate::R<NVIC_ISER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISER1` writer"]
pub struct W(crate::W<NVIC_ISER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NVIC_ISER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENA32` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
pub type SETENA32_R = crate::BitReader<bool>;
#[doc = "Field `SETENA32` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
pub type SETENA32_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER1_SPEC, bool, O>;
#[doc = "Field `SETENA33` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
pub type SETENA33_R = crate::BitReader<bool>;
#[doc = "Field `SETENA33` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
pub type SETENA33_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER1_SPEC, bool, O>;
#[doc = "Field `SETENA34` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
pub type SETENA34_R = crate::BitReader<bool>;
#[doc = "Field `SETENA34` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
pub type SETENA34_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER1_SPEC, bool, O>;
#[doc = "Field `SETENA35` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
pub type SETENA35_R = crate::BitReader<bool>;
#[doc = "Field `SETENA35` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
pub type SETENA35_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER1_SPEC, bool, O>;
#[doc = "Field `SETENA36` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
pub type SETENA36_R = crate::BitReader<bool>;
#[doc = "Field `SETENA36` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
pub type SETENA36_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER1_SPEC, bool, O>;
#[doc = "Field `SETENA37` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
pub type SETENA37_R = crate::BitReader<bool>;
#[doc = "Field `SETENA37` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
pub type SETENA37_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER1_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NVIC_ISER1_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena32(&self) -> SETENA32_R {
        SETENA32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena33(&self) -> SETENA33_R {
        SETENA33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena34(&self) -> SETENA34_R {
        SETENA34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena35(&self) -> SETENA35_R {
        SETENA35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena36(&self) -> SETENA36_R {
        SETENA36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena37(&self) -> SETENA37_R {
        SETENA37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena32(&mut self) -> SETENA32_W<0> {
        SETENA32_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena33(&mut self) -> SETENA33_W<1> {
        SETENA33_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena34(&mut self) -> SETENA34_W<2> {
        SETENA34_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena35(&mut self) -> SETENA35_W<3> {
        SETENA35_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena36(&mut self) -> SETENA36_W<4> {
        SETENA36_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena37(&mut self) -> SETENA37_W<5> {
        SETENA37_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser1](index.html) module"]
pub struct NVIC_ISER1_SPEC;
impl crate::RegisterSpec for NVIC_ISER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iser1::R](R) reader structure"]
impl crate::Readable for NVIC_ISER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iser1::W](W) writer structure"]
impl crate::Writable for NVIC_ISER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISER1 to value 0"]
impl crate::Resettable for NVIC_ISER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
