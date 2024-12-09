#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "Field `MINREV` reader - 3:0\\]
Minor revision of IP 0-15."]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - 7:4\\]
Major revision of IP 0-15"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `INSTIDX` reader - 11:8\\]
IP Instance ID number. If multiple instances of IP exist in the device, this field can identify the instance number (0-15)."]
pub type InstidxR = crate::FieldReader;
#[doc = "Field `STDIPOFF` reader - 15:12\\]
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address)"]
pub type StdipoffR = crate::FieldReader;
#[doc = "Field `MODID` reader - 31:16\\]
Module identifier used to uniquely identify this IP."]
pub type ModidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of IP 0-15."]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of IP 0-15"]
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
Standard IP MMR block offset. Standard IP MMRs are the set of from aggregated IRQ registers till DTB. 0: Standard IP MMRs do not exist 0x1-0xF: Standard IP MMRs begin at offset of (64*STDIPOFF from the base IP address)"]
    #[inline(always)]
    pub fn stdipoff(&self) -> StdipoffR {
        StdipoffR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identifier used to uniquely identify this IP."]
    #[inline(always)]
    pub fn modid(&self) -> ModidR {
        ModidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
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
#[doc = "`reset()` method sets DESC to value 0x2548_0000"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0x2548_0000;
}
