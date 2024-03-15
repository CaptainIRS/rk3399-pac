#[doc = "Register `I2CM_SS_SCL_HCNT_0_ADDR` reader"]
pub type R = crate::R<I2cmSsSclHcnt0AddrSpec>;
#[doc = "Register `I2CM_SS_SCL_HCNT_0_ADDR` writer"]
pub type W = crate::W<I2cmSsSclHcnt0AddrSpec>;
#[doc = "Field `I2CMP_SS_SCL_HCNT0` reader - I2C DDC Slow Speed SCL High Level Control Register 0"]
pub type I2cmpSsSclHcnt0R = crate::FieldReader;
#[doc = "Field `I2CMP_SS_SCL_HCNT0` writer - I2C DDC Slow Speed SCL High Level Control Register 0"]
pub type I2cmpSsSclHcnt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2C DDC Slow Speed SCL High Level Control Register 0"]
    #[inline(always)]
    pub fn i2cmp_ss_scl_hcnt0(&self) -> I2cmpSsSclHcnt0R {
        I2cmpSsSclHcnt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C DDC Slow Speed SCL High Level Control Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2cmp_ss_scl_hcnt0(&mut self) -> I2cmpSsSclHcnt0W<I2cmSsSclHcnt0AddrSpec> {
        I2cmpSsSclHcnt0W::new(self, 0)
    }
}
#[doc = "I2C DDC Slow Speed SCL High Level Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_ss_scl_hcnt_0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_ss_scl_hcnt_0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmSsSclHcnt0AddrSpec;
impl crate::RegisterSpec for I2cmSsSclHcnt0AddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_ss_scl_hcnt_0_addr::R`](R) reader structure"]
impl crate::Readable for I2cmSsSclHcnt0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_ss_scl_hcnt_0_addr::W`](W) writer structure"]
impl crate::Writable for I2cmSsSclHcnt0AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SS_SCL_HCNT_0_ADDR to value 0x6c"]
impl crate::Resettable for I2cmSsSclHcnt0AddrSpec {
    const RESET_VALUE: u8 = 0x6c;
}
