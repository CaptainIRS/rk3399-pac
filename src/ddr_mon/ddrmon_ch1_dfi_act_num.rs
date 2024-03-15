#[doc = "Register `DDRMON_CH1_DFI_ACT_NUM` reader"]
pub type R = crate::R<DdrmonCh1DfiActNumSpec>;
#[doc = "Field `CH1_DFI_ACT_NUM` reader - DFI active command number in the statistics range of the channel 1"]
pub type Ch1DfiActNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI active command number in the statistics range of the channel 1"]
    #[inline(always)]
    pub fn ch1_dfi_act_num(&self) -> Ch1DfiActNumR {
        Ch1DfiActNumR::new(self.bits)
    }
}
#[doc = "Channel 1 DFI Active Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_dfi_act_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh1DfiActNumSpec;
impl crate::RegisterSpec for DdrmonCh1DfiActNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch1_dfi_act_num::R`](R) reader structure"]
impl crate::Readable for DdrmonCh1DfiActNumSpec {}
#[doc = "`reset()` method sets DDRMON_CH1_DFI_ACT_NUM to value 0"]
impl crate::Resettable for DdrmonCh1DfiActNumSpec {
    const RESET_VALUE: u32 = 0;
}
