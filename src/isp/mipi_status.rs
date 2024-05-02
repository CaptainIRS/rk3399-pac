#[doc = "Register `MIPI_STATUS` reader"]
pub type R = crate::R<MipiStatusSpec>;
#[doc = "Field `ADD_DATA_AVAIL` reader - 1: additional data fifo contains data 0: additional data\n\nfifo is empty"]
pub type AddDataAvailR = crate::BitReader;
#[doc = "Field `STOPSTATE` reader - Data Lane is in stopstate. This register is directly\n\nconnected to the synchronized input signal\n\nstopstate\\[n-1:0\\]
where n denotes the lane number 1..4"]
pub type StopstateR = crate::FieldReader;
#[doc = "Field `S_STOPSTATE_CLK` reader - sensor clock lane is in stopstate. This register is\n\ndirectly connected to the synchronized input signal 's_stopstateclk'"]
pub type SStopstateClkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: additional data fifo contains data 0: additional data\n\nfifo is empty"]
    #[inline(always)]
    pub fn add_data_avail(&self) -> AddDataAvailR {
        AddDataAvailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data Lane is in stopstate. This register is directly\n\nconnected to the synchronized input signal\n\nstopstate\\[n-1:0\\]
where n denotes the lane number 1..4"]
    #[inline(always)]
    pub fn stopstate(&self) -> StopstateR {
        StopstateR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - sensor clock lane is in stopstate. This register is\n\ndirectly connected to the synchronized input signal 's_stopstateclk'"]
    #[inline(always)]
    pub fn s_stopstate_clk(&self) -> SStopstateClkR {
        SStopstateClkR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "global status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiStatusSpec;
impl crate::RegisterSpec for MipiStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_status::R`](R) reader structure"]
impl crate::Readable for MipiStatusSpec {}
#[doc = "`reset()` method sets MIPI_STATUS to value 0"]
impl crate::Resettable for MipiStatusSpec {
    const RESET_VALUE: u32 = 0;
}
