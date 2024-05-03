#[doc = "Register `IMAGE_VALID_CONF` reader"]
pub type R = crate::R<ImageValidConfSpec>;
#[doc = "Register `IMAGE_VALID_CONF` writer"]
pub type W = crate::W<ImageValidConfSpec>;
#[doc = "Field `IMAGE_VALID` reader - 31:0\\]
This field must have the address value of the start of the flash vector table in order to enable the boot FW in ROM to transfer control to a flash image. Any illegal vector table start address value will force the boot FW in ROM to transfer control to the serial boot loader in ROM."]
pub type ImageValidR = crate::FieldReader<u32>;
#[doc = "Field `IMAGE_VALID` writer - 31:0\\]
This field must have the address value of the start of the flash vector table in order to enable the boot FW in ROM to transfer control to a flash image. Any illegal vector table start address value will force the boot FW in ROM to transfer control to the serial boot loader in ROM."]
pub type ImageValidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field must have the address value of the start of the flash vector table in order to enable the boot FW in ROM to transfer control to a flash image. Any illegal vector table start address value will force the boot FW in ROM to transfer control to the serial boot loader in ROM."]
    #[inline(always)]
    pub fn image_valid(&self) -> ImageValidR {
        ImageValidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field must have the address value of the start of the flash vector table in order to enable the boot FW in ROM to transfer control to a flash image. Any illegal vector table start address value will force the boot FW in ROM to transfer control to the serial boot loader in ROM."]
    #[inline(always)]
    #[must_use]
    pub fn image_valid(&mut self) -> ImageValidW<ImageValidConfSpec> {
        ImageValidW::new(self, 0)
    }
}
#[doc = "Image Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`image_valid_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`image_valid_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImageValidConfSpec;
impl crate::RegisterSpec for ImageValidConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`image_valid_conf::R`](R) reader structure"]
impl crate::Readable for ImageValidConfSpec {}
#[doc = "`write(|w| ..)` method takes [`image_valid_conf::W`](W) writer structure"]
impl crate::Writable for ImageValidConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMAGE_VALID_CONF to value 0xffff_ffff"]
impl crate::Resettable for ImageValidConfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
