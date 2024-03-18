#[doc = "Register `RKI2C_ST` reader"]
pub type R = crate::R<Rki2cStSpec>;
#[doc = "Field `SDA_ST` reader - sda status 1'b0: sda status low 1'b0: sda status high"]
pub type SdaStR = crate::BitReader;
#[doc = "Field `SCL_ST` reader - scl status 1'b0: scl status low 1'b0: scl status high"]
pub type SclStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sda status 1'b0: sda status low 1'b0: sda status high"]
    #[inline(always)]
    pub fn sda_st(&self) -> SdaStR {
        SdaStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - scl status 1'b0: scl status low 1'b0: scl status high"]
    #[inline(always)]
    pub fn scl_st(&self) -> SclStR {
        SclStR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "status debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cStSpec;
impl crate::RegisterSpec for Rki2cStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_st::R`](R) reader structure"]
impl crate::Readable for Rki2cStSpec {}
#[doc = "`reset()` method sets RKI2C_ST to value 0"]
impl crate::Resettable for Rki2cStSpec {
    const RESET_VALUE: u32 = 0;
}
