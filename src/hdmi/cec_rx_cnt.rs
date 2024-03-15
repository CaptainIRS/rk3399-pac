#[doc = "Register `CEC_RX_CNT` reader"]
pub type R = crate::R<CecRxCntSpec>;
#[doc = "CEC Receiver Counter register:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CecRxCnt {
    #[doc = "0: 16-byte data is received"]
    D0 = 0,
    #[doc = "1: 16-byte data is received"]
    D1 = 1,
    #[doc = "16: 16-byte data is received"]
    D16 = 16,
}
impl From<CecRxCnt> for u8 {
    #[inline(always)]
    fn from(variant: CecRxCnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CecRxCnt {
    type Ux = u8;
}
#[doc = "Field `CEC_RX_CNT` reader - CEC Receiver Counter register:"]
pub type CecRxCntR = crate::FieldReader<CecRxCnt>;
impl CecRxCntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CecRxCnt> {
        match self.bits {
            0 => Some(CecRxCnt::D0),
            1 => Some(CecRxCnt::D1),
            16 => Some(CecRxCnt::D16),
            _ => None,
        }
    }
    #[doc = "16-byte data is received"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == CecRxCnt::D0
    }
    #[doc = "16-byte data is received"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == CecRxCnt::D1
    }
    #[doc = "16-byte data is received"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == CecRxCnt::D16
    }
}
impl R {
    #[doc = "Bits 0:4 - CEC Receiver Counter register:"]
    #[inline(always)]
    pub fn cec_rx_cnt(&self) -> CecRxCntR {
        CecRxCntR::new(self.bits & 0x1f)
    }
}
#[doc = "CEC Receiver Counter register: 5'd0: No data received 5'd1: 1-byte data is received 5'd16: 16-byte data is received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_rx_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecRxCntSpec;
impl crate::RegisterSpec for CecRxCntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_rx_cnt::R`](R) reader structure"]
impl crate::Readable for CecRxCntSpec {}
#[doc = "`reset()` method sets CEC_RX_CNT to value 0"]
impl crate::Resettable for CecRxCntSpec {
    const RESET_VALUE: u8 = 0;
}
