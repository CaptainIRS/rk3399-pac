#[doc = "Register `DDR_DENALI_CTL_182` reader"]
pub type R = crate::R<DdrDenaliCtl182Spec>;
#[doc = "Register `DDR_DENALI_CTL_182` writer"]
pub type W = crate::W<DdrDenaliCtl182Spec>;
#[doc = "Field `ZQINIT_F1` reader - Number of cycles needed for a ZQINIT command for frequency copy 1."]
pub type ZqinitF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQINIT_F1` writer - Number of cycles needed for a ZQINIT command for frequency copy 1."]
pub type ZqinitF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ZQCL_F1` reader - Number of cycles needed for a ZQCL command for frequency copy 1."]
pub type ZqclF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQCL_F1` writer - Number of cycles needed for a ZQCL command for frequency copy 1."]
pub type ZqclF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQINIT command for frequency copy 1."]
    #[inline(always)]
    pub fn zqinit_f1(&self) -> ZqinitF1R {
        ZqinitF1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQCL command for frequency copy 1."]
    #[inline(always)]
    pub fn zqcl_f1(&self) -> ZqclF1R {
        ZqclF1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQINIT command for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zqinit_f1(&mut self) -> ZqinitF1W<DdrDenaliCtl182Spec> {
        ZqinitF1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQCL command for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zqcl_f1(&mut self) -> ZqclF1W<DdrDenaliCtl182Spec> {
        ZqclF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_182::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_182::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl182Spec;
impl crate::RegisterSpec for DdrDenaliCtl182Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_182::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl182Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_182::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl182Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_182 to value 0"]
impl crate::Resettable for DdrDenaliCtl182Spec {
    const RESET_VALUE: u32 = 0;
}
