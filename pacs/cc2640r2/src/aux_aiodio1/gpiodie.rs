#[doc = "Register `GPIODIE` reader"]
pub struct R(crate::R<GPIODIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIODIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIODIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIODIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIODIE` writer"]
pub struct W(crate::W<GPIODIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIODIE_SPEC>;
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
impl From<crate::W<GPIODIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIODIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO7_0` reader - 7:0\\]
Write 1 to bit index n in this bit vector to enable digital input buffer for AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to disable digital input buffer for AUXIO\\[8i+n\\]. You must enable the digital input buffer for AUXIO\\[8i+n\\]
to read the pin value in GPIODIN. You must disable the digital input buffer for analog input or pins that float to avoid current leakage."]
pub type IO7_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO7_0` writer - 7:0\\]
Write 1 to bit index n in this bit vector to enable digital input buffer for AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to disable digital input buffer for AUXIO\\[8i+n\\]. You must enable the digital input buffer for AUXIO\\[8i+n\\]
to read the pin value in GPIODIN. You must disable the digital input buffer for analog input or pins that float to avoid current leakage."]
pub type IO7_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIODIE_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIODIE_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write 1 to bit index n in this bit vector to enable digital input buffer for AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to disable digital input buffer for AUXIO\\[8i+n\\]. You must enable the digital input buffer for AUXIO\\[8i+n\\]
to read the pin value in GPIODIN. You must disable the digital input buffer for analog input or pins that float to avoid current leakage."]
    #[inline(always)]
    pub fn io7_0(&self) -> IO7_0_R {
        IO7_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write 1 to bit index n in this bit vector to enable digital input buffer for AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to disable digital input buffer for AUXIO\\[8i+n\\]. You must enable the digital input buffer for AUXIO\\[8i+n\\]
to read the pin value in GPIODIN. You must disable the digital input buffer for analog input or pins that float to avoid current leakage."]
    #[inline(always)]
    #[must_use]
    pub fn io7_0(&mut self) -> IO7_0_W<0> {
        IO7_0_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodie](index.html) module"]
pub struct GPIODIE_SPEC;
impl crate::RegisterSpec for GPIODIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiodie::R](R) reader structure"]
impl crate::Readable for GPIODIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiodie::W](W) writer structure"]
impl crate::Writable for GPIODIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIODIE to value 0"]
impl crate::Resettable for GPIODIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
