#[doc = "Register `EMMCCORE_VENDOR` reader"]
pub type R = crate::R<EmmccoreVendorSpec>;
#[doc = "Register `EMMCCORE_VENDOR` writer"]
pub type W = crate::W<EmmccoreVendorSpec>;
#[doc = "Field `ENHANCEDSTROBE` reader - This bit enables the enhanced strobe logic of the Host Controller"]
pub type EnhancedstrobeR = crate::BitReader;
#[doc = "Field `ENHANCEDSTROBE` writer - This bit enables the enhanced strobe logic of the Host Controller"]
pub type EnhancedstrobeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit enables the enhanced strobe logic of the Host Controller"]
    #[inline(always)]
    pub fn enhancedstrobe(&self) -> EnhancedstrobeR {
        EnhancedstrobeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the enhanced strobe logic of the Host Controller"]
    #[inline(always)]
    #[must_use]
    pub fn enhancedstrobe(&mut self) -> EnhancedstrobeW<EmmccoreVendorSpec> {
        EnhancedstrobeW::new(self, 0)
    }
}
#[doc = "Vendor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_vendor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_vendor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreVendorSpec;
impl crate::RegisterSpec for EmmccoreVendorSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_vendor::R`](R) reader structure"]
impl crate::Readable for EmmccoreVendorSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_vendor::W`](W) writer structure"]
impl crate::Writable for EmmccoreVendorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_VENDOR to value 0"]
impl crate::Resettable for EmmccoreVendorSpec {
    const RESET_VALUE: u16 = 0;
}
