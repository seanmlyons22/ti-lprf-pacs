#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTERENABLE` reader - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
pub type MASTERENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MASTERENABLE` writer - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
pub type MASTERENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRTOCTRL` reader - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
pub type PRTOCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTOCTRL` writer - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
pub type PRTOCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
    #[inline(always)]
    pub fn masterenable(&self) -> MASTERENABLE_R {
        MASTERENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    pub fn prtoctrl(&self) -> PRTOCTRL_R {
        PRTOCTRL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
    #[inline(always)]
    #[must_use]
    pub fn masterenable(&mut self) -> MASTERENABLE_W<0> {
        MASTERENABLE_W::new(self)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn prtoctrl(&mut self) -> PRTOCTRL_W<5> {
        PRTOCTRL_W::new(self)
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
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
