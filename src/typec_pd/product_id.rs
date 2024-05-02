#[doc = "Register `PRODUCT_ID` reader"]
pub type R = crate::R<ProductIdSpec>;
#[doc = "Field `USB_Product_ID_PID` reader - A unique 16-bit unsigned integer. \n\nAssigned uniquely by the Vendor to \n\nidentify the TCPC."]
pub type UsbProductIdPidR = crate::FieldReader<u16>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - A unique 16-bit unsigned integer. \n\nAssigned uniquely by the Vendor to \n\nidentify the TCPC."]
    #[inline(always)]
    pub fn usb_product_id_pid(&self) -> UsbProductIdPidR {
        UsbProductIdPidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Product ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProductIdSpec;
impl crate::RegisterSpec for ProductIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`product_id::R`](R) reader structure"]
impl crate::Readable for ProductIdSpec {}
#[doc = "`reset()` method sets PRODUCT_ID to value 0"]
impl crate::Resettable for ProductIdSpec {
    const RESET_VALUE: u32 = 0;
}
