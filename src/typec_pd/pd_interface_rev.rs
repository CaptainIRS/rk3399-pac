#[doc = "Register `PD_INTERFACE_REV` reader"]
pub type R = crate::R<PdInterfaceRevSpec>;
#[doc = "Field `bcd_USB_PD_InterBlock_Specification_Version` reader - 0001 0000 - Version1.0 \n\n0001 0001 \n\n- \n\nVersion \n\n1.1 Etc."]
pub type BcdUsbPdInterBlockSpecificationVersionR = crate::FieldReader;
#[doc = "Field `bcd_USB_PD_InterBlock_Specification_Revision` reader - 0001 0000 - Revision 1.0 (this \n\nrelease)"]
pub type BcdUsbPdInterBlockSpecificationRevisionR = crate::FieldReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore \n\nread \n\nvalue. \n\nAlways \n\nreads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 0001 0000 - Version1.0 \n\n0001 0001 \n\n- \n\nVersion \n\n1.1 Etc."]
    #[inline(always)]
    pub fn bcd_usb_pd_inter_block_specification_version(
        &self,
    ) -> BcdUsbPdInterBlockSpecificationVersionR {
        BcdUsbPdInterBlockSpecificationVersionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 0001 0000 - Revision 1.0 (this \n\nrelease)"]
    #[inline(always)]
    pub fn bcd_usb_pd_inter_block_specification_revision(
        &self,
    ) -> BcdUsbPdInterBlockSpecificationRevisionR {
        BcdUsbPdInterBlockSpecificationRevisionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore \n\nread \n\nvalue. \n\nAlways \n\nreads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PD Interface Block Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_interface_rev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdInterfaceRevSpec;
impl crate::RegisterSpec for PdInterfaceRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_interface_rev::R`](R) reader structure"]
impl crate::Readable for PdInterfaceRevSpec {}
#[doc = "`reset()` method sets PD_INTERFACE_REV to value 0x1010"]
impl crate::Resettable for PdInterfaceRevSpec {
    const RESET_VALUE: u32 = 0x1010;
}
