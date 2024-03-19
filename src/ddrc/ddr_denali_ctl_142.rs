#[doc = "Register `DDR_DENALI_CTL_142` reader"]
pub type R = crate::R<DdrDenaliCtl142Spec>;
#[doc = "Register `DDR_DENALI_CTL_142` writer"]
pub type W = crate::W<DdrDenaliCtl142Spec>;
#[doc = "Field `MR13_DATA_0` reader - Data to program into memory mode register 13 for chip select 0."]
pub type Mr13Data0R = crate::FieldReader<u16>;
#[doc = "Field `MR13_DATA_0` writer - Data to program into memory mode register 13 for chip select 0."]
pub type Mr13Data0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR14_DATA_F0_0` reader - Data to program into memory mode register 14 for chip select 0."]
pub type Mr14DataF0_0R = crate::FieldReader<u16>;
#[doc = "Field `MR14_DATA_F0_0` writer - Data to program into memory mode register 14 for chip select 0."]
pub type Mr14DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 13 for chip select 0."]
    #[inline(always)]
    pub fn mr13_data_0(&self) -> Mr13Data0R {
        Mr13Data0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 14 for chip select 0."]
    #[inline(always)]
    pub fn mr14_data_f0_0(&self) -> Mr14DataF0_0R {
        Mr14DataF0_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 13 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr13_data_0(&mut self) -> Mr13Data0W<DdrDenaliCtl142Spec> {
        Mr13Data0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 14 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr14_data_f0_0(&mut self) -> Mr14DataF0_0W<DdrDenaliCtl142Spec> {
        Mr14DataF0_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_142::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_142::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl142Spec;
impl crate::RegisterSpec for DdrDenaliCtl142Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_142::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl142Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_142::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl142Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_142 to value 0"]
impl crate::Resettable for DdrDenaliCtl142Spec {
    const RESET_VALUE: u32 = 0;
}
