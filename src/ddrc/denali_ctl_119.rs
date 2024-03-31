#[doc = "Register `DENALI_CTL_119` reader"]
pub type R = crate::R<DenaliCtl119Spec>;
#[doc = "Field `PERIPHERAL_MRR_DATA` reader - Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits (7:0) indicate the read data and bits (15:8) indicate the chip."]
pub type PeripheralMrrDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits (7:0) indicate the read data and bits (15:8) indicate the chip."]
    #[inline(always)]
    pub fn peripheral_mrr_data(&self) -> PeripheralMrrDataR {
        PeripheralMrrDataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_119::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl119Spec;
impl crate::RegisterSpec for DenaliCtl119Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_119::R`](R) reader structure"]
impl crate::Readable for DenaliCtl119Spec {}
#[doc = "`reset()` method sets DENALI_CTL_119 to value 0"]
impl crate::Resettable for DenaliCtl119Spec {
    const RESET_VALUE: u32 = 0;
}
