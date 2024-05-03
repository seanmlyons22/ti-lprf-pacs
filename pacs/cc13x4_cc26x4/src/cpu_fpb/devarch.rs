#[doc = "Register `DEVARCH` reader"]
pub type R = crate::R<DevarchSpec>;
#[doc = "Register `DEVARCH` writer"]
pub type W = crate::W<DevarchSpec>;
#[doc = "Field `ARCHPART` reader - 11:0\\]
Defines the architecture of the component"]
pub type ArchpartR = crate::FieldReader<u16>;
#[doc = "Field `ARCHPART` writer - 11:0\\]
Defines the architecture of the component"]
pub type ArchpartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ARCHVER` reader - 15:12\\]
Defines the architecture version of the component"]
pub type ArchverR = crate::FieldReader;
#[doc = "Field `ARCHVER` writer - 15:12\\]
Defines the architecture version of the component"]
pub type ArchverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REVISION` reader - 19:16\\]
Defines the architecture revision of the component"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 19:16\\]
Defines the architecture revision of the component"]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESENT` reader - 20:20\\]
Defines that the DEVARCH register is present"]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - 20:20\\]
Defines that the DEVARCH register is present"]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARCHITECT` reader - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
pub type ArchitectR = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECT` writer - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
pub type ArchitectW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the architecture of the component"]
    #[inline(always)]
    pub fn archpart(&self) -> ArchpartR {
        ArchpartR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Defines the architecture version of the component"]
    #[inline(always)]
    pub fn archver(&self) -> ArchverR {
        ArchverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision of the component"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Defines that the DEVARCH register is present"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
    #[inline(always)]
    pub fn architect(&self) -> ArchitectR {
        ArchitectR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the architecture of the component"]
    #[inline(always)]
    #[must_use]
    pub fn archpart(&mut self) -> ArchpartW<DevarchSpec> {
        ArchpartW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Defines the architecture version of the component"]
    #[inline(always)]
    #[must_use]
    pub fn archver(&mut self) -> ArchverW<DevarchSpec> {
        ArchverW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision of the component"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<DevarchSpec> {
        RevisionW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Defines that the DEVARCH register is present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<DevarchSpec> {
        PresentW::new(self, 20)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
    #[inline(always)]
    #[must_use]
    pub fn architect(&mut self) -> ArchitectW<DevarchSpec> {
        ArchitectW::new(self, 21)
    }
}
#[doc = "Provides CoreSight discovery information for the FPB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devarch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devarch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevarchSpec;
impl crate::RegisterSpec for DevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devarch::R`](R) reader structure"]
impl crate::Readable for DevarchSpec {}
#[doc = "`write(|w| ..)` method takes [`devarch::W`](W) writer structure"]
impl crate::Writable for DevarchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVARCH to value 0"]
impl crate::Resettable for DevarchSpec {
    const RESET_VALUE: u32 = 0;
}
