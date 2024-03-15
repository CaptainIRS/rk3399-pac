#[doc = "Register `I2CM_SS_SCL_HCNT_1_ADDR` reader"]
pub type R = crate::R<I2cmSsSclHcnt1AddrSpec>;
#[doc = "Register `I2CM_SS_SCL_HCNT_1_ADDR` writer"]
pub type W = crate::W<I2cmSsSclHcnt1AddrSpec>;
#[doc = "Field `I2CMP_SS_SCL_HCNT1` reader - I2C DDC Slow Speed SCL High Level Control Register 1"]
pub type I2cmpSsSclHcnt1R = crate::FieldReader;
#[doc = "Field `I2CMP_SS_SCL_HCNT1` writer - I2C DDC Slow Speed SCL High Level Control Register 1"]
pub type I2cmpSsSclHcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2C DDC Slow Speed SCL High Level Control Register 1"]
    #[inline(always)]
    pub fn i2cmp_ss_scl_hcnt1(&self) -> I2cmpSsSclHcnt1R {
        I2cmpSsSclHcnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C DDC Slow Speed SCL High Level Control Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmp_ss_scl_hcnt1(&mut self) -> I2cmpSsSclHcnt1W<I2cmSsSclHcnt1AddrSpec> {
        I2cmpSsSclHcnt1W::new(self, 0)
    }
}
#[doc = "I2C DDC Slow Speed SCL High Level Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ss_scl_hcnt_1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ss_scl_hcnt_1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmSsSclHcnt1AddrSpec;
impl crate::RegisterSpec for I2cmSsSclHcnt1AddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_ss_scl_hcnt_1_addr::R`](R) reader structure"]
impl crate::Readable for I2cmSsSclHcnt1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_ss_scl_hcnt_1_addr::W`](W) writer structure"]
impl crate::Writable for I2cmSsSclHcnt1AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SS_SCL_HCNT_1_ADDR to value 0"]
impl crate::Resettable for I2cmSsSclHcnt1AddrSpec {
    const RESET_VALUE: u8 = 0;
}
