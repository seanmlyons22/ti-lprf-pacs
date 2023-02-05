#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "Field `MINREV` reader - 3:0\\]
Minor rev of the IP"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MINREV` writer - 3:0\\]
Minor rev of the IP"]
pub type MinrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJREV` reader - 7:4\\]
Major rev of the IP"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `MAJREV` writer - 7:4\\]
Major rev of the IP"]
pub type MajrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INSTNUM` reader - 11:8\\]
IP Instance Number. If multiple instances of IP exist in the device, this field can identify the instance number"]
pub type InstnumR = crate::FieldReader;
#[doc = "Field `INSTNUM` writer - 11:8\\]
IP Instance Number. If multiple instances of IP exist in the device, this field can identify the instance number"]
pub type InstnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stdipoff {
    #[doc = "1: These MMRs begin at offset 64*STDIPOFF from IP base address"]
    Stdipmmr = 1,
    #[doc = "0: STDIP MMRs do not exist"]
    NoStdipmmr = 0,
}
impl From<Stdipoff> for u8 {
    #[inline(always)]
    fn from(variant: Stdipoff) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stdipoff {
    type Ux = u8;
}
impl crate::IsEnum for Stdipoff {}
#[doc = "Field `STDIPOFF` reader - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB."]
pub type StdipoffR = crate::FieldReader<Stdipoff>;
impl StdipoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stdipoff> {
        match self.bits {
            1 => Some(Stdipoff::Stdipmmr),
            0 => Some(Stdipoff::NoStdipmmr),
            _ => None,
        }
    }
    #[doc = "These MMRs begin at offset 64*STDIPOFF from IP base address"]
    #[inline(always)]
    pub fn is_stdipmmr(&self) -> bool {
        *self == Stdipoff::Stdipmmr
    }
    #[doc = "STDIP MMRs do not exist"]
    #[inline(always)]
    pub fn is_no_stdipmmr(&self) -> bool {
        *self == Stdipoff::NoStdipmmr
    }
}
#[doc = "Field `STDIPOFF` writer - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB."]
pub type StdipoffW<'a, REG> = crate::FieldWriter<'a, REG, 4, Stdipoff>;
impl<'a, REG> StdipoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "These MMRs begin at offset 64*STDIPOFF from IP base address"]
    #[inline(always)]
    pub fn stdipmmr(self) -> &'a mut crate::W<REG> {
        self.variant(Stdipoff::Stdipmmr)
    }
    #[doc = "STDIP MMRs do not exist"]
    #[inline(always)]
    pub fn no_stdipmmr(self) -> &'a mut crate::W<REG> {
        self.variant(Stdipoff::NoStdipmmr)
    }
}
#[doc = "Field `MODULEID` reader - 31:16\\]
Module identifier used to uniquely identify this IP."]
pub type ModuleidR = crate::FieldReader<u16>;
#[doc = "Field `MODULEID` writer - 31:16\\]
Module identifier used to uniquely identify this IP."]
pub type ModuleidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor rev of the IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major rev of the IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
IP Instance Number. If multiple instances of IP exist in the device, this field can identify the instance number"]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB."]
    #[inline(always)]
    pub fn stdipoff(&self) -> StdipoffR {
        StdipoffR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identifier used to uniquely identify this IP."]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Minor rev of the IP"]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MinrevW<DescSpec> {
        MinrevW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major rev of the IP"]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MajrevW<DescSpec> {
        MajrevW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
IP Instance Number. If multiple instances of IP exist in the device, this field can identify the instance number"]
    #[inline(always)]
    #[must_use]
    pub fn instnum(&mut self) -> InstnumW<DescSpec> {
        InstnumW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB."]
    #[inline(always)]
    #[must_use]
    pub fn stdipoff(&mut self) -> StdipoffW<DescSpec> {
        StdipoffW::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identifier used to uniquely identify this IP."]
    #[inline(always)]
    #[must_use]
    pub fn moduleid(&mut self) -> ModuleidW<DescSpec> {
        ModuleidW::new(self, 16)
    }
}
#[doc = "This register identifies the peripheral and its exact version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`write(|w| ..)` method takes [`desc::W`](W) writer structure"]
impl crate::Writable for DescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESC to value 0x0141_1010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0x0141_1010;
}
