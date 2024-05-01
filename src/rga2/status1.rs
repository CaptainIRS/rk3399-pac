#[doc = "Register `STATUS1` reader"]
pub type R = crate::R<Status1Spec>;
#[doc = "RGA engine status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRgaSta {
    #[doc = "0: idle"]
    B0 = 0,
    #[doc = "1: working"]
    B1 = 1,
}
impl From<SwRgaSta> for bool {
    #[inline(always)]
    fn from(variant: SwRgaSta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RGA_STA` reader - RGA engine status"]
pub type SwRgaStaR = crate::BitReader<SwRgaSta>;
impl SwRgaStaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRgaSta {
        match self.bits {
            false => SwRgaSta::B0,
            true => SwRgaSta::B1,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRgaSta::B0
    }
    #[doc = "working"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRgaSta::B1
    }
}
#[doc = "Field `SW_CMD_CUR_NUM` reader - RGA command current number"]
pub type SwCmdCurNumR = crate::FieldReader<u16>;
#[doc = "Field `SW_CMD_TOTAL_NUM` reader - RGA command total number"]
pub type SwCmdTotalNumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - RGA engine status"]
    #[inline(always)]
    pub fn sw_rga_sta(&self) -> SwRgaStaR {
        SwRgaStaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:19 - RGA command current number"]
    #[inline(always)]
    pub fn sw_cmd_cur_num(&self) -> SwCmdCurNumR {
        SwCmdCurNumR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - RGA command total number"]
    #[inline(always)]
    pub fn sw_cmd_total_num(&self) -> SwCmdTotalNumR {
        SwCmdTotalNumR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "RGA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status1Spec;
impl crate::RegisterSpec for Status1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1::R`](R) reader structure"]
impl crate::Readable for Status1Spec {}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for Status1Spec {
    const RESET_VALUE: u32 = 0;
}
