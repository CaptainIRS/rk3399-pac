#[doc = "Register `DENALI_CTL_156` reader"]
pub type R = crate::R<DenaliCtl156Spec>;
#[doc = "Register `DENALI_CTL_156` writer"]
pub type W = crate::W<DenaliCtl156Spec>;
#[doc = "Field `MR13_DATA_1` reader - Data to program into memory mode register 13 for chip select 1."]
pub type Mr13Data1R = crate::FieldReader<u16>;
#[doc = "Field `MR13_DATA_1` writer - Data to program into memory mode register 13 for chip select 1."]
pub type Mr13Data1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR14_DATA_F0_1` reader - Data to program into memory mode register 14 for chip select 1."]
pub type Mr14DataF0_1R = crate::FieldReader<u16>;
#[doc = "Field `MR14_DATA_F0_1` writer - Data to program into memory mode register 14 for chip select 1."]
pub type Mr14DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 13 for chip select 1."]
    #[inline(always)]
    pub fn mr13_data_1(&self) -> Mr13Data1R {
        Mr13Data1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 14 for chip select 1."]
    #[inline(always)]
    pub fn mr14_data_f0_1(&self) -> Mr14DataF0_1R {
        Mr14DataF0_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 13 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr13_data_1(&mut self) -> Mr13Data1W<DenaliCtl156Spec> {
        Mr13Data1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 14 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr14_data_f0_1(&mut self) -> Mr14DataF0_1W<DenaliCtl156Spec> {
        Mr14DataF0_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_156::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_156::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl156Spec;
impl crate::RegisterSpec for DenaliCtl156Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_156::R`](R) reader structure"]
impl crate::Readable for DenaliCtl156Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_156::W`](W) writer structure"]
impl crate::Writable for DenaliCtl156Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_156 to value 0"]
impl crate::Resettable for DenaliCtl156Spec {
    const RESET_VALUE: u32 = 0;
}
