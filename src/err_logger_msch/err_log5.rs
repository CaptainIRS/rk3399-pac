#[doc = "Register `ErrLog5` reader"]
pub type R = crate::R<ErrLog5Spec>;
#[doc = "Field `AXIID` reader - AXI ID for AXI master. It is read as 0 for AHB Master. Unused bits\n\nare read as 0."]
pub type AxiidR = crate::FieldReader<u16>;
#[doc = "Field `MID` reader - Master ID\n\nMaster ID for each master."]
pub type MidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - AXI ID for AXI master. It is read as 0 for AHB Master. Unused bits\n\nare read as 0."]
    #[inline(always)]
    pub fn axiid(&self) -> AxiidR {
        AxiidR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Master ID\n\nMaster ID for each master."]
    #[inline(always)]
    pub fn mid(&self) -> MidR {
        MidR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "User bits in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrLog5Spec;
impl crate::RegisterSpec for ErrLog5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_log5::R`](R) reader structure"]
impl crate::Readable for ErrLog5Spec {}
#[doc = "`reset()` method sets ErrLog5 to value 0"]
impl crate::Resettable for ErrLog5Spec {
    const RESET_VALUE: u32 = 0;
}
