#[doc = "Register `CIC_STATUS2` reader"]
pub type R = crate::R<CicStatus2Spec>;
#[doc = "Field `DDR0_FREQ_CHANGE_REQ_TYPE` reader - Channel 0 DDR PHY frequency change request type"]
pub type Ddr0FreqChangeReqTypeR = crate::FieldReader;
#[doc = "Field `DDR0_FREQ_CHANGE_REQ` reader - Channel 0 DDR PHY frequency change request"]
pub type Ddr0FreqChangeReqR = crate::BitReader;
#[doc = "Field `DDR0_CNTRL_FREQ_CHANGE_REQ_TYPE` reader - Channel 0 DDR controller frequency change request type"]
pub type Ddr0CntrlFreqChangeReqTypeR = crate::FieldReader;
#[doc = "Field `DDR0_CNTRL_FREQ_CHANGE_REQ` reader - Channel 0 DDR controller frequency change request"]
pub type Ddr0CntrlFreqChangeReqR = crate::BitReader;
#[doc = "Field `DDR1_FREQ_CHANGE_REQ_TYPE` reader - Channel 1 DDR PHY frequency change request type"]
pub type Ddr1FreqChangeReqTypeR = crate::FieldReader;
#[doc = "Field `DDR1_FREQ_CHANGE_REQ` reader - Channel 1 DDR PHY frequency change request"]
pub type Ddr1FreqChangeReqR = crate::BitReader;
#[doc = "Field `DDR1_CNTRL_FREQ_CHANGE_REQ_TYPE` reader - Channel 1 DDR controller frequency change request type"]
pub type Ddr1CntrlFreqChangeReqTypeR = crate::FieldReader;
#[doc = "Field `DDR1_CNTRL_FREQ_CHANGE_REQ` reader - Channel 1 DDR controller frequency change request"]
pub type Ddr1CntrlFreqChangeReqR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Channel 0 DDR PHY frequency change request type"]
    #[inline(always)]
    pub fn ddr0_freq_change_req_type(&self) -> Ddr0FreqChangeReqTypeR {
        Ddr0FreqChangeReqTypeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Channel 0 DDR PHY frequency change request"]
    #[inline(always)]
    pub fn ddr0_freq_change_req(&self) -> Ddr0FreqChangeReqR {
        Ddr0FreqChangeReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Channel 0 DDR controller frequency change request type"]
    #[inline(always)]
    pub fn ddr0_cntrl_freq_change_req_type(&self) -> Ddr0CntrlFreqChangeReqTypeR {
        Ddr0CntrlFreqChangeReqTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Channel 0 DDR controller frequency change request"]
    #[inline(always)]
    pub fn ddr0_cntrl_freq_change_req(&self) -> Ddr0CntrlFreqChangeReqR {
        Ddr0CntrlFreqChangeReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Channel 1 DDR PHY frequency change request type"]
    #[inline(always)]
    pub fn ddr1_freq_change_req_type(&self) -> Ddr1FreqChangeReqTypeR {
        Ddr1FreqChangeReqTypeR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Channel 1 DDR PHY frequency change request"]
    #[inline(always)]
    pub fn ddr1_freq_change_req(&self) -> Ddr1FreqChangeReqR {
        Ddr1FreqChangeReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Channel 1 DDR controller frequency change request type"]
    #[inline(always)]
    pub fn ddr1_cntrl_freq_change_req_type(&self) -> Ddr1CntrlFreqChangeReqTypeR {
        Ddr1CntrlFreqChangeReqTypeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Channel 1 DDR controller frequency change request"]
    #[inline(always)]
    pub fn ddr1_cntrl_freq_change_req(&self) -> Ddr1CntrlFreqChangeReqR {
        Ddr1CntrlFreqChangeReqR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "DDR Controller LP Interface Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicStatus2Spec;
impl crate::RegisterSpec for CicStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cic_status2::R`](R) reader structure"]
impl crate::Readable for CicStatus2Spec {}
#[doc = "`reset()` method sets CIC_STATUS2 to value 0"]
impl crate::Resettable for CicStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
