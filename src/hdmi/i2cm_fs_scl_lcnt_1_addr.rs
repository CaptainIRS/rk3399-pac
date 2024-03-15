#[doc = "Register `I2CM_FS_SCL_LCNT_1_ADDR` reader"]
pub type R = crate::R<I2cmFsSclLcnt1AddrSpec>;
#[doc = "Register `I2CM_FS_SCL_LCNT_1_ADDR` writer"]
pub type W = crate::W<I2cmFsSclLcnt1AddrSpec>;
#[doc = "Field `I2CMP_FS_SCL_LCNT1` reader - I2C DDC Fast Speed SCL Low Level Control Register 1"]
pub type I2cmpFsSclLcnt1R = crate::FieldReader;
#[doc = "Field `I2CMP_FS_SCL_LCNT1` writer - I2C DDC Fast Speed SCL Low Level Control Register 1"]
pub type I2cmpFsSclLcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2C DDC Fast Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    pub fn i2cmp_fs_scl_lcnt1(&self) -> I2cmpFsSclLcnt1R {
        I2cmpFsSclLcnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C DDC Fast Speed SCL Low Level Control Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmp_fs_scl_lcnt1(&mut self) -> I2cmpFsSclLcnt1W<I2cmFsSclLcnt1AddrSpec> {
        I2cmpFsSclLcnt1W::new(self, 0)
    }
}
#[doc = "I2C DDC Fast Speed SCL Low Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_fs_scl_lcnt_1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_fs_scl_lcnt_1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmFsSclLcnt1AddrSpec;
impl crate::RegisterSpec for I2cmFsSclLcnt1AddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_fs_scl_lcnt_1_addr::R`](R) reader structure"]
impl crate::Readable for I2cmFsSclLcnt1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_fs_scl_lcnt_1_addr::W`](W) writer structure"]
impl crate::Writable for I2cmFsSclLcnt1AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_FS_SCL_LCNT_1_ADDR to value 0"]
impl crate::Resettable for I2cmFsSclLcnt1AddrSpec {
    const RESET_VALUE: u8 = 0;
}
