#[doc = "Register `DENALI_CTL_120` reader"]
pub type R = crate::R<DenaliCtl120Spec>;
#[doc = "Field `PERIPHERAL_MRR_DATA` reader - Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits (7:0) indicate the read data and bits (15:8) indicate the chip."]
pub type PeripheralMrrDataR = crate::FieldReader;
#[doc = "Field `AUTO_TEMPCHK_VAL_0` reader - MR4 data for all devices on chip 0 accessed by automatic MRR commands. Bits (3:0) correlate to the device on the lower byte, bits (7:4) correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits."]
pub type AutoTempchkVal0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits (7:0) indicate the read data and bits (15:8) indicate the chip."]
    #[inline(always)]
    pub fn peripheral_mrr_data(&self) -> PeripheralMrrDataR {
        PeripheralMrrDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - MR4 data for all devices on chip 0 accessed by automatic MRR commands. Bits (3:0) correlate to the device on the lower byte, bits (7:4) correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits."]
    #[inline(always)]
    pub fn auto_tempchk_val_0(&self) -> AutoTempchkVal0R {
        AutoTempchkVal0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_120::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl120Spec;
impl crate::RegisterSpec for DenaliCtl120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_120::R`](R) reader structure"]
impl crate::Readable for DenaliCtl120Spec {}
#[doc = "`reset()` method sets DENALI_CTL_120 to value 0"]
impl crate::Resettable for DenaliCtl120Spec {
    const RESET_VALUE: u32 = 0;
}
