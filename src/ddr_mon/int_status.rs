#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Field `CH0_BELOW_INT` reader - This is the interrupt status of DDR read and write burst number less\n\nthan low threshold in channel 0"]
pub type Ch0BelowIntR = crate::BitReader;
#[doc = "Field `CH0_OVER_INT` reader - This is the interrupt status of DDR read and write burst number\n\nmore than high threshold in channel 0"]
pub type Ch0OverIntR = crate::BitReader;
#[doc = "Field `CH1_BELOW_INT` reader - This is the interrupt status of DDR read and write burst number less\n\nthan low threshold in channel 0"]
pub type Ch1BelowIntR = crate::BitReader;
#[doc = "Field `CH1_OVER_INT` reader - This is the interrupt status of DDR read and write burst number\n\nmore than high threshold in channel 0"]
pub type Ch1OverIntR = crate::BitReader;
#[doc = "Field `CH0_WR_ADDR_HIT` reader - This is the interrupt status of channel 0 write address hit the setting\n\nrange"]
pub type Ch0WrAddrHitR = crate::BitReader;
#[doc = "Field `CH0_RD_ADDR_HIT` reader - This is the interrupt status of channel 0 read address hit the setting\n\nrange"]
pub type Ch0RdAddrHitR = crate::BitReader;
#[doc = "Field `CH1_WR_ADDR_HIT` reader - This is the interrupt status of channel 1 write address hit the setting\n\nrange"]
pub type Ch1WrAddrHitR = crate::BitReader;
#[doc = "Field `CH1_RD_ADDR_HIT` reader - This is the interrupt status of channel 1 read address hit the setting\n\nrange"]
pub type Ch1RdAddrHitR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the interrupt status of DDR read and write burst number less\n\nthan low threshold in channel 0"]
    #[inline(always)]
    pub fn ch0_below_int(&self) -> Ch0BelowIntR {
        Ch0BelowIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt status of DDR read and write burst number\n\nmore than high threshold in channel 0"]
    #[inline(always)]
    pub fn ch0_over_int(&self) -> Ch0OverIntR {
        Ch0OverIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt status of DDR read and write burst number less\n\nthan low threshold in channel 0"]
    #[inline(always)]
    pub fn ch1_below_int(&self) -> Ch1BelowIntR {
        Ch1BelowIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt status of DDR read and write burst number\n\nmore than high threshold in channel 0"]
    #[inline(always)]
    pub fn ch1_over_int(&self) -> Ch1OverIntR {
        Ch1OverIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt status of channel 0 write address hit the setting\n\nrange"]
    #[inline(always)]
    pub fn ch0_wr_addr_hit(&self) -> Ch0WrAddrHitR {
        Ch0WrAddrHitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt status of channel 0 read address hit the setting\n\nrange"]
    #[inline(always)]
    pub fn ch0_rd_addr_hit(&self) -> Ch0RdAddrHitR {
        Ch0RdAddrHitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt status of channel 1 write address hit the setting\n\nrange"]
    #[inline(always)]
    pub fn ch1_wr_addr_hit(&self) -> Ch1WrAddrHitR {
        Ch1WrAddrHitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt status of channel 1 read address hit the setting\n\nrange"]
    #[inline(always)]
    pub fn ch1_rd_addr_hit(&self) -> Ch1RdAddrHitR {
        Ch1RdAddrHitR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
