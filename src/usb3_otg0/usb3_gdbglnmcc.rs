#[doc = "Register `USB3_GDBGLNMCC` reader"]
pub type R = crate::R<Usb3GdbglnmccSpec>;
#[doc = "Field `LNMCC_BERC` reader - LNMCC_BERC This field indicates the bit error rate information for the port selected in the GDBGFIFOSPACE.PortSelect field.This field is for debug purposes only."]
pub type LnmccBercR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - LNMCC_BERC This field indicates the bit error rate information for the port selected in the GDBGFIFOSPACE.PortSelect field.This field is for debug purposes only."]
    #[inline(always)]
    pub fn lnmcc_berc(&self) -> LnmccBercR {
        LnmccBercR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Global Debug LNMCC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbglnmcc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GdbglnmccSpec;
impl crate::RegisterSpec for Usb3GdbglnmccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gdbglnmcc::R`](R) reader structure"]
impl crate::Readable for Usb3GdbglnmccSpec {}
#[doc = "`reset()` method sets USB3_GDBGLNMCC to value 0"]
impl crate::Resettable for Usb3GdbglnmccSpec {
    const RESET_VALUE: u32 = 0;
}
