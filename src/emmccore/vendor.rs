#[doc = "Register `VENDOR` reader"]
pub type R = crate::R<VendorSpec>;
#[doc = "Register `VENDOR` writer"]
pub type W = crate::W<VendorSpec>;
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
    pub fn enhancedstrobe(&mut self) -> EnhancedstrobeW<VendorSpec> {
        EnhancedstrobeW::new(self, 0)
    }
}
#[doc = "Vendor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vendor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendorSpec;
impl crate::RegisterSpec for VendorSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vendor::R`](R) reader structure"]
impl crate::Readable for VendorSpec {}
#[doc = "`write(|w| ..)` method takes [`vendor::W`](W) writer structure"]
impl crate::Writable for VendorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets VENDOR to value 0"]
impl crate::Resettable for VendorSpec {
    const RESET_VALUE: u16 = 0;
}
