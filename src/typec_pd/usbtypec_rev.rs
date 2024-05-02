#[doc = "Register `USBTYPEC_REV` reader"]
pub type R = crate::R<UsbtypecRevSpec>;
#[doc = "Field `USB_Type_C_Revision` reader - Version number assigned by \n\nUSB-IF (Currently at Revision \n\n1.1 - 00010001)"]
pub type UsbTypeCRevisionR = crate::FieldReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Version number assigned by \n\nUSB-IF (Currently at Revision \n\n1.1 - 00010001)"]
    #[inline(always)]
    pub fn usb_type_c_revision(&self) -> UsbTypeCRevisionR {
        UsbTypeCRevisionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "USB Type-C Revision Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbtypec_rev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbtypecRevSpec;
impl crate::RegisterSpec for UsbtypecRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbtypec_rev::R`](R) reader structure"]
impl crate::Readable for UsbtypecRevSpec {}
#[doc = "`reset()` method sets USBTYPEC_REV to value 0x11"]
impl crate::Resettable for UsbtypecRevSpec {
    const RESET_VALUE: u32 = 0x11;
}
