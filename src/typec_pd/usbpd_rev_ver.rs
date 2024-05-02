#[doc = "Register `USBPD_REV_VER` reader"]
pub type R = crate::R<UsbpdRevVerSpec>;
#[doc = "Field `bcdUSBPD_Version` reader - 0001 0000 - Version1.0, \n\n0001 0001 - Version 1.1, etc."]
pub type BcdUsbpdVersionR = crate::FieldReader;
#[doc = "Field `bcdUSBPD_Revision` reader - 0010 0000 - Revision2.0"]
pub type BcdUsbpdRevisionR = crate::FieldReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 0001 0000 - Version1.0, \n\n0001 0001 - Version 1.1, etc."]
    #[inline(always)]
    pub fn bcd_usbpd_version(&self) -> BcdUsbpdVersionR {
        BcdUsbpdVersionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 0010 0000 - Revision2.0"]
    #[inline(always)]
    pub fn bcd_usbpd_revision(&self) -> BcdUsbpdRevisionR {
        BcdUsbpdRevisionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "USB PD Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpd_rev_ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbpdRevVerSpec;
impl crate::RegisterSpec for UsbpdRevVerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbpd_rev_ver::R`](R) reader structure"]
impl crate::Readable for UsbpdRevVerSpec {}
#[doc = "`reset()` method sets USBPD_REV_VER to value 0x2011"]
impl crate::Resettable for UsbpdRevVerSpec {
    const RESET_VALUE: u32 = 0x2011;
}
