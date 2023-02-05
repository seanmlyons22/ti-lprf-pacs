#[doc = "Register `OPTIONS` reader"]
pub struct R(crate::R<OPTIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS` writer"]
pub struct W(crate::W<OPTIONS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS_SPEC>;
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
impl From<crate::W<OPTIONS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKCP_CONFIGURATION` reader - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
pub type PKCP_CONFIGURATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKCP_CONFIGURATION` writer - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
pub type PKCP_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPTIONS_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED2` reader - 4:2\\]
Ignore on read"]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 4:2\\]
Ignore on read"]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTIONS_SPEC, u8, u8, 3, O>;
#[doc = "Field `SEQUENCER_CONFIGURATION` reader - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
pub type SEQUENCER_CONFIGURATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQUENCER_CONFIGURATION` writer - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
pub type SEQUENCER_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPTIONS_SPEC, u8, u8, 2, O>;
#[doc = "Field `PROGRAM_RAM` reader - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
pub type PROGRAM_RAM_R = crate::BitReader<bool>;
#[doc = "Field `PROGRAM_RAM` writer - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
pub type PROGRAM_RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `PROTECTION_OPTION` reader - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
pub type PROTECTION_OPTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROTECTION_OPTION` writer - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
pub type PROTECTION_OPTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPTIONS_SPEC, u8, u8, 3, O>;
#[doc = "Field `INT_MASKING` reader - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
pub type INT_MASKING_R = crate::BitReader<bool>;
#[doc = "Field `INT_MASKING` writer - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
pub type INT_MASKING_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Ignore on read"]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Ignore on read"]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTIONS_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PKCP_CONFIGURATION_R {
        PKCP_CONFIGURATION_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SEQUENCER_CONFIGURATION_R {
        SEQUENCER_CONFIGURATION_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&self) -> PROGRAM_RAM_R {
        PROGRAM_RAM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&self) -> PROTECTION_OPTION_R {
        PROTECTION_OPTION_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&self) -> INT_MASKING_R {
        INT_MASKING_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline(always)]
    #[must_use]
    pub fn pkcp_configuration(&mut self) -> PKCP_CONFIGURATION_W<0> {
        PKCP_CONFIGURATION_W::new(self)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sequencer_configuration(&mut self) -> SEQUENCER_CONFIGURATION_W<5> {
        SEQUENCER_CONFIGURATION_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn program_ram(&mut self) -> PROGRAM_RAM_W<7> {
        PROGRAM_RAM_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn protection_option(&mut self) -> PROTECTION_OPTION_W<8> {
        PROTECTION_OPTION_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn int_masking(&mut self) -> INT_MASKING_W<11> {
        INT_MASKING_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options](index.html) module"]
pub struct OPTIONS_SPEC;
impl crate::RegisterSpec for OPTIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options::R](R) reader structure"]
impl crate::Readable for OPTIONS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options::W](W) writer structure"]
impl crate::Writable for OPTIONS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTIONS to value 0x20"]
impl crate::Resettable for OPTIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
