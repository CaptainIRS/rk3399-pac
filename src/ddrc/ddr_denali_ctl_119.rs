#[doc = "Register `DDR_DENALI_CTL_119` reader"]
pub type R = crate::R<DdrDenaliCtl119Spec>;
#[doc = "Field `PERIPHERAL_MRR_DATA` reader - Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits (7:0) indicate the read data and bits (15:8) indicate the chip. READ- ONLY"]
pub type PeripheralMrrDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits (7:0) indicate the read data and bits (15:8) indicate the chip. READ- ONLY"]
    #[inline(always)]
    pub fn peripheral_mrr_data(&self) -> PeripheralMrrDataR {
        PeripheralMrrDataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_119::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl119Spec;
impl crate::RegisterSpec for DdrDenaliCtl119Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_119::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl119Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_119 to value 0"]
impl crate::Resettable for DdrDenaliCtl119Spec {
    const RESET_VALUE: u32 = 0;
}
