#[doc = "Register `DENALI_CTL_141` reader"]
pub type R = crate::R<DenaliCtl141Spec>;
#[doc = "Register `DENALI_CTL_141` writer"]
pub type W = crate::W<DenaliCtl141Spec>;
#[doc = "Field `MR12_DATA_F1_0` reader - Data to program into memory mode register 12 for chip select 0."]
pub type Mr12DataF1_0R = crate::FieldReader<u16>;
#[doc = "Field `MR12_DATA_F1_0` writer - Data to program into memory mode register 12 for chip select 0."]
pub type Mr12DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR12_DATA_F2_0` reader - Data to program into memory mode register 12 for chip select 0."]
pub type Mr12DataF2_0R = crate::FieldReader<u16>;
#[doc = "Field `MR12_DATA_F2_0` writer - Data to program into memory mode register 12 for chip select 0."]
pub type Mr12DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 12 for chip select 0."]
    #[inline(always)]
    pub fn mr12_data_f1_0(&self) -> Mr12DataF1_0R {
        Mr12DataF1_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 12 for chip select 0."]
    #[inline(always)]
    pub fn mr12_data_f2_0(&self) -> Mr12DataF2_0R {
        Mr12DataF2_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 12 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr12_data_f1_0(&mut self) -> Mr12DataF1_0W<DenaliCtl141Spec> {
        Mr12DataF1_0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 12 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr12_data_f2_0(&mut self) -> Mr12DataF2_0W<DenaliCtl141Spec> {
        Mr12DataF2_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_141::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_141::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl141Spec;
impl crate::RegisterSpec for DenaliCtl141Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_141::R`](R) reader structure"]
impl crate::Readable for DenaliCtl141Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_141::W`](W) writer structure"]
impl crate::Writable for DenaliCtl141Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_141 to value 0"]
impl crate::Resettable for DenaliCtl141Spec {
    const RESET_VALUE: u32 = 0;
}
