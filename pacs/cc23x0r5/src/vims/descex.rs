#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "Field `ROMSZ` reader - 14:0\\]
Provides the size of ROM in Bytes."]
pub type RomszR = crate::FieldReader<u16>;
#[doc = "Field `FLSZ` reader - 26:15\\]
This provides the total FLASH size in Kilo Bytes. The total FLASH size is (FLSZ + 1)KB"]
pub type FlszR = crate::FieldReader<u16>;
#[doc = "Field `NBANK` reader - 27:27\\]
Provides the FLASH Bank count"]
pub type NbankR = crate::BitReader;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
Provides the size of ROM in Bytes."]
    #[inline(always)]
    pub fn romsz(&self) -> RomszR {
        RomszR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:26 - 26:15\\]
This provides the total FLASH size in Kilo Bytes. The total FLASH size is (FLSZ + 1)KB"]
    #[inline(always)]
    pub fn flsz(&self) -> FlszR {
        FlszR::new(((self.bits >> 15) & 0x0fff) as u16)
    }
    #[doc = "Bit 27 - 27:27\\]
Provides the FLASH Bank count"]
    #[inline(always)]
    pub fn nbank(&self) -> NbankR {
        NbankR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "This register describes the configuration of VIMS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescexSpec;
impl crate::RegisterSpec for DescexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex::R`](R) reader structure"]
impl crate::Readable for DescexSpec {}
#[doc = "`write(|w| ..)` method takes [`descex::W`](W) writer structure"]
impl crate::Writable for DescexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX to value 0x08ff_b000"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0x08ff_b000;
}
