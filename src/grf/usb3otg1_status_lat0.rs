#[doc = "Register `USB3OTG1_STATUS_LAT0` reader"]
pub type R = crate::R<Usb3otg1StatusLat0Spec>;
#[doc = "Register `USB3OTG1_STATUS_LAT0` writer"]
pub type W = crate::W<Usb3otg1StatusLat0Spec>;
#[doc = "Field `USBCPHY1_OTG_UTMI_IDDIG` reader - status of usbcphy1_otg_utmi_iddig\\[31:0\\]"]
pub type Usbcphy1OtgUtmiIddigR = crate::FieldReader<u32>;
#[doc = "Field `USBCPHY1_OTG_UTMI_IDDIG` writer - status of usbcphy1_otg_utmi_iddig\\[31:0\\]"]
pub type Usbcphy1OtgUtmiIddigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - status of usbcphy1_otg_utmi_iddig\\[31:0\\]"]
    #[inline(always)]
    pub fn usbcphy1_otg_utmi_iddig(&self) -> Usbcphy1OtgUtmiIddigR {
        Usbcphy1OtgUtmiIddigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - status of usbcphy1_otg_utmi_iddig\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn usbcphy1_otg_utmi_iddig(&mut self) -> Usbcphy1OtgUtmiIddigW<Usb3otg1StatusLat0Spec> {
        Usbcphy1OtgUtmiIddigW::new(self, 0)
    }
}
#[doc = "USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg1_status_lat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg1_status_lat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3otg1StatusLat0Spec;
impl crate::RegisterSpec for Usb3otg1StatusLat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3otg1_status_lat0::R`](R) reader structure"]
impl crate::Readable for Usb3otg1StatusLat0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3otg1_status_lat0::W`](W) writer structure"]
impl crate::Writable for Usb3otg1StatusLat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3OTG1_STATUS_LAT0 to value 0"]
impl crate::Resettable for Usb3otg1StatusLat0Spec {
    const RESET_VALUE: u32 = 0;
}
