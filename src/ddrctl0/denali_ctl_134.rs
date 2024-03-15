#[doc = "Register `DENALI_CTL_134` reader"]
pub type R = crate::R<DenaliCtl134Spec>;
#[doc = "Register `DENALI_CTL_134` writer"]
pub type W = crate::W<DenaliCtl134Spec>;
#[doc = "Field `MR2_DATA_F0_0` reader - Data to program into memory mode register 2 for chip select 0 for frequency copy 0."]
pub type Mr2DataF0_0R = crate::FieldReader<u16>;
#[doc = "Field `MR2_DATA_F0_0` writer - Data to program into memory mode register 2 for chip select 0 for frequency copy 0."]
pub type Mr2DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR0_DATA_F1_0` reader - Data to program into memory mode register 0 for chip select 0 for frequency copy 1."]
pub type Mr0DataF1_0R = crate::FieldReader<u16>;
#[doc = "Field `MR0_DATA_F1_0` writer - Data to program into memory mode register 0 for chip select 0 for frequency copy 1."]
pub type Mr0DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 2 for chip select 0 for frequency copy 0."]
    #[inline(always)]
    pub fn mr2_data_f0_0(&self) -> Mr2DataF0_0R {
        Mr2DataF0_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 0 for chip select 0 for frequency copy 1."]
    #[inline(always)]
    pub fn mr0_data_f1_0(&self) -> Mr0DataF1_0R {
        Mr0DataF1_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 2 for chip select 0 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr2_data_f0_0(&mut self) -> Mr2DataF0_0W<DenaliCtl134Spec> {
        Mr2DataF0_0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 0 for chip select 0 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr0_data_f1_0(&mut self) -> Mr0DataF1_0W<DenaliCtl134Spec> {
        Mr0DataF1_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_134::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_134::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl134Spec;
impl crate::RegisterSpec for DenaliCtl134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_134::R`](R) reader structure"]
impl crate::Readable for DenaliCtl134Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_134::W`](W) writer structure"]
impl crate::Writable for DenaliCtl134Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_134 to value 0"]
impl crate::Resettable for DenaliCtl134Spec {
    const RESET_VALUE: u32 = 0;
}
