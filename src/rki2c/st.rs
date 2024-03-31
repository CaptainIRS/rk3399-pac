#[doc = "Register `ST` reader"]
pub type R = crate::R<StSpec>;
#[doc = "Field `SDA_ST` reader - sda status\n\n1'b0: sda status low\n\n1'b0: sda status high"]
pub type SdaStR = crate::BitReader;
#[doc = "Field `SCL_ST` reader - scl status\n\n1'b0: scl status low\n\n1'b0: scl status high"]
pub type SclStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sda status\n\n1'b0: sda status low\n\n1'b0: sda status high"]
    #[inline(always)]
    pub fn sda_st(&self) -> SdaStR {
        SdaStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - scl status\n\n1'b0: scl status low\n\n1'b0: scl status high"]
    #[inline(always)]
    pub fn scl_st(&self) -> SclStR {
        SclStR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "status debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StSpec;
impl crate::RegisterSpec for StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st::R`](R) reader structure"]
impl crate::Readable for StSpec {}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for StSpec {
    const RESET_VALUE: u32 = 0;
}
