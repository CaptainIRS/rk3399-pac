#[doc = "Register `GDBGLNMCC` reader"]
pub type R = crate::R<GdbglnmccSpec>;
#[doc = "Field `LNMCC_BERC` reader - LNMCC_BERC\n\nThis field indicates the bit error rate information for the port\n\nselected in the GDBGFIFOSPACE.PortSelect field.This field is for\n\ndebug purposes only."]
pub type LnmccBercR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - LNMCC_BERC\n\nThis field indicates the bit error rate information for the port\n\nselected in the GDBGFIFOSPACE.PortSelect field.This field is for\n\ndebug purposes only."]
    #[inline(always)]
    pub fn lnmcc_berc(&self) -> LnmccBercR {
        LnmccBercR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Global Debug LNMCC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdbglnmcc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdbglnmccSpec;
impl crate::RegisterSpec for GdbglnmccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdbglnmcc::R`](R) reader structure"]
impl crate::Readable for GdbglnmccSpec {}
#[doc = "`reset()` method sets GDBGLNMCC to value 0"]
impl crate::Resettable for GdbglnmccSpec {
    const RESET_VALUE: u32 = 0;
}
