#[doc = "Register `USB3OTG0_STATUS_CB` reader"]
pub type R = crate::R<Usb3otg0StatusCbSpec>;
#[doc = "Register `USB3OTG0_STATUS_CB` writer"]
pub type W = crate::W<Usb3otg0StatusCbSpec>;
#[doc = "Field `USB3OTG0_HOST_CURRENT_BELT` reader - status of usb3otg0_host_current_belt"]
pub type Usb3otg0HostCurrentBeltR = crate::FieldReader<u16>;
#[doc = "Field `USB3OTG0_HOST_CURRENT_BELT` writer - status of usb3otg0_host_current_belt"]
pub type Usb3otg0HostCurrentBeltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - status of usb3otg0_host_current_belt"]
    #[inline(always)]
    pub fn usb3otg0_host_current_belt(&self) -> Usb3otg0HostCurrentBeltR {
        Usb3otg0HostCurrentBeltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - status of usb3otg0_host_current_belt"]
    #[inline(always)]
    #[must_use]
    pub fn usb3otg0_host_current_belt(&mut self) -> Usb3otg0HostCurrentBeltW<Usb3otg0StatusCbSpec> {
        Usb3otg0HostCurrentBeltW::new(self, 0)
    }
}
#[doc = "USB3 OTG0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg0_status_cb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg0_status_cb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3otg0StatusCbSpec;
impl crate::RegisterSpec for Usb3otg0StatusCbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3otg0_status_cb::R`](R) reader structure"]
impl crate::Readable for Usb3otg0StatusCbSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3otg0_status_cb::W`](W) writer structure"]
impl crate::Writable for Usb3otg0StatusCbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3OTG0_STATUS_CB to value 0"]
impl crate::Resettable for Usb3otg0StatusCbSpec {
    const RESET_VALUE: u32 = 0;
}
