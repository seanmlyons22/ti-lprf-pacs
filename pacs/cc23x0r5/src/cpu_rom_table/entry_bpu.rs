#[doc = "Register `EntryBPU` reader"]
pub type R = crate::R<EntryBpuSpec>;
#[doc = "Register `EntryBPU` writer"]
pub type W = crate::W<EntryBpuSpec>;
#[doc = "Field `EntryPresent` reader - 0:0\\]
This bit indicates whether an entry is present at this location in the ROM table. 0x0:Rom table entry is not present and must be skipped. 0x1:Rom table entry is present."]
pub type EntryPresentR = crate::BitReader;
#[doc = "Field `EntryPresent` writer - 0:0\\]
This bit indicates whether an entry is present at this location in the ROM table. 0x0:Rom table entry is not present and must be skipped. 0x1:Rom table entry is present."]
pub type EntryPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Format` reader - 1:1\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
pub type FormatR = crate::BitReader;
#[doc = "Field `Format` writer - 1:1\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 11:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED2` writer - 11:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AddressOffset` reader - 31:12\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
pub type AddressOffsetR = crate::FieldReader<u32>;
#[doc = "Field `AddressOffset` writer - 31:12\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
pub type AddressOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
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
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit indicates whether an entry is present at this location in the ROM table. 0x0:Rom table entry is not present and must be skipped. 0x1:Rom table entry is present."]
    #[inline(always)]
    #[must_use]
    pub fn entry_present(&mut self) -> EntryPresentW<EntryBpuSpec> {
        EntryPresentW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FormatW<EntryBpuSpec> {
        FormatW::new(self, 1)
    }
    #[doc = "Bits 2:11 - 11:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<EntryBpuSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Base address of the highest 4KB block for the component, relative to the ROM address."]
    #[inline(always)]
    #[must_use]
    pub fn address_offset(&mut self) -> AddressOffsetW<EntryBpuSpec> {
        AddressOffsetW::new(self, 12)
    }
}
#[doc = "Points to the BPU at 0xE0002000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_bpu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_bpu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntryBpuSpec;
impl crate::RegisterSpec for EntryBpuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry_bpu::R`](R) reader structure"]
impl crate::Readable for EntryBpuSpec {}
#[doc = "`write(|w| ..)` method takes [`entry_bpu::W`](W) writer structure"]
impl crate::Writable for EntryBpuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EntryBPU to value 0xfff0_3002"]
impl crate::Resettable for EntryBpuSpec {
    const RESET_VALUE: u32 = 0xfff0_3002;
}