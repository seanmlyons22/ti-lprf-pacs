#[doc = "Register `OPTIONS` reader"]
pub type R = crate::R<OptionsSpec>;
#[doc = "Register `OPTIONS` writer"]
pub type W = crate::W<OptionsSpec>;
#[doc = "Field `PKCP_CONFIGURATION` reader - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
pub type PkcpConfigurationR = crate::FieldReader;
#[doc = "Field `RESERVED2` reader - 4:2\\]
Ignore on read"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SEQUENCER_CONFIGURATION` reader - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
pub type SequencerConfigurationR = crate::FieldReader;
#[doc = "Field `PROGRAM_RAM` reader - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
pub type ProgramRamR = crate::BitReader;
#[doc = "Field `PROTECTION_OPTION` reader - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
pub type ProtectionOptionR = crate::FieldReader;
#[doc = "Field `INT_MASKING` reader - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
pub type IntMaskingR = crate::BitReader;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Ignore on read"]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PkcpConfigurationR {
        PkcpConfigurationR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SequencerConfigurationR {
        SequencerConfigurationR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&self) -> ProgramRamR {
        ProgramRamR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&self) -> ProtectionOptionR {
        ProtectionOptionR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&self) -> IntMaskingR {
        IntMaskingR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`options::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionsSpec;
impl crate::RegisterSpec for OptionsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`options::R`](R) reader structure"]
impl crate::Readable for OptionsSpec {}
#[doc = "`write(|w| ..)` method takes [`options::W`](W) writer structure"]
impl crate::Writable for OptionsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTIONS to value 0x20"]
impl crate::Resettable for OptionsSpec {
    const RESET_VALUE: u32 = 0x20;
}
