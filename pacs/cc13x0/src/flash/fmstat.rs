#[doc = "Register `FMSTAT` reader"]
pub type R = crate::R<FmstatSpec>;
#[doc = "Register `FMSTAT` writer"]
pub type W = crate::W<FmstatSpec>;
#[doc = "Field `SLOCK` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SlockR = crate::BitReader;
#[doc = "Field `PSUSP` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type PsuspR = crate::BitReader;
#[doc = "Field `ESUSP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type EsuspR = crate::BitReader;
#[doc = "Field `VOLSTAT` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type VolstatR = crate::BitReader;
#[doc = "Field `CSTAT` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type CstatR = crate::BitReader;
#[doc = "Field `INVDAT` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type InvdatR = crate::BitReader;
#[doc = "Field `PGM` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type PgmR = crate::BitReader;
#[doc = "Field `ERS` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type ErsR = crate::BitReader;
#[doc = "Field `BUSY` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type BusyR = crate::BitReader;
#[doc = "Field `CV` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type CvR = crate::BitReader;
#[doc = "Field `EV` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EvR = crate::BitReader;
#[doc = "Field `PCV` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type PcvR = crate::BitReader;
#[doc = "Field `PGV` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type PgvR = crate::BitReader;
#[doc = "Field `DBF` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type DbfR = crate::BitReader;
#[doc = "Field `ILA` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type IlaR = crate::BitReader;
#[doc = "Field `RVF` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type RvfR = crate::BitReader;
#[doc = "Field `RDVER` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type RdverR = crate::BitReader;
#[doc = "Field `RVSUSP` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RvsuspR = crate::BitReader;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type Reserved18R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn slock(&self) -> SlockR {
        SlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psusp(&self) -> PsuspR {
        PsuspR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esusp(&self) -> EsuspR {
        EsuspR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn volstat(&self) -> VolstatR {
        VolstatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cstat(&self) -> CstatR {
        CstatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn invdat(&self) -> InvdatR {
        InvdatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm(&self) -> PgmR {
        PgmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ers(&self) -> ErsR {
        ErsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pcv(&self) -> PcvR {
        PcvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgv(&self) -> PgvR {
        PgvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ila(&self) -> IlaR {
        IlaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf(&self) -> RvfR {
        RvfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdver(&self) -> RdverR {
        RdverR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsusp(&self) -> RvsuspR {
        RvsuspR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmstatSpec;
impl crate::RegisterSpec for FmstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmstat::R`](R) reader structure"]
impl crate::Readable for FmstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fmstat::W`](W) writer structure"]
impl crate::Writable for FmstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMSTAT to value 0"]
impl crate::Resettable for FmstatSpec {
    const RESET_VALUE: u32 = 0;
}
