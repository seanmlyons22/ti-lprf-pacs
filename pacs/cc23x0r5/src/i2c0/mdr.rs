#[doc = "Register `MDR` reader"]
pub type R = crate::R<MdrSpec>;
#[doc = "Register `MDR` writer"]
pub type W = crate::W<MdrSpec>;
#[doc = "Field `DATA` reader - 7:0\\]
When Read: Last RX Data is returned When Written: Data is transferred during TX transaction"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 7:0\\]
When Read: Last RX Data is returned When Written: Data is transferred during TX transaction"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
When Read: Last RX Data is returned When Written: Data is transferred during TX transaction"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
When Read: Last RX Data is returned When Written: Data is transferred during TX transaction"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdrSpec;
impl crate::RegisterSpec for MdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdr::R`](R) reader structure"]
impl crate::Readable for MdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdr::W`](W) writer structure"]
impl crate::Writable for MdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MdrSpec {
    const RESET_VALUE: u32 = 0;
}
