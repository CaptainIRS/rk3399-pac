#[doc = "Register `PHY_I2CM_FS_SCL_LCNT_0_ADDR` reader"]
pub type R = crate::R<PhyI2cmFsSclLcnt0AddrSpec>;
#[doc = "Register `PHY_I2CM_FS_SCL_LCNT_0_ADDR` writer"]
pub type W = crate::W<PhyI2cmFsSclLcnt0AddrSpec>;
#[doc = "Field `I2CMP_FS_SCL_LCNT0` reader - PHY I2C Fast Speed SCL Low Level Control Register 0"]
pub type I2cmpFsSclLcnt0R = crate::FieldReader;
#[doc = "Field `I2CMP_FS_SCL_LCNT0` writer - PHY I2C Fast Speed SCL Low Level Control Register 0"]
pub type I2cmpFsSclLcnt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PHY I2C Fast Speed SCL Low Level Control Register 0"]
    #[inline(always)]
    pub fn i2cmp_fs_scl_lcnt0(&self) -> I2cmpFsSclLcnt0R {
        I2cmpFsSclLcnt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PHY I2C Fast Speed SCL Low Level Control Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmp_fs_scl_lcnt0(&mut self) -> I2cmpFsSclLcnt0W<PhyI2cmFsSclLcnt0AddrSpec> {
        I2cmpFsSclLcnt0W::new(self, 0)
    }
}
#[doc = "PHY I2C Fast Speed SCL Low Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_fs_scl_lcnt_0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_fs_scl_lcnt_0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmFsSclLcnt0AddrSpec;
impl crate::RegisterSpec for PhyI2cmFsSclLcnt0AddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_fs_scl_lcnt_0_addr::R`](R) reader structure"]
impl crate::Readable for PhyI2cmFsSclLcnt0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_fs_scl_lcnt_0_addr::W`](W) writer structure"]
impl crate::Writable for PhyI2cmFsSclLcnt0AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_FS_SCL_LCNT_0_ADDR to value 0x24"]
impl crate::Resettable for PhyI2cmFsSclLcnt0AddrSpec {
    const RESET_VALUE: u8 = 0x24;
}
