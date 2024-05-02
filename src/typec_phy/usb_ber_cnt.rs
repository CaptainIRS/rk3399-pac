#[doc = "Register `USB_BER_CNT` reader"]
pub type R = crate::R<UsbBerCntSpec>;
#[doc = "Field `FIELD1` reader - Current value of USB 30 loopback lsave Bit Error Count from the PCS. \n\n(Not re-synchronized to apb_pclk)"]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current value of USB 30 loopback lsave Bit Error Count from the PCS. \n\n(Not re-synchronized to apb_pclk)"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "USB loopback slave BER count isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ber_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbBerCntSpec;
impl crate::RegisterSpec for UsbBerCntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usb_ber_cnt::R`](R) reader structure"]
impl crate::Readable for UsbBerCntSpec {}
#[doc = "`reset()` method sets USB_BER_CNT to value 0"]
impl crate::Resettable for UsbBerCntSpec {
    const RESET_VALUE: u16 = 0;
}
