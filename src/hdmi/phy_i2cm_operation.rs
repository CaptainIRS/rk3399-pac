#[doc = "Register `PHY_I2CM_OPERATION` writer"]
pub type W = crate::W<PhyI2cmOperationSpec>;
#[doc = "Field `RD` writer - Read operation request"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` writer - Write operation request"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Read operation request"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<PhyI2cmOperationSpec> {
        RdW::new(self, 0)
    }
    #[doc = "Bit 4 - Write operation request"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<PhyI2cmOperationSpec> {
        WrW::new(self, 4)
    }
}
#[doc = "Read operation request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_operation::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmOperationSpec;
impl crate::RegisterSpec for PhyI2cmOperationSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_operation::W`](W) writer structure"]
impl crate::Writable for PhyI2cmOperationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_OPERATION to value 0"]
impl crate::Resettable for PhyI2cmOperationSpec {
    const RESET_VALUE: u8 = 0;
}
