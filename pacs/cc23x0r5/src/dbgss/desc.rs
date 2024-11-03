#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "3:0\\]
Minor revision of IP (0-15).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Minrev {
    #[doc = "15: Maximum possible value"]
    Max = 15,
    #[doc = "0: Minimum Value"]
    Min = 0,
}
impl From<Minrev> for u8 {
    #[inline(always)]
    fn from(variant: Minrev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Minrev {
    type Ux = u8;
}
impl crate::IsEnum for Minrev {}
#[doc = "Field `MINREV` reader - 3:0\\]
Minor revision of IP (0-15)."]
pub type MinrevR = crate::FieldReader<Minrev>;
impl MinrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Minrev> {
        match self.bits {
            15 => Some(Minrev::Max),
            0 => Some(Minrev::Min),
            _ => None,
        }
    }
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Minrev::Max
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Minrev::Min
    }
}
#[doc = "Field `MINREV` writer - 3:0\\]
Minor revision of IP (0-15)."]
pub type MinrevW<'a, REG> = crate::FieldWriter<'a, REG, 4, Minrev>;
impl<'a, REG> MinrevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Minrev::Max)
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Minrev::Min)
    }
}
#[doc = "7:4\\]
Major revision of IP (0-15).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Majrev {
    #[doc = "15: Maximum possible value"]
    Max = 15,
    #[doc = "0: Minimum Value"]
    Min = 0,
}
impl From<Majrev> for u8 {
    #[inline(always)]
    fn from(variant: Majrev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Majrev {
    type Ux = u8;
}
impl crate::IsEnum for Majrev {}
#[doc = "Field `MAJREV` reader - 7:4\\]
Major revision of IP (0-15)."]
pub type MajrevR = crate::FieldReader<Majrev>;
impl MajrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Majrev> {
        match self.bits {
            15 => Some(Majrev::Max),
            0 => Some(Majrev::Min),
            _ => None,
        }
    }
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Majrev::Max
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Majrev::Min
    }
}
#[doc = "Field `MAJREV` writer - 7:4\\]
Major revision of IP (0-15)."]
pub type MajrevW<'a, REG> = crate::FieldWriter<'a, REG, 4, Majrev>;
impl<'a, REG> MajrevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Majrev::Max)
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Majrev::Min)
    }
}
#[doc = "11:8\\]
IP Instance ID number. If multiple instances of IP exist in the device, this field can identify the instance number (0-15).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Instidx {
    #[doc = "15: Maximum possible value"]
    Max = 15,
    #[doc = "0: Minimum Value"]
    Min = 0,
}
impl From<Instidx> for u8 {
    #[inline(always)]
    fn from(variant: Instidx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Instidx {
    type Ux = u8;
}
impl crate::IsEnum for Instidx {}
#[doc = "Field `INSTIDX` reader - 11:8\\]
IP Instance ID number. If multiple instances of IP exist in the device, this field can identify the instance number (0-15)."]
pub type InstidxR = crate::FieldReader<Instidx>;
impl InstidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Instidx> {
        match self.bits {
            15 => Some(Instidx::Max),
            0 => Some(Instidx::Min),
            _ => None,
        }
    }
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Instidx::Max
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Instidx::Min
    }
}
#[doc = "Field `INSTIDX` writer - 11:8\\]
IP Instance ID number. If multiple instances of IP exist in the device, this field can identify the instance number (0-15)."]
pub type InstidxW<'a, REG> = crate::FieldWriter<'a, REG, 4, Instidx>;
impl<'a, REG> InstidxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Instidx::Max)
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Instidx::Min)
    }
}
#[doc = "15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address) 0: STDIP MMRs do not exist 0x1-0xF: These MMRs begin at offset 64*STDIPOFF from IP base address\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stdipoff {
    #[doc = "15: Maximum possible value"]
    Max = 15,
    #[doc = "0: Minimum Value"]
    Min = 0,
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
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address) 0: STDIP MMRs do not exist 0x1-0xF: These MMRs begin at offset 64*STDIPOFF from IP base address"]
pub type StdipoffR = crate::FieldReader<Stdipoff>;
impl StdipoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stdipoff> {
        match self.bits {
            15 => Some(Stdipoff::Max),
            0 => Some(Stdipoff::Min),
            _ => None,
        }
    }
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Stdipoff::Max
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Stdipoff::Min
    }
}
#[doc = "Field `STDIPOFF` writer - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address) 0: STDIP MMRs do not exist 0x1-0xF: These MMRs begin at offset 64*STDIPOFF from IP base address"]
pub type StdipoffW<'a, REG> = crate::FieldWriter<'a, REG, 4, Stdipoff>;
impl<'a, REG> StdipoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Stdipoff::Max)
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Stdipoff::Min)
    }
}
#[doc = "31:16\\]
Module identifier used to uniquely identify this IP.\n\nValue on reset: 45645"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Moduleid {
    #[doc = "65535: Maximum possible value"]
    Max = 65535,
    #[doc = "0: Minimum Value"]
    Min = 0,
}
impl From<Moduleid> for u16 {
    #[inline(always)]
    fn from(variant: Moduleid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Moduleid {
    type Ux = u16;
}
impl crate::IsEnum for Moduleid {}
#[doc = "Field `MODULEID` reader - 31:16\\]
Module identifier used to uniquely identify this IP."]
pub type ModuleidR = crate::FieldReader<Moduleid>;
impl ModuleidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Moduleid> {
        match self.bits {
            65535 => Some(Moduleid::Max),
            0 => Some(Moduleid::Min),
            _ => None,
        }
    }
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Moduleid::Max
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Moduleid::Min
    }
}
#[doc = "Field `MODULEID` writer - 31:16\\]
Module identifier used to uniquely identify this IP."]
pub type ModuleidW<'a, REG> = crate::FieldWriter<'a, REG, 16, Moduleid>;
impl<'a, REG> ModuleidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Maximum possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Moduleid::Max)
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Moduleid::Min)
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of IP (0-15)."]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of IP (0-15)."]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
IP Instance ID number. If multiple instances of IP exist in the device, this field can identify the instance number (0-15)."]
    #[inline(always)]
    pub fn instidx(&self) -> InstidxR {
        InstidxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address) 0: STDIP MMRs do not exist 0x1-0xF: These MMRs begin at offset 64*STDIPOFF from IP base address"]
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
Minor revision of IP (0-15)."]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MinrevW<DescSpec> {
        MinrevW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of IP (0-15)."]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MajrevW<DescSpec> {
        MajrevW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
IP Instance ID number. If multiple instances of IP exist in the device, this field can identify the instance number (0-15)."]
    #[inline(always)]
    #[must_use]
    pub fn instidx(&mut self) -> InstidxW<DescSpec> {
        InstidxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address) 0: STDIP MMRs do not exist 0x1-0xF: These MMRs begin at offset 64*STDIPOFF from IP base address"]
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
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DESC to value 0xb24d_1010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0xb24d_1010;
}
