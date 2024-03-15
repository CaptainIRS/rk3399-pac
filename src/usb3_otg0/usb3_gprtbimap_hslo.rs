#[doc = "Register `USB3_GPRTBIMAP_HSLO` reader"]
pub type R = crate::R<Usb3GprtbimapHsloSpec>;
#[doc = "Register `USB3_GPRTBIMAP_HSLO` writer"]
pub type W = crate::W<Usb3GprtbimapHsloSpec>;
#[doc = "Field `BINUM1` reader - HS USB Instance Number for Port 1 Application-programmable ID field."]
pub type Binum1R = crate::FieldReader;
#[doc = "Field `BINUM1` writer - HS USB Instance Number for Port 1 Application-programmable ID field."]
pub type Binum1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - HS USB Instance Number for Port 1 Application-programmable ID field."]
    #[inline(always)]
    pub fn binum1(&self) -> Binum1R {
        Binum1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - HS USB Instance Number for Port 1 Application-programmable ID field."]
    #[inline(always)]
    #[must_use]
    pub fn binum1(&mut self) -> Binum1W<Usb3GprtbimapHsloSpec> {
        Binum1W::new(self, 0)
    }
}
#[doc = "Global High-Speed Port to Bus Instance Mapping Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gprtbimap_hslo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gprtbimap_hslo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GprtbimapHsloSpec;
impl crate::RegisterSpec for Usb3GprtbimapHsloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gprtbimap_hslo::R`](R) reader structure"]
impl crate::Readable for Usb3GprtbimapHsloSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gprtbimap_hslo::W`](W) writer structure"]
impl crate::Writable for Usb3GprtbimapHsloSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GPRTBIMAP_HSLO to value 0"]
impl crate::Resettable for Usb3GprtbimapHsloSpec {
    const RESET_VALUE: u32 = 0;
}
