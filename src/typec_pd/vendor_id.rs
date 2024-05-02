#[doc = "Register `VENDOR_ID` reader"]
pub type R = crate::R<VendorIdSpec>;
#[doc = "Field `Vendor_ID_VID` reader - A unique 16-bit unsigned integer. \n\nAssigned by the USB-IF to the \n\nVendor."]
pub type VendorIdVidR = crate::FieldReader<u16>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - A unique 16-bit unsigned integer. \n\nAssigned by the USB-IF to the \n\nVendor."]
    #[inline(always)]
    pub fn vendor_id_vid(&self) -> VendorIdVidR {
        VendorIdVidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Vendor ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendor_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendorIdSpec;
impl crate::RegisterSpec for VendorIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vendor_id::R`](R) reader structure"]
impl crate::Readable for VendorIdSpec {}
#[doc = "`reset()` method sets VENDOR_ID to value 0"]
impl crate::Resettable for VendorIdSpec {
    const RESET_VALUE: u32 = 0;
}
