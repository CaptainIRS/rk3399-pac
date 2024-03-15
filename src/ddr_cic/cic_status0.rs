#[doc = "Register `CIC_STATUS0` reader"]
pub type R = crate::R<CicStatus0Spec>;
#[doc = "Field `CHG_DONE` reader - Frequency change done"]
pub type ChgDoneR = crate::BitReader;
#[doc = "Field `CHG_FAIL` reader - Frequency change fail"]
pub type ChgFailR = crate::BitReader;
#[doc = "Field `CHG_FREQ_WAIT` reader - Frequency change wait"]
pub type ChgFreqWaitR = crate::BitReader;
#[doc = "Field `SREF_REQ_EXT_CH0` reader - Channel 0 external self-refresh request"]
pub type SrefReqExtCh0R = crate::BitReader;
#[doc = "Field `SREF_DONE_EXT_CH0` reader - Channel 0 external self-refresh done"]
pub type SrefDoneExtCh0R = crate::BitReader;
#[doc = "Field `SREF_REQ_EXT_CH1` reader - Channel 1 external self-refresh request"]
pub type SrefReqExtCh1R = crate::BitReader;
#[doc = "Field `SREF_DONE_EXT_CH1` reader - Channel 1 external self-refresh done"]
pub type SrefDoneExtCh1R = crate::BitReader;
#[doc = "Field `STATE_FC` reader - Frequency change state machine"]
pub type StateFcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Frequency change done"]
    #[inline(always)]
    pub fn chg_done(&self) -> ChgDoneR {
        ChgDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frequency change fail"]
    #[inline(always)]
    pub fn chg_fail(&self) -> ChgFailR {
        ChgFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frequency change wait"]
    #[inline(always)]
    pub fn chg_freq_wait(&self) -> ChgFreqWaitR {
        ChgFreqWaitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 external self-refresh request"]
    #[inline(always)]
    pub fn sref_req_ext_ch0(&self) -> SrefReqExtCh0R {
        SrefReqExtCh0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 external self-refresh done"]
    #[inline(always)]
    pub fn sref_done_ext_ch0(&self) -> SrefDoneExtCh0R {
        SrefDoneExtCh0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 external self-refresh request"]
    #[inline(always)]
    pub fn sref_req_ext_ch1(&self) -> SrefReqExtCh1R {
        SrefReqExtCh1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 1 external self-refresh done"]
    #[inline(always)]
    pub fn sref_done_ext_ch1(&self) -> SrefDoneExtCh1R {
        SrefDoneExtCh1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Frequency change state machine"]
    #[inline(always)]
    pub fn state_fc(&self) -> StateFcR {
        StateFcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
#[doc = "DDR Controller LP Interface Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicStatus0Spec;
impl crate::RegisterSpec for CicStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cic_status0::R`](R) reader structure"]
impl crate::Readable for CicStatus0Spec {}
#[doc = "`reset()` method sets CIC_STATUS0 to value 0"]
impl crate::Resettable for CicStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
