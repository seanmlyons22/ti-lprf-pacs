#[doc = "Register `EntrySCS` reader"]
pub type R = crate::R<EntryScsSpec>;
#[doc = "Register `EntrySCS` writer"]
pub type W = crate::W<EntryScsSpec>;
#[doc = "Field `EntryPresent` reader - 0:0\\]
This bit indicates whether an entry is present at this location in the ROM table. 0x0:Rom table entry is not present and must be skipped. 0x1:Rom table entry is present."]
pub type EntryPresentR = crate::BitReader;
#[doc = "Field `Format` reader - 1:1\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
pub type FormatR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 11:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `AddressOffset` reader - 31:12\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
pub type AddressOffsetR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit indicates whether an entry is present at this location in the ROM table. 0x0:Rom table entry is not present and must be skipped. 0x1:Rom table entry is present."]
    #[inline(always)]
    pub fn entry_present(&self) -> EntryPresentR {
        EntryPresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11 - 11:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
    #[inline(always)]
    pub fn address_offset(&self) -> AddressOffsetR {
        AddressOffsetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "Points to the System Control Space (SCS) at 0xE000E000. This includes core debug control registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_scs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_scs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntryScsSpec;
impl crate::RegisterSpec for EntryScsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry_scs::R`](R) reader structure"]
impl crate::Readable for EntryScsSpec {}
#[doc = "`write(|w| ..)` method takes [`entry_scs::W`](W) writer structure"]
impl crate::Writable for EntryScsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EntrySCS to value 0xfff0_f003"]
impl crate::Resettable for EntryScsSpec {
    const RESET_VALUE: u32 = 0xfff0_f003;
}
