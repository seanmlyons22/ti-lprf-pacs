#[doc = "Register `SDR` reader"]
pub type R = crate::R<SdrSpec>;
#[doc = "Register `SDR` writer"]
pub type W = crate::W<SdrSpec>;
#[doc = "Field `DATA` reader - 7:0\\]
Data for transfer. This field contains the data for transfer during a slave receive or a transmit operation. When written the register data is used as transmit data When read, this register returns the last data received. Data is stored until next update, either by a system write to the master for transmit or by an external master to the slave for receive."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 7:0\\]
Data for transfer. This field contains the data for transfer during a slave receive or a transmit operation. When written the register data is used as transmit data When read, this register returns the last data received. Data is stored until next update, either by a system write to the master for transmit or by an external master to the slave for receive."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data for transfer. This field contains the data for transfer during a slave receive or a transmit operation. When written the register data is used as transmit data When read, this register returns the last data received. Data is stored until next update, either by a system write to the master for transmit or by an external master to the slave for receive."]
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
Data for transfer. This field contains the data for transfer during a slave receive or a transmit operation. When written the register data is used as transmit data When read, this register returns the last data received. Data is stored until next update, either by a system write to the master for transmit or by an external master to the slave for receive."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<SdrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SdrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrSpec;
impl crate::RegisterSpec for SdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdr::R`](R) reader structure"]
impl crate::Readable for SdrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdr::W`](W) writer structure"]
impl crate::Writable for SdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDR to value 0"]
impl crate::Resettable for SdrSpec {
    const RESET_VALUE: u32 = 0;
}