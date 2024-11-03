#[doc = "Register `SICR` reader"]
pub type R = crate::R<SicrSpec>;
#[doc = "Register `SICR` writer"]
pub type W = crate::W<SicrSpec>;
#[doc = "Field `DATAIC` reader - 0:0\\]
Data interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.DATARIS and SMIS.DATAMIS."]
pub type DataicR = crate::BitReader;
#[doc = "Field `DATAIC` writer - 0:0\\]
Data interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.DATARIS and SMIS.DATAMIS."]
pub type DataicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTIC` reader - 1:1\\]
Start condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STARTRIS and SMIS.STARTMIS."]
pub type StarticR = crate::BitReader;
#[doc = "Field `STARTIC` writer - 1:1\\]
Start condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STARTRIS and SMIS.STARTMIS."]
pub type StarticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIC` reader - 2:2\\]
Stop condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
pub type StopicR = crate::BitReader;
#[doc = "Field `STOPIC` writer - 2:2\\]
Stop condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
pub type StopicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.DATARIS and SMIS.DATAMIS."]
    #[inline(always)]
    pub fn dataic(&self) -> DataicR {
        DataicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STARTRIS and SMIS.STARTMIS."]
    #[inline(always)]
    pub fn startic(&self) -> StarticR {
        StarticR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub fn stopic(&self) -> StopicR {
        StopicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.DATARIS and SMIS.DATAMIS."]
    #[inline(always)]
    #[must_use]
    pub fn dataic(&mut self) -> DataicW<SicrSpec> {
        DataicW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STARTRIS and SMIS.STARTMIS."]
    #[inline(always)]
    #[must_use]
    pub fn startic(&mut self) -> StarticW<SicrSpec> {
        StarticW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear 0 - Writing 0 has no effect 1 - Set interrupt Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    #[must_use]
    pub fn stopic(&mut self) -> StopicW<SicrSpec> {
        StopicW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SicrSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SicrSpec;
impl crate::RegisterSpec for SicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sicr::R`](R) reader structure"]
impl crate::Readable for SicrSpec {}
#[doc = "`write(|w| ..)` method takes [`sicr::W`](W) writer structure"]
impl crate::Writable for SicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SICR to value 0"]
impl crate::Resettable for SicrSpec {
    const RESET_VALUE: u32 = 0;
}
