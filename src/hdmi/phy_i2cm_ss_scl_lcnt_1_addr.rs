#[doc = "Register `PHY_I2CM_SS_SCL_LCNT_1_ADDR` reader"]
pub type R = crate::R<PhyI2cmSsSclLcnt1AddrSpec>;
#[doc = "Register `PHY_I2CM_SS_SCL_LCNT_1_ADDR` writer"]
pub type W = crate::W<PhyI2cmSsSclLcnt1AddrSpec>;
#[doc = "Field `I2CMP_SS_SCL_LCNT1` reader - PHY I2C Slow Speed SCL Low Level Control Register 1"]
pub type I2cmpSsSclLcnt1R = crate::FieldReader;
#[doc = "Field `I2CMP_SS_SCL_LCNT1` writer - PHY I2C Slow Speed SCL Low Level Control Register 1"]
pub type I2cmpSsSclLcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PHY I2C Slow Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    pub fn i2cmp_ss_scl_lcnt1(&self) -> I2cmpSsSclLcnt1R {
        I2cmpSsSclLcnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PHY I2C Slow Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmp_ss_scl_lcnt1(&mut self) -> I2cmpSsSclLcnt1W<PhyI2cmSsSclLcnt1AddrSpec> {
        I2cmpSsSclLcnt1W::new(self, 0)
    }
}
#[doc = "PHY I2C Slow Speed SCL Low Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_ss_scl_lcnt_1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_ss_scl_lcnt_1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmSsSclLcnt1AddrSpec;
impl crate::RegisterSpec for PhyI2cmSsSclLcnt1AddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_ss_scl_lcnt_1_addr::R`](R) reader structure"]
impl crate::Readable for PhyI2cmSsSclLcnt1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_ss_scl_lcnt_1_addr::W`](W) writer structure"]
impl crate::Writable for PhyI2cmSsSclLcnt1AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_SS_SCL_LCNT_1_ADDR to value 0"]
impl crate::Resettable for PhyI2cmSsSclLcnt1AddrSpec {
    const RESET_VALUE: u8 = 0;
}
